use flate2::{write::GzEncoder, Compression};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::io::{stdin, stdout, Read, Write};

const ENTROPY_THRESHOLD: f32 = 6f32;
const SAMPLE_PERCENT: f32 = 0.5f32;

fn get_sampling_idx(len: usize) -> Vec<usize> {
    (0..(len as f32 * SAMPLE_PERCENT) as usize)
        .into_par_iter()
        .map(|_| fastrand::usize(..len))
        .collect()
}

fn compute_entropy(data: &[u8]) -> f32 {
    let indexes = get_sampling_idx(data.len());
    let total = indexes.len() as f32;

    let counts = indexes
        .into_par_iter()
        .fold(FxHashMap::default, |mut counts, idx| {
            *counts.entry(data[idx]).or_insert(0) += 1;
            counts
        })
        .reduce(FxHashMap::default, |mut counts, other_counts| {
            for (k, v) in other_counts {
                *counts.entry(k).or_insert(0) += v;
            }
            counts
        });

    counts
        .into_par_iter()
        .map(|(_, count)| {
            let p = count as f32 / total;
            -p * p.log2()
        })
        .sum::<f32>()
}

fn main() {
    // read all data from stdin
    let mut data = Vec::new();

    stdin().read_to_end(&mut data).unwrap();

    // Check the entropy
    let entropy = compute_entropy(&data);

    if entropy > ENTROPY_THRESHOLD {
        // Write all data to stdout
        stdout().write_all(&data).unwrap();
    } else {
        // compress with gzip and write to stdout
        let mut encoder = GzEncoder::new(Vec::with_capacity(data.len()), Compression::new(6));
        encoder.write_all(&data).unwrap();

        let compressed = encoder.finish().unwrap();

        stdout().write_all(&compressed).unwrap();
    }
}
