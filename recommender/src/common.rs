use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use instant_distance::HnswMap;

pub type CentroidMap<const N: usize> = HnswMap<Point<N>, usize>;

pub type ClusterMap<const N: usize> = HnswMap<EmbeddingPoint<N>, u32>;

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Point<const N: usize>(#[serde_as(as = "[_; N]")] pub [f32; N]);

impl<const N: usize> instant_distance::Point for Point<N> {
    // Euclidean distance
    fn distance(&self, other: &Self) -> f32 {
        let mut sum_of_squares = 0.0;
        for i in 0..N {
            sum_of_squares += (self.0[i] - other.0[i]).powi(2);
        }
        (sum_of_squares as f32).sqrt()
    }
}

impl<const N: usize> From<Point<N>> for EmbeddingPoint<N> {
    fn from(point: Point<N>) -> Self {
        EmbeddingPoint(point.0)
    }
}

impl<const N: usize> From<Vec<f32>> for Point<N> {
    fn from(vec: Vec<f32>) -> Self {
        let mut array = [0.0; N];
        array.copy_from_slice(&vec);
        Point(array)
    }
}

pub type EmbeddingCluster<const N: usize> = Vec<CompactEmbedding<N>>;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CompactEmbedding<const N: usize>(pub u32, pub EmbeddingPoint<N>);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmbeddingPoint<const N: usize>(#[serde_as(as = "[_; N]")] pub [f32; N]);

impl<const N: usize> instant_distance::Point for EmbeddingPoint<N> {
    // Cosine similarity
    fn distance(&self, other: &Self) -> f32 {
        let mut dot_product = 0.0;
        let mut magnitude_self = 0.0;
        let mut magnitude_other = 0.0;

        for i in 0..N {
            dot_product += self.0[i] * other.0[i];
            magnitude_self += self.0[i].powi(2);
            magnitude_other += other.0[i].powi(2);
        }

        let magnitude_self = magnitude_self.sqrt();
        let magnitude_other = magnitude_other.sqrt();

        (dot_product / (magnitude_self * magnitude_other)) as f32
    }
}

impl<const N: usize> From<EmbeddingPoint<N>> for Vec<f32> {
    fn from(embedding_point: EmbeddingPoint<N>) -> Self {
        embedding_point.0.to_vec()
    }
}

pub struct Vectors<const N: usize> {
    pub centroid_map: CentroidMap<N>,
    pub clusters: Vec<EmbeddingCluster<N>>,
}

pub fn find_embedding_point_by_id<const N: usize>(
    clusters: &[EmbeddingCluster<N>],
    target_id: u32,
) -> Option<EmbeddingPoint<N>> {
    clusters
        .iter()
        .flat_map(|cluster| cluster.iter())
        .find_map(|embedding: &CompactEmbedding<N>| {
            if embedding.0 == target_id {
                Some(embedding.1.clone())
            } else {
                None
            }
        })
}
