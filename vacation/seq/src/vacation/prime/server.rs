use crate::vacation::prime::manager::Manager;
use crate::vacation::prime::communication::{Query, Response};
use crate::vacation::prime::database::{Database};


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
