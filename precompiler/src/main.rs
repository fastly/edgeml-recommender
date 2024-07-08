use instant_distance::Builder;
use recommender::{CentroidMap, ClusterMap, EmbeddingCluster, Point};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time;

const CENTROIDS: &str = include_str!("../../data/clusters/centroids.json");
const CLUSTERS: &str = include_str!("../../data/clusters/combined.json");

const OUTPUT_DIR: &str = "../data/precompiled";

fn main() -> Result<(), std::io::Error> {
    println!("Loading centroids...");
    let start = time::Instant::now();

    // Load all centroids in memory.
    // [centroid:f32[], ...]
    // centroids[0] is the centroid for cluster 0.
    let centroids: Vec<Point<5>> =
        serde_json::from_str(CENTROIDS).expect("Failed to parse centroids data");

    let centroids_length = centroids.len();

    println!(
        "\tLoaded {} centroids in {:?}",
        centroids_length,
        start.elapsed()
    );

    let start = time::Instant::now();
    let centroid_map: CentroidMap<5> =
        Builder::default().build(centroids, (0..centroids_length).collect());

    println!("\tBuilt HnswMap for centroids in {:?}", start.elapsed());

    #[cfg(feature = "msgpack")]
    write_it(
        rmp_serde::encode::to_vec(&centroid_map).unwrap(),
        &format!("centroids-map-{}.msgpack", centroids_length),
    )?;
    #[cfg(feature = "bincode")]
    write_it(
        bincode::serialize(&centroid_map).unwrap(),
        &format!("centroids-map-{}.bincode", centroids_length),
    )?;

    println!("Loading embeddings...");
    let start = time::Instant::now();

    // Load all vectors in memory.
    // [[[id:str, embedding:f32[]], ...], ...]
    // clusters[0] is all vectors in cluster 0, embeddings[1] is all vectors in cluster 1, etc.
    let clusters: Vec<EmbeddingCluster<5>> =
        serde_json::from_str(CLUSTERS).expect("Failed to parse embeddings data");

    println!(
        "\tLoaded {} embeddings in {} clusters in {:?}",
        clusters.iter().map(|e| e.len()).sum::<usize>(),
        clusters.len(),
        start.elapsed()
    );

    #[cfg(feature = "msgpack")]
    write_it(
        rmp_serde::encode::to_vec(&clusters).unwrap(),
        "../clusters/combined.msgpack",
    )?;
    #[cfg(feature = "bincode")]
    write_it(
        bincode::serialize(&clusters).unwrap(),
        "../clusters/combined.bincode",
    )?;

    for n in 0..clusters.len() {
        // Build HnswMap
        println!(
            "Building HnwsMap for {} embeddings in cluster {}...",
            clusters[n].len(),
            n
        );
        let start = time::Instant::now();

        // e: [id, embedding:f32[]]
        let (vecs, ids): (Vec<_>, Vec<_>) = clusters[n].iter().map(|e| (e.1.clone(), e.0)).unzip();

        let cluster: ClusterMap<5> = Builder::default().build(vecs, ids);

        println!("\tDone in: {:?}", start.elapsed());

        #[cfg(feature = "msgpack")]
        write_it(
            rmp_serde::encode::to_vec(&cluster).unwrap(),
            &format!("cluster-map-{}.msgpack", n),
        )?;
        #[cfg(feature = "bincode")]
        write_it(
            bincode::serialize(&cluster).unwrap(),
            &format!("cluster-map-{}.bincode", n),
        )?;
    }

    println!("All done.");
    Ok(())
}

fn write_it(val: Vec<u8>, output_name: &str) -> Result<(), std::io::Error> {
    let file_path = format!("{}/{}", OUTPUT_DIR, output_name);

    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&val)?;
    writer.flush()?;

    let size_in_mb = val.len() as f64 / (1024.0 * 1024.0);
    println!("\t{:.2} MB written to disk: {}.", size_in_mb, file_path);

    Ok(())
}
