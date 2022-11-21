use crate::vacation::prime::manager::Manager;
use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::prime::database::{Database, IndexedQuery, index_queries, compute, resolve};



/// This server algorithm pretends to know nothing.
/// It just applies changes to the database system in the order that we requests arrived.
/// For requests that address the same database, we abort and retry.
pub(crate) fn server_naive_go(db:Database, batch: Vec<IndexedQuery>, responses: Vec<Response>) -> (Manager, Vec<Response>) {

    let shared = Arc::new(db);
    let mut qd = Vec::new();
    for query in batch {
        let owned = shared.clone();
        let delta = compute(owned, query);
        qd.push(qr);
    }

    let (redo, cresponses) = db.apply_delta(qd);
    let responses_p = insert_at_index(responses, cresponses);

    if redo.not_empty() {
        server_naive_go(db, redo, responses_p)
    } else {
        (db, responses)
    }
}

pub(crate) fn server_naive(db:Database, batch: Vec<Query>) -> (Manager, Vec<Response>) {
    let batch_p = index_queries(batch);
    let l = batch_p.len();
    let responses = Vec::with_capacity(l);
    server_naive_go(db, batch_p, responses)
}

/// This server algorithm uses batching and performs reordering of queries.
/// It applies writes before reads, so reads see the most up-to-date data.
pub(crate) fn server_wr(db: Database, batch: Vec<Query>) -> (Manager,Vec<Response>) {
    let (reads, writes) = split(batch);
    let mut responses = Vec::with_capacity(batch.len());
    for write in writes {
        let resp = db.issue_write(write);
        responses.push(resp);
    }

    let shared = Arc::new(db);
    for read in reads {
        let own_db = shared.clone();
        let resp = issue_read(own_db, read);
        responses.push(resp);
    }

    (db, responses)
}

// Note that the nature of transactions changed now:
// Before the benchmark was actually spanning transactions across client calls.
// Now this is not possible anymore!
// The developer now has to handle overwrites and a client needs to restart its computation again.
// Effectively, the database system needs to either be extended to transactions or the
// client has to cope with consistency on the client side, for example via eventual consistency.
