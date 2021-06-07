use std::collections::{HashMap};

use actix::prelude::*;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, web, HttpResponse};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::errors::ElectionError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ballot {
    id: Uuid,
    cast_time: Option<time::OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Vote {
    ballot_id: Uuid,
    voorzitter: String,
    ondervoorzitter: String,
    penning_meester: String,
    secretaris: String,
}

impl Ballot {
    async fn create(pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Ballot,
            "INSERT INTO ballots (id) VALUES (uuid_generate_v4()) RETURNING *"
        )
        .fetch_one(pool)
        .await
    }

    /// Cast the vote, consuming it and setting the cast time, thus preventing it from re-use
    async fn cast(vote: &Vote, pool: &PgPool) -> Result<Vote, ElectionError> {
        let tx = pool.begin().await?;

        let vote = sqlx::query_as!(
            Vote, 
            r#"INSERT INTO votes (ballot_id, voorzitter, ondervoorzitter, penning_meester, secretaris)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *"#,
            vote.ballot_id, vote.voorzitter, vote.ondervoorzitter, vote.penning_meester, vote.secretaris,
        ).fetch_one(pool).await?;

        sqlx::query!("UPDATE ballots SET cast_time = NOW() WHERE id=$1", vote.ballot_id).execute(pool).await?;

        tx.commit().await?;

        Ok(vote)
    }

    /// Load a ballot, if it's already used, return an invalid balid error
    async fn load(id: &Uuid, pool: &PgPool) -> Result<Self, ElectionError> {
        let ballot = sqlx::query_as!(Ballot, "SELECT * FROM ballots WHERE id=$1", id).fetch_one(pool).await?;

        if ballot.cast_time.is_some() {
            return Err(ElectionError::AlreadyVoted)
        }
        Ok(ballot)
    }
}


#[post("/ballots")]
async fn create_ballot(
    state: Data<crate::State>,
) -> Result<actix_web::HttpResponse, ElectionError> {
    let ballot = Ballot::create(&state.db).await?;

    Ok(HttpResponse::Created().json(ballot))
}

#[post("/vote")]
async fn cast_vote(
    vote: Json<Vote>,
    state: Data<crate::State>,
) -> Result<actix_web::HttpResponse, ElectionError> {
    let vote = Ballot::cast(&vote, &state.db).await?;

    if let Err(e) = state.ws.send(NewVote).await {
        log::error!("Unable to notify new vote: {:?}", e);
    }

    Ok(HttpResponse::Created().json(vote))
}

/// This should be used to display the candidate list to the user
#[get("/ballots/{uuid}")]
async fn show_ballot(uuid: Path<Uuid>, state: Data<crate::State>,) -> Result<actix_web::HttpResponse, ElectionError> {
    let ballot = Ballot::load(&uuid, &state.db).await?;

    Ok(HttpResponse::Ok().json(ballot))
}

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_ballot);
    cfg.service(cast_vote);
    cfg.service(show_ballot);
}

#[derive(Clone)]
pub(crate) struct ElectionServer {
    sessions: HashMap<usize, Recipient<NewVote>>,
    rng: ThreadRng,
}

impl ElectionServer {
    pub(crate) fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    // Notify all connected users about a new vote
    fn broadcast_vote(&self, msg: NewVote) {
        for session in self.sessions.values() {
            let _ = session.do_send(msg);
        }
    }
}

impl Actor for ElectionServer {
    type Context = Context<Self>;
}

#[derive(Message, Serialize, Clone, Copy)]
#[rtype(result = "()")]
pub(crate) struct NewVote;

impl Handler<NewVote> for ElectionServer {
    type Result = ();

    fn handle(&mut self, msg: NewVote, _ctx: &mut Context<Self>) -> Self::Result {
        self.broadcast_vote(msg);
    }
}

#[derive(Message)]
#[rtype(usize)]
pub(crate) struct Connect {
    pub(crate) addr: Recipient<NewVote>,
}

impl Handler<Connect> for ElectionServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        let session_id = self.rng.gen::<usize>();
        self.sessions.insert(session_id, msg.addr);
        log::info!("new connection!");
        session_id
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

impl Handler<Disconnect> for ElectionServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);
        log::info!("session disconnected");
    }
}
