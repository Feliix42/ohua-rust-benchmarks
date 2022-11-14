use crate::decoder::DecodedFlow;

/// the wordlist the attacks are taken from
pub static ATTACK_WORDLIST: &[&str] = &[
    "about", "after", "all", "also", "and", "any", "back", "because", "but", "can", "come",
    "could", "day", "even", "first", "for", "from", "get", "give", "good", "have", "him", "how",
    "into", "its", "just", "know", "like", "look", "make", "most", "new", "not", "now", "one",
    "only", "other", "out", "over", "people", "say", "see", "she", "some", "take", "than", "that",
    "their", "them", "then", "there", "these", "they", "think", "this", "time", "two", "use",
    "want", "way", "well", "what", "when", "which", "who", "will", "with", "work", "would", "year",
    "your",
];

#[derive(Debug, PartialEq)]
pub enum DetectorResult {
    SignatureMatch,
    Clear,
}

pub fn detect(flow: &str) -> DetectorResult {
    // run preprocessing
    let tmp = flow.to_lowercase();
    // FIXME: Originally, the authors also envisioned normalization from escape
    // sequences, however this was not used in the benchmark code from STAMP

    // run the signature detection
    for signature in ATTACK_WORDLIST {
        if tmp.contains(signature) {
            return DetectorResult::SignatureMatch;
        }
    }

    DetectorResult::Clear
}

pub fn bind_detect(flow: Option<DecodedFlow>) -> Option<usize> {
    flow.and_then(|decoded_flow| {
        if detect(&decoded_flow.data) == DetectorResult::SignatureMatch {
            Some(decoded_flow.flow_id)
        } else {
            None
        }
    })
}

pub struct AttackDetector {
    attacks: Vec<usize>,
}

impl AttackDetector {
    pub fn new() -> Self {
        AttackDetector { attacks: Vec::new() }
    }

    pub fn detect(&mut self, decoded: Option<DecodedFlow>) {
        match bind_detect(decoded) {
            Some(flow_id) => self.attacks.push(flow_id),
            _ => ()
        }
    }

    pub fn get_attacks(self) -> Vec<usize> {
        self.attacks
    }
}

pub fn get_attacks(detected:Vec<Option<usize>>) -> Vec<usize> {
    let mut rs = Vec::new();
    for d in detected {
        match d {
            Some(a) => rs.push(a),
            _ => ()
        }
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_recognition() {
        assert_eq!(run_detector("test"), DetectorResult::Clear);
        assert_eq!(run_detector("abouts"), DetectorResult::SignatureMatch);
    }

    #[test]
    fn capitalization_normalization() {
        assert_eq!(run_detector("aBoUt"), DetectorResult::SignatureMatch);
    }

    //     #[test]
    //     fn escape_sequence_normalization() {
    //         assert_eq!(run_detector("%41Bout"), DetectorResult::SignatureMatch);
    //     }
}
