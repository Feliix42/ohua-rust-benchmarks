use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::prime::database::{
    compute, index_queries_and_responses, insert_at_index, issue_read, seq_arc_unwrap, split,
    unwrap_responses, Database, Delta, IndexedQuery, NotEmpty,
};
use std::sync::Arc;

pub(crate) type Server = fn(Database, Vec<Query>) -> (Database, Vec<Response>);

/// This server algorithm pretends to know nothing.
/// It just applies changes to the database system in the order that we requests arrived.
/// For requests that address the same database, we abort and retry.
#[allow(dead_code)]
pub(crate) fn naive_go(
    package: (Database, Vec<Option<Response>>),
    batch: Vec<IndexedQuery>,
) -> (Database, Vec<Option<Response>>) {
    let (mut db, responses): (Database, Vec<Option<Response>>) = package; // FIXME
    let dbp: Database = db.clone(); // certainly expensive
    let shared: Arc<Database> = Arc::new(dbp);
    let mut qd: Vec<(IndexedQuery, Option<Response>)> = Vec::new();
    for query0 in batch {
        let query: IndexedQuery = query0;
        let owned: Arc<Database> = shared.clone();
        let delta: (IndexedQuery, Option<Response>) = compute(owned, query);
        qd.push(delta);
    }

    let (redo, cresponses): (Vec<IndexedQuery>, Vec<(usize, Response)>) = db.apply_delta(qd);
    let responses_p: Vec<Option<Response>> = insert_at_index(responses, cresponses);
    let pending: bool = redo.not_empty();
    let packaged: (Database, Vec<Option<Response>>) = (db, responses_p);
    if pending {
        naive_go(packaged, redo)
    } else {
        packaged
    }
}

/// Note, this algo penalizes clients whose queries were successfull.
// We use the YCSB benchmark to show how those can be responded to and the failed ones are merged
// with the next set of queries. The failed queries being at the front of the ones worked.
// This requires showing latency metrics!
#[allow(dead_code)]
pub fn naive(db: Database, batch: Vec<Query>) -> (Database, Vec<Response>) {
    let (batch_p, responses): (Vec<IndexedQuery>, Vec<Option<Response>>) =
        index_queries_and_responses(batch);
    let state: (Database, Vec<Option<Response>>) = (db, responses);
    let (dbp, responsesp): (Database, Vec<Option<Response>>) = naive_go(state, batch_p);
    let responsespp: Vec<Response> = unwrap_responses(responsesp);
    (dbp, responsespp)
}


/// This server algorithm uses batching and performs reordering of queries.
/// It applies writes before reads, so reads see the most up-to-date data.
pub fn writes_before_reads(db: Database, batch: Vec<Query>) -> (Database, Vec<Response>) {
    let (reads, writes, mut responses): (Vec<Query>, Vec<Query>, Vec<Response>) = split(batch);
    for write0 in writes {
        let write: Query = write0;
        let resp: Response = db.issue_write(write);
        responses.push(resp);
    }

    let shared: Arc<Database> = Arc::new(db);
    for read0 in reads {
        let read: Query = read0;
        let own_db: Arc<Database> = shared.clone();
        let resp: Response = issue_read(own_db, read);
        responses.push(resp);
    }

    // TODO check whether this would also work:
    // let dbp = shared.own();
    // It should and makes for a much nicer API because it works
    // based in state threading in Ohua!
    seq_arc_unwrap(shared, responses)
}

// Note that the nature of transactions changed now:
// Before the benchmark was actually spanning transactions across client calls.
// Now this is not possible anymore!
// The developer now has to handle overwrites and a client needs to restart its computation again.
// Effectively, the database system needs to either be extended to transactions or the
// client has to cope with consistency on the client side, for example via eventual consistency.
