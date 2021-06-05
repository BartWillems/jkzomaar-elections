use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, web, HttpResponse, Responder};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::errors::ElectionError;

pub(crate) struct Election {
    inner: Mutex<InnerElection>,
}

impl Election {
    pub(crate) fn new() -> Self {
        Self {
            inner: Mutex::new(InnerElection {
                ballots: HashSet::new(),
                candidates: HashSet::new(),
                votes: HashMap::new(),
            }),
        }
    }

    pub(crate) async fn create_ballot(&self) -> Result<Uuid, ElectionError> {
        let mut inner = self.inner.lock().await;
        let uuid = Uuid::new_v4();
        match inner.ballots.insert(uuid) {
            true => Ok(uuid),
            false => Err(ElectionError::DuplicateBallotCreation),
        }
    }

    pub(crate) async fn vote(&self, id: Uuid, candidate: String) -> Result<(), ElectionError> {
        let mut inner = self.inner.lock().await;

        if !inner.ballots.contains(&id) {
            return Err(ElectionError::InvalidUuid);
        }

        if !inner.candidates.contains(&candidate) {
            return Err(ElectionError::InvalidCandidate);
        }

        if inner.votes.contains_key(&id) {
            return Err(ElectionError::AlreadyVoted);
        }

        inner.votes.insert(id, candidate);

        Ok(())
    }
}

struct InnerElection {
    /// available ballots
    ballots: HashSet<Uuid>,
    /// Possible candidates
    candidates: HashSet<String>,
    /// Pool of cast ballots
    votes: HashMap<Uuid, String>,
}

#[post("/ballots")]
async fn create_ballot(
    state: Data<crate::State>,
) -> Result<actix_web::HttpResponse, ElectionError> {
    let uuid = state.election.create_ballot().await?;

    Ok(HttpResponse::Created().json(uuid))
}

#[post("/ballots/{uuid}")]
async fn vote(
    uuid: Path<Uuid>,
    candidate: Json<String>,
    state: Data<crate::State>,
) -> Result<actix_web::HttpResponse, ElectionError> {
    state
        .election
        .vote(uuid.into_inner(), candidate.into_inner())
        .await?;

    Ok(HttpResponse::Created().json("Great Success!"))
}

/// This should be used to display the candidate list to the user
#[get("/ballots/{uuid}")]
async fn show_ballot(uuid: Path<Uuid>) -> Result<actix_web::HttpResponse, ElectionError> {
    todo!();
}

pub(crate) fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_ballot);
    cfg.service(vote);
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
