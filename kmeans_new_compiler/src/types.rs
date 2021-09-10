use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

/// A single value (in the original STAMP benchmark referred to as object).
#[derive(Clone, Debug)]
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

    /// calcualates the multi-dimensional spatial Euclid distance square
    fn euclidian_distance(&self, centroid: &Centroid) -> f32 {
        let mut sum = 0f32;

        for idx in 0..self.values.len() {
            sum += (self.values[idx] - centroid.coordinates[idx]).powi(2)
        }

        sum
    }

    /// Finds the nearest centroid by using euclidian distance for calculation. Returns the index of the matching centroid
    pub fn find_nearest_centroid(&self, centroids: &Vec<Centroid>) -> usize {
        let mut best_fit = 0;
        let mut best_distance = std::f32::MAX;

        for idx in 0..centroids.len() {
            let distance = self.euclidian_distance(&centroids[idx]);
            if distance < best_distance {
                best_distance = distance;
                best_fit = idx;
            }
        }

        best_fit
    }
}

#[derive(Clone, Debug)]
pub struct Centroid {
    pub coordinates: Vec<f32>,
}

impl Centroid {
    pub fn randomly_generate(values: &Vec<Value>, cluster_count: usize) -> Vec<Self> {
        // initialize the PRNG -- the original implementation seeded with 7 (why?),
        // but since we are using a different RNG that's not really relevant ¯\_(ツ)_/¯
        let mut rng = ChaCha12Rng::seed_from_u64(0);
        let mut centroids = Vec::new();

        let number_of_values = values.len();
        for _ in 0..cluster_count {
            let idx = rng.next_u64() as usize % number_of_values;

            centroids.push(Self {
                coordinates: values[idx].values.clone(),
            });
        }

        centroids
    }

    pub fn from_assignments(values: &Vec<Vec<Value>>, num_centroids: usize) -> Vec<Self> {
        let mut sums = vec![vec![0f32; values[0][0].values.len()]; num_centroids];
        let mut elements_in_cluster = vec![0; num_centroids];

        // form the sums for all centroids
        for val in values.iter().flatten() {
            for idx in 0..val.values.len() {
                sums[val.associated_cluster][idx] += val.values[idx];
            }
            elements_in_cluster[val.associated_cluster] += 1;
        }

        for centroid_no in 0..num_centroids {
            for sum in sums[centroid_no].iter_mut() {
                *sum /= elements_in_cluster[centroid_no] as f32;
            }
        }

        sums.into_iter()
            .map(|coords| Self {
                coordinates: coords,
            })
            .collect()
    }
}

/// Applies a zscore transformation to all values. Requires that all values in the list have the same number of attributes
pub fn apply_zscore_transform(values: &mut Vec<Value>) {
    // iterate through columns in the matrix
    for pos in 0..values[0].values.len() {
        let mut sum = 0f32;
        for val in values.iter() {
            sum += val.values[pos];
        }

        let sample_mean = sum / values.len() as f32;

        sum = 0f32;
        for val in values.iter() {
            sum += (val.values[pos] - sample_mean).powi(2);
        }
        let sample_std_derivation = (sum / values.len() as f32).sqrt();

        // now apply the zscore transformation to all values
        for val in values.iter_mut() {
            val.values[pos] = (val.values[pos] - sample_mean) / sample_std_derivation;
        }
    }
}

#[inline(always)]
pub fn create_centroids(values: Vec<Vec<Value>>, old_centroids: Arc<Vec<Centroid>>) -> (Vec<Vec<Value>>, Arc<Vec<Centroid>>) {
    let cs = Arc::new(Centroid::from_assignments(&values, old_centroids.len()));

    (values, cs)
}

#[inline(always)]
pub fn inc(run_no: u32) -> u32 {
    run_no + 1
}

pub fn reassign_values(mut values: Vec<Value>, centroids: Arc<Vec<Centroid>>) -> (Vec<Value>, u32) {
    let mut delta = 0;

    for value in values.iter_mut() {
        let new_cluster = value.find_nearest_centroid(&centroids);
        if new_cluster != value.associated_cluster {
            value.associated_cluster = new_cluster;
            delta += 1;
        }
    }

    (values, delta)
}

#[inline(always)]
pub fn should_continue(current_delta: f32, threshold: f32, runs: u32) -> bool {
    current_delta > threshold && runs < 500
    // runs < 500
}

pub fn evaluate_results(values: Vec<(Vec<Value>, u32)>) -> (Vec<Vec<Value>>, f32) {
    let (new_values, deltas): (Vec<Vec<Value>>, Vec<u32>) = values.into_iter().unzip();

    let current_delta = deltas.into_iter().sum::<u32>() as f32 / new_values.len() as f32;

    (new_values, current_delta)
}
