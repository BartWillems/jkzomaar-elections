use std::collections::{HashMap};

use actix::prelude::*;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, web, HttpResponse};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{Connection, PgPool};
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
    async fn cast(vote: Vote, pool: &PgPool) -> Result<Results, ElectionError> {


        let mut conn = pool.acquire().await?;

        let scores: Results = conn.transaction(|conn| Box::pin(async move {
            sqlx::query!("LOCK TABLE votes IN ACCESS EXCLUSIVE MODE").execute(&mut *conn).await?;

            let vote = sqlx::query_as!(
                Vote, 
                r#"INSERT INTO votes (ballot_id, voorzitter, ondervoorzitter, penning_meester, secretaris)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *"#,
                vote.ballot_id, vote.voorzitter, vote.ondervoorzitter, vote.penning_meester, vote.secretaris,
            ).fetch_one(&mut *conn).await?;

            sqlx::query!("UPDATE ballots SET cast_time = NOW() WHERE id=$1", vote.ballot_id).execute(&mut *conn).await?;

            let scores = Election::results_tx(&mut *conn).await?;

            sqlx::Result::Ok(scores)
        })).await?;

        log::debug!("Scores: {:?}", scores);


        Ok(scores)
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

struct Election;

#[derive(Debug, Serialize)]
struct Candidate {
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Candidates {
    voorzitters: Vec<Candidate>,
    ondervoorzitters: Vec<Candidate>,
    penning_meesters: Vec<Candidate>,
    secretarissen: Vec<Candidate>,
}

#[derive(Debug, Serialize, Clone)]
struct Score {
    count: i64,
    name: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Results {
    voorzitters: Vec<Score>,
    ondervoorzitters: Vec<Score>,
    penning_meesters: Vec<Score>,
    secretarissen: Vec<Score>,
}

impl Election {
    async fn load_candidates(pool: &PgPool) -> Result<Candidates, sqlx::Error> {
        let voorzitters = sqlx::query_as!(Candidate, "SELECT * FROM voorzitters").fetch_all(pool);
        let ondervoorzitters = sqlx::query_as!(Candidate, "SELECT * FROM ondervoorzitters").fetch_all(pool);
        let secretarissen = sqlx::query_as!(Candidate, "SELECT * FROM secretarissen").fetch_all(pool);
        let penning_meesters = sqlx::query_as!(Candidate, "SELECT * FROM penning_meesters").fetch_all(pool);

        let (voorzitters, ondervoorzitters, secretarissen, penning_meesters) = futures::try_join!(voorzitters, ondervoorzitters, secretarissen, penning_meesters)?;

        Ok(Candidates {
            voorzitters,
            ondervoorzitters,
            penning_meesters,
            secretarissen,
        })
    }

    async fn results_tx(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Result<Results, sqlx::Error> {
        let voorzitters = sqlx::query_as!(Score, r#"SELECT COUNT(voorzitter) as "count!", voorzitter as name FROM votes GROUP BY name"#).fetch_all(&mut *tx).await?;
        let ondervoorzitters = sqlx::query_as!(Score, r#"SELECT COUNT(ondervoorzitter) as "count!", ondervoorzitter as name FROM votes GROUP BY name"#).fetch_all(&mut *tx).await?;
        let secretarissen = sqlx::query_as!(Score, r#"SELECT COUNT(secretaris) as "count!", secretaris as name FROM votes GROUP BY name"#).fetch_all(&mut *tx).await?;
        let penning_meesters = sqlx::query_as!(Score, r#"SELECT COUNT(penning_meester) as "count!", penning_meester as name FROM votes GROUP BY name"#).fetch_all(&mut *tx).await?;
        
        Ok(Results {
            voorzitters,
            ondervoorzitters,
            penning_meesters,
            secretarissen,
        })
    }

    async fn results(pool: &PgPool) -> Result<Results, sqlx::Error> {
        let vzt = sqlx::query_as!(Score, r#"SELECT COUNT(voorzitter) as "count!", voorzitter as name FROM votes GROUP BY name"#).fetch_all(pool);
        let ovzt = sqlx::query_as!(Score, r#"SELECT COUNT(ondervoorzitter) as "count!", ondervoorzitter as name FROM votes GROUP BY name"#).fetch_all(pool);
        let sec = sqlx::query_as!(Score, r#"SELECT COUNT(secretaris) as "count!", secretaris as name FROM votes GROUP BY name"#).fetch_all(pool);
        let pm = sqlx::query_as!(Score, r#"SELECT COUNT(penning_meester) as "count!", penning_meester as name FROM votes GROUP BY name"#).fetch_all(pool);
        

        let (voorzitters, ondervoorzitters, secretarissen, penning_meesters) = futures::try_join!(vzt, ovzt, sec, pm)?;

        Ok(Results {
            voorzitters,
            ondervoorzitters,
            penning_meesters,
            secretarissen,
        })
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
    let election_score = Ballot::cast(vote.into_inner(), &state.db).await?;

    if let Err(e) = state.ws.send(NewVote(election_score.clone())).await {
        log::error!("Unable to notify new vote: {:?}", e);
    }

    Ok(HttpResponse::Created().json(election_score))
}

/// This should be used to display the candidate list to the user
#[get("/ballots/{uuid}")]
async fn show_ballot(uuid: Path<Uuid>, state: Data<crate::State>) -> Result<actix_web::HttpResponse, ElectionError> {
    let ballot = Ballot::load(&uuid, &state.db).await?;

    Ok(HttpResponse::Ok().json(ballot))
}

#[get("/candidates")]
async fn get_candidates(state: Data<crate::State>) -> Result<actix_web::HttpResponse, ElectionError> {
    let candidates = Election::load_candidates(&state.db).await?;

    Ok(HttpResponse::Ok().json(candidates))
}

#[get("/result")]
async fn get_results(state: Data<crate::State>) -> Result<actix_web::HttpResponse, ElectionError> {
    let results = Election::results(&state.db).await?;

    Ok(HttpResponse::Ok().json(results))
}

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_ballot);
    cfg.service(cast_vote);
    cfg.service(show_ballot);
    cfg.service(get_candidates);
    cfg.service(get_results);
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
            let _ = session.do_send(msg.clone());
        }
    }
}

impl Actor for ElectionServer {
    type Context = Context<Self>;
}

#[derive(Debug, Message, Serialize, Clone)]
#[rtype(result = "()")]
pub(crate) struct NewVote(Results);

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
        log::info!("Connection size: {}", self.sessions.len());
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
