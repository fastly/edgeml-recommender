use crate::common::{CentroidMap, EmbeddingCluster, Vectors};

use once_cell::sync::Lazy;
use std::time;

pub const DIMENSIONS: usize = 5;
const CENTROIDS: &[u8] = include_bytes!("../../data/precompiled/centroids-map.bincode");
const CLUSTERS: &[u8] = include_bytes!("../../data/clusters/combined.bincode");

pub static VEC_DB: Lazy<Vectors<DIMENSIONS>> = Lazy::new(|| {
    println!("💅 WIZENING 💅");

    println!("Loading centroid map...");
    let start = time::Instant::now();

    // Load the centroid HnswMap in memory.
    let centroid_map: CentroidMap<DIMENSIONS> =
        bincode::deserialize(CENTROIDS).expect("Failed to parse centroid map");

    println!("\tLoaded HnswMap for centroids in {:?}", start.elapsed());

    println!("Loading embeddings...");
    let start = time::Instant::now();

    // Load all vectors in memory.
    // [[[id:str, embedding:f32[]], ...], ...]
    // clusters[0] is all vectors in cluster 0, embeddings[1] is all vectors in cluster 1, etc.
    let clusters: Vec<EmbeddingCluster<DIMENSIONS>> =
        bincode::deserialize(CLUSTERS).expect("Failed to parse embeddings data");

    println!(
        "\tLoaded {} embeddings in {} clusters in {:?}",
        clusters.iter().map(|e| e.len()).sum::<usize>(),
        clusters.len(),
        start.elapsed()
    );

    Vectors {
        centroid_map,
        clusters,
    }
});

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    Lazy::force(&VEC_DB);
}
