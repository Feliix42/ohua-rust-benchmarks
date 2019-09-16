use crate::detector::{ATTACK_WORDLIST, DetectorResult};
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

mod detector;

pub type Packet = String;

/// Generate a packet stream that is to be used as input for the intruder
/// detection algorithm.
///
/// The `flowcount` regulates, how many individual flows of input data are
/// generated that may be splitted up into two or more packets. The
/// `attack_percentage` determines roughly he percentage of attacks that are to
/// be generated. This may vary based on the `seed` used for the PRNG. The
/// `maximal_packet_len` determines, how long a non-attack flow may become.
pub fn generate_stream(
    flowcount: usize,
    attack_percentage: u8,
    max_packet_len: u64,
    seed: u64,
) -> VecDeque<Packet> {
    // this is just asserted to be safe and b/c this is outside the benchmark itself
    assert!(attack_percentage <= 100);

    // initialize the PRNG
    let mut rng = ChaCha12Rng::seed_from_u64(seed);

    // preprare the returned vector
    let mut stream = VecDeque::with_capacity(flowcount);
    // memorize the FlowIds of all generated attacks to verify the identified
    // threats after the run
    let mut attacks: HashSet<usize> = HashSet::new();

    for flow_number in 0..flowcount {
        let is_attack = (rng.next_u32() % 100) < attack_percentage.into();

        let flow = if is_attack {
            // mark flow number as attacked
            attacks.insert(flow_number);
            // randomly choose a string from the set above
            let pos = rng.next_u32() as usize % ATTACK_WORDLIST.len();
            String::from(ATTACK_WORDLIST[pos])
        } else {
            // generate a random string
            let char_range = b'~' - b' ';
            let generated = String::from_iter(
                (0..max_packet_len)
                    .map(|_| char::from((rng.next_u32() % char_range as u32) as u8 + b' ')),
            );

            // check if an attack was generated
            if detector::run_detector(&generated) == DetectorResult::SignatureMatch {
                attacks.insert(flow_number);
            }

            generated
        };

        // TODO: split into packets
    }

    stream
}
