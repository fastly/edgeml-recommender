use crate::common::{find_embedding_point_by_id, ClusterMap, Point};
use crate::helpers;
use crate::vec_db::{DIMENSIONS, VEC_DB};

use fastly::Error;
use instant_distance::{Builder, MapItem, Search};
use std::ops::Deref;
use std::time;

// Takes a vector of IDs and returns nearest neighbour IDs.
pub fn get_recommendations(ids: &Vec<u32>, offset: usize, recs: usize) -> Result<Vec<u32>, Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    let vec_db = VEC_DB.deref();

    println!("Computing median vector for ids {:?}...", ids);
    let start = time::Instant::now();

    let embeddings_for_ids: Vec<Vec<f32>> = ids
        .iter()
        .filter_map(|id| find_embedding_point_by_id(&vec_db.clusters, *id))
        .map(|embedding_point| embedding_point.into())
        .collect();

    let median_v: Point<DIMENSIONS> = helpers::median_vector(&embeddings_for_ids).into();

    println!("\tComputed {:?} in {:?}", median_v, start.elapsed());

    // Find nearest cluster.
    println!("🔍 Searching for nearest cluster:");
    let start = time::Instant::now();
    let mut search = Search::default();

    let MapItem {
        value: cluster_id, ..
    } = vec_db
        .centroid_map
        .search(&median_v, &mut search)
        .next()
        .unwrap();

    println!("\tFound cluster {} in {:?}", cluster_id, start.elapsed());

    // Load vectors for cluster.
    println!("Loading vectors for cluster...");
    let start = time::Instant::now();

    // e: [id, embedding:f32[]]
    let (vecs, ids): (Vec<_>, Vec<_>) = vec_db.clusters[*cluster_id]
        .iter()
        .map(|e| (e.1.clone(), e.0))
        .unzip();

    let cluster: ClusterMap<DIMENSIONS> = Builder::default().build(vecs, ids);

    println!(
        "\tBuilt HnswMap for {} embeddings in cluster {}: {:?}",
        cluster.values.len(),
        cluster_id,
        start.elapsed()
    );

    // Find nearest neighbours within cluster.
    println!("🔍 Searching for approximate nearest neighbour(s):");
    let start = time::Instant::now();
    let mut search = Search::default();

    let nearest_neighbors: Vec<u32> = cluster
        .search(&median_v.into(), &mut search)
        .skip(offset)
        .take(recs) // Limit the results to MAX_RECS
        .map(|MapItem { value: obj_id, .. }| *obj_id)
        .collect();

    println!("\tNearest neighbours found in {:?}", start.elapsed());
    println!(
        "💅 Nearest neighbours {}-{}: {:?}",
        offset,
        offset + recs,
        nearest_neighbors
    );

    Ok(nearest_neighbors)
}
