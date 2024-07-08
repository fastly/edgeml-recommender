import os
from sklearn.cluster import KMeans
import numpy as np
import json
import matplotlib.pyplot as plt
import seaborn as sns
import time
from itertools import groupby

desired_k_clusters = 500

# Get the directory path of the current script.
script_dir = os.path.dirname(os.path.abspath(__file__))

combined_embeddings_file = os.path.join(script_dir, "../data/embeddings/combined.json")
output_dir_partitions = os.path.join(script_dir, "../data/clusters")

if not os.path.exists(combined_embeddings_file):
    print(
        f"\t✋ Combined data file ({combined_embeddings_file}) deosn't exist. Run create-embeddings.py first."
    )
    exit(1)

print("Loading combined embeddings...")

with open(combined_embeddings_file, "r") as f:
    combined_data = json.load(f)

# Partition the embeddings.
print(f"\n🪣  Performing k-means clustering, desired clusters {desired_k_clusters}:")
os.environ["TOKENIZERS_PARALLELISM"] = "true"
start_time = time.time()

# Convert the vectors from the combined data to a DataFrame.
vectors = np.array([entry["vector"] for entry in combined_data])

# Perform k-means quantization.
kmeans = KMeans(n_clusters=desired_k_clusters, random_state=0).fit(vectors)

# Predict the cluster assignments and get the centroid vector for each cluster.
cluster_assignments = kmeans.predict(vectors)
centroid_vectors = kmeans.cluster_centers_

# Update the combined data with cluster assignment.
for i in range(len(combined_data)):
    combined_data[i]["cluster"] = str(cluster_assignments[i])

os.makedirs(output_dir_partitions, exist_ok=True)

# Store centroids.
# The format is compact: [ f64[], ... ]
# The array index is the centroid ID.
# f64[] is the centroid vector, corresponding to the arithmetic mean of embeddings in the cluster..

centroids = [vector.tolist() for _, vector in enumerate(centroid_vectors)]
with open(f"{output_dir_partitions}/centroids.json", "w") as f:
    json.dump(centroids, f)

# Store combined data with cluster assignments.
# The format is compact: [ [ (cluster0) [embedding_id, f64[]], ... ], [ (cluster1) [], ... ], ...]
# The outermost array index is the cluster ID.
# The innermost arrays contain the embedding ID (corresponding to an id in the dataset)
# and the embedding vector.

# Sort the combined data by cluster.
sorted_objects = sorted(combined_data, key=lambda obj: int(obj["cluster"]))
# Group the sorted objects by cluster.
grouped_objects = groupby(sorted_objects, key=lambda obj: int(obj["cluster"]))
# Transform the grouped objects into the compact combined data format.
compact_combined_data = [
    [[obj["id"], obj["vector"]] for obj in group] for _, group in grouped_objects
]
with open(
    f"{output_dir_partitions}/combined.json",
    "w",
) as f:
    json.dump(compact_combined_data, f)

print(
    f"\n\t🤙 Partitioned {len(combined_data)} datapoints in {desired_k_clusters} clusters: {time.time() - start_time:.2f} seconds"
)

# Glean the shape of the data.
# Plot the distribution of vectors per cluster.
unique, counts = np.unique(cluster_assignments, return_counts=True)
cluster_distribution = dict(zip(unique, counts))

min_count = min(cluster_distribution.values())
max_count = max(cluster_distribution.values())

print(f"\n\tPer cluster: {min_count} to {max_count} vectors")

# Calculate mean and standard deviation.
mean_count = np.mean(counts)
std_dev_count = np.std(counts)

# Define a threshold for "even" distribution.
threshold = 0.1 * mean_count

# Check if the distribution is even.
if all(abs(count - mean_count) <= threshold for count in counts):
    print("\tDistribution is roughly even.")
else:
    print(f"\tMean count: {mean_count:.2f}, standard deviation: {std_dev_count:.2f}")

# Pretty print the distribution as a histogram.
plot_file_name = f"{output_dir_partitions}/distribution-k{desired_k_clusters}.png"
plt.figure(figsize=(15, 6))
sns.barplot(x=list(cluster_distribution.keys()), y=list(cluster_distribution.values()))
plt.xticks([])
plt.xlabel(f"{desired_k_clusters} clusters")
plt.ylabel("Number of vectors")
plt.title("Distribution of vectors per cluster")
plt.savefig(plot_file_name, bbox_inches="tight")
plt.close()
print(f"\n\t📊 Cluster distribution plot: {plot_file_name}")

print("\tDone! 🎉")
