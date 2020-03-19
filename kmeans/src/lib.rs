use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A single value (in the original STAMP benchmark referred to as object).
pub struct Value {
    /// the contents that form the observation
    pub values: Vec<f32>,
    /// cluster the value belongs to
    pub associated_cluster: usize,
}

impl Value {
    /// Reads a Vec of values from a text file.
    pub fn load_from_text_file<P: AsRef<Path>>(txt_file: P) -> std::io::Result<Vec<Self>> {
        let f = File::open(txt_file)?;
        let reader = BufReader::new(f);

        let mut values = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut observations = line.split_whitespace();
            // ignore the first attribute
            observations.next();
            // parse the remaining items into a vector of values
            values.push(Self {
                values: observations.map(str::parse).map(Result::unwrap).collect(),
                associated_cluster: 0,
            })
        }

        Ok(values)
    }
}

/// Assigns a cluster to each value in the input vector.
pub fn randomly_assign_cluster(values: &mut Vec<Value>, cluster_count: usize) {
    // initialize the PRNG -- the original implementation seeded with 7 (why?),
    // but since we are using a different RNG that's not really relevant ¯\_(ツ)_/¯
    let mut rng = ChaCha12Rng::seed_from_u64(0);

    for val in values.iter_mut() {
        let cluster = rng.next_u64() as usize % cluster_count;
        val.associated_cluster = cluster;
    }
}

/// Applies a zscore transformation to all values. Requires that all values in the list have the same number of attributes
pub fn apply_zscore_transform(values: &mut Vec<Value>) {
    // iterate through columns in the matrix
    for pos in 0..values[0].values.len() {
        let mut sum = 0;
        for val in &values {
            sum += val.values[pos];
        }

        let sample_mean = sum / values.len();

        sum = 0;
        for val in &values {
            sum += (val.values[pos] - sample_mean).pow(2);
        }
        let sample_std_derivation = (sum / values.len()).sqrt();

        // now apply the zscore transformation to all values
        for val in values.iter_mut() {
            val.values[pos] = (val.values[pos] - sample_mean) / sample_std_derivation;
        }
    }
}
