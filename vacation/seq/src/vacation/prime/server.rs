use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::prime::database::{
    compute, index_queries_and_responses, insert_at_index, issue_read, seq_arc_unwrap, split,
    unwrap_responses, Database, Delta, IndexedQuery, NotEmpty,
};
use std::sync::Arc;

/// This server algorithm pretends to know nothing.
/// It just applies changes to the database system in the order that we requests arrived.
/// For requests that address the same database, we abort and retry.
pub(crate) fn server_naive_go(
    mut db: Database,
    batch: Vec<IndexedQuery>,
    responses: Vec<Option<Response>>,
) -> (Database, Vec<Option<Response>>) {
    let dbp = db.clone(); // certainly expensive
    let shared = Arc::new(dbp);
    let mut qd = Vec::new();
    for query in batch {
        let owned = shared.clone();
        let delta = compute(owned, query);
        qd.push(delta);
    }

    let (redo, cresponses) = db.apply_delta(qd);
    let responses_p = insert_at_index(responses, cresponses);

    if redo.not_empty() {
        server_naive_go(db, redo, responses_p)
    } else {
        (db, responses_p)
    }
}

/// Note, this algo penalizes clients whose queries were successfull.
// We use the YCSB benchmark to show how those can be responded to and the failed ones are merged
// with the next set of queries. The failed queries being at the front of the ones worked.
// This requires showing latency metrics!
pub(crate) fn server_naive(db: Database, batch: Vec<Query>) -> (Database, Vec<Response>) {
    let (batch_p, responses) = index_queries_and_responses(batch);
    let (dbp, responsesp) = server_naive_go(db, batch_p, responses);
    let responsespp = unwrap_responses(responsesp);
    (dbp, responsespp)
}

/// This server algorithm uses batching and performs reordering of queries.
/// It applies writes before reads, so reads see the most up-to-date data.
pub(crate) fn server_wr(mut db: Database, batch: Vec<Query>) -> (Database, Vec<Response>) {
    let mut responses = Vec::with_capacity(batch.len());
    let (reads, writes) = split(batch);
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

    seq_arc_unwrap(shared, responses)
}

// Note that the nature of transactions changed now:
// Before the benchmark was actually spanning transactions across client calls.
// Now this is not possible anymore!
// The developer now has to handle overwrites and a client needs to restart its computation again.
// Effectively, the database system needs to either be extended to transactions or the
// client has to cope with consistency on the client side, for example via eventual consistency.
