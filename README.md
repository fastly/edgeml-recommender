# POC: Recommendation System on Fastly Compute (Rust 🦀)

This repo is a batteries-included experiment in building a performant, ML-powered recommendation system on top of [Fastly Compute](https://www.fastly.com/products/edge-compute). It was inspired by [`fastly/compute-recommender-met-demo`](https://github.com/fastly/compute-recommender-met-demo). Make sure to check out [the excellent explanation video](https://www.youtube.com/watch?v=1oheoNras9Q) on Fastly Developers Live.

## 👀 See it in action

The [demo](./met-example/) uses the [New York Met Museum's](https://www.metmuseum.org/about-the-met/policies-and-documents/open-access#get-started-header) open data to add a recommender feature to the Met's website.

Go to [edgeml-recommender.edgecompute.app/art/collection/search/1](https://edgeml-recommender.edgecompute.app/art/collection/search/1) and start browsing different objects in the Met Museum's collection. 

As you browse, your personalised recommendations will be displayed on-page, under the `✨ For you: other artworks matching your interests` heading.

Open the developer console to see the recommendation engine backend response time:

```
✨ Recommendations generated in 46.39ms ✨`
```

(^ 🚀That's really fast 🚀 ^)

## 🤖 ML at the edge

> 🤯  [Fastly Compute](https://www.fastly.com/resources/datasheets/edge-compute/fastly-compute/) is an _extremely_ fast and secure edge compute platform. It compiles your custom code to [WebAssembly (Wasm)](https://webassembly.org/) and runs it at the Fastly edge, instantly. However, the maximum compiled package size for a Compute service is 100MB. The maximum heap size is 128MB. Wasm means there's no multithreading or filesystem access. This means that 

This proof-of-concept will show you how to build a performant similarity search engine that can power recommendations for half a million objects, entirely at Fastly's edge. 

### Preparing the data

To prepare this demo, a pre-processing script was used to generate a word description for each object in the Met open dataset. 

[Embeddings](https://developers.google.com/machine-learning/crash-course/embeddings/video-lecture) were then computed for each description using the lightweight [`sentence-transformers/all-MiniLM-L12-v2`](https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2) language model.

> Embeddings make it easier to do machine learning on large inputs like sparse vectors representing words. Ideally, an embedding captures some of the semantics of the input by placing semantically similar inputs close together in the embedding space. An embedding can be learned and reused across models.
> – [Google Machine Learning Crash Course](https://developers.google.com/machine-learning/crash-course/embeddings/video-lecture)

This resulted in 384-dimensional dense vector representations for each object in the Met dataset, which were then reduced to **5 dimensions** using [principal component analysis (PCA)](https://en.wikipedia.org/wiki/Principal_component_analysis).

> Example: 🖼️ [Taking Up The Net by Thomas Eakins](https://www.metmuseum.org/art/collection/search/10826)
> ```json
> {
>    "id": 10826,
>    "vector": [-0.26523229479789734, -0.0947713553905487, -0.1279277801513672, 0.013157757930457592, -0.045752234756946564]
> }
> ```

The resulting embeddings were partitioned into **500 clusters** using [K-means clustering](https://towardsdatascience.com/understanding-k-means-clustering-in-machine-learning-6a6e67336aa1), with each cluster represented by a **centroid**–the arithmetic mean of all its embeddings. 

[Hierarchical Navigable Small World graphs](https://arxiv.org/abs/1603.09320) were computed for the centroids and for each cluster, and precompiled to [bincode](https://github.com/bincode-org/bincode), a binary zero-fluff encoding scheme chosen for its [deserialization performance in Rust](https://github.com/djkoloski/rust_serialization_benchmark).

> Hierarchical Navigable Small World (HNSW) is a method for finding similar items quickly. It builds multiple layers of linked points, where each layer helps in narrowing down the search. Items are randomly assigned to layers, with fewer items in higher layers. By navigating from the top layer down, the method quickly zooms in on the most similar items.

The precompiled HNSW maps were stored in a [KV Store](https://docs.fastly.com/en/guides/working-with-kv-stores), to enable high-performance, low-latency access from Compute.

### What happens at the edge

A Fastly Compute service acts as our ML inference backend. It receives a request with a list of Met Museum object IDs, representing someone's browsing history. 

Having loaded all embeddings in memory, the Compute program calculates a median vector for the embeddings corresponding to these object IDs, to represent an approximation of browsing interest.

It uses this median vector to perform a [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)-search on the HNSW map of **centroids**, the result of which identifies a **cluster** of embeddings that are most similar to browsing interest, from which recommendations will be computed.

A precompiled HNSW map of the cluster is loaded from KV Store, and a cosine similarity search is performed to return the most relevant recommendations.

This all happens in the blink of an eye.

## 🔧 Experiment with your own data & model

### Pre-requisites
* [Git LFS](https://git-lfs.com/) 
* [Python3](https://www.python.org/downloads/)
* Python script dependencies:
    ```sh
    pip3 install -r requirements.txt 
    ```
* [Fastly CLI](https://www.fastly.com/documentation/reference/tools/cli/#installing)
* [Rust](https://www.fastly.com/documentation/guides/compute/#install-language-tooling) language tooling for Fastly Compute
    ```sh
    rustup target add wasm32-wasi --toolchain stable
    ```

### 1. Get a copy of the dataset

Clone this repo and its submodules and download all LFS objects:

```sh
git lfs clone --recurse-submodules https://github.com/fastly/edgeml-recommender.git
```

This includes the `MetObjects.csv` dataset from [`metmuseum/openaccess`](https://github.com/metmuseum/openaccess).

### 2. Pre-process the data

Take a peek at the pre-processing script that creates naive descriptions for each object in the dataset. You can experiment with the fields you'd like to include from the dataset. When you're happy, run the script:

```sh
python3 scripts/preprocess.py
```

### 3. Create embeddings

Feel free to experiment with different [language models](https://huggingface.co/models?pipeline_tag=sentence-similarity&sort=downloads) and vector dimensionality by changing the `model_name` and `desired_embedding_dimensions` inside the [`create-embeddings.py`](./scripts/create-embeddings.py) script. Depending on your configuration and your local machine, generating around half a million embeddings from pre-processed data will take a long time! ☕ 

```sh
python3 scripts/create-embeddings.py
```

The output of this step is a combined embeddings file, in JSON format. 

> ✨ For convenience, this repo includes a [complete set of embeddings](./data/embeddings/combined.json) generated using [`sentence-transformers/all-MiniLM-L12-v2`](https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2) and [PCA](https://en.wikipedia.org/wiki/Principal_component_analysis)-fit reduction to 5 dimensions.

### 4. Partition the embeddings

Feel free to adjust the `desired_k_clusters` in the [`partition.py`](./scripts/partition.py) script, aiming for no more than _10K embeddings_ per cluster:

```sh
python3 partition.py
```

This partitions all embeddings into **clusters** and compute the [**centroid**](https://towardsdatascience.com/understanding-k-means-clustering-in-machine-learning-6a6e67336aa1) for each cluster. The re-organized embeddings and centroids will stored, separately, in JSON  format. 

### 5. Precompile the search graphs

Next, compute and precompile the [HNSW graphs](https://towardsdatascience.com/similarity-search-part-4-hierarchical-navigable-small-world-hnsw-2aad4fe87d37) for the centroids and for each cluster. 

```sh
cd precompiler
cargo run
```

This generates [bincode](https://github.com/bincode-org/bincode) files in [`data/precompiled`](./data/precompiled/).

It also creates a bincode version of the clusters input file in [`data/clusters`](./data/clusters/), for fast deserialization at the edge.

> ✨ For convenience, this repo includes sample `data/clusters/combined.bincode` and `data/precompiled/centroids-map.bincode` files for K=500.

### 6. Publish & upload to KV Store

Publish your Compute program ([`recommender`](./recommender/)) to Fastly's network:

```sh
cd recommender
fastly compute publish
```

Run the custom post_build script when prompted:
```sh
INFO: This project has a custom post_build script defined in the fastly.toml manifest:
...
Do you want to run this now? [y/N] y
```

Make a note of your new Compute domain:
```sh
View this service at:
        https://edgeml-recommender-engine.edgecompute.app
```

A successful run will also create a [KV Store](https://www.fastly.com/documentation/guides/concepts/edge-state/data-stores/#kv-stores), `vector_db`, as specified in [`recommender/fastly.toml`](./recommender/fastly.toml). List all KV stores and **make a note of the `vector_db` store ID**:

```sh
fastly kv-store list
```

Then run the following script to upload to the KV Store all precompiled HNSW graphs from the previous step (this will take a few minutes ☕):

```sh
../scripts/upload-to-kv-store.sh YOUR_STORE_ID
```

That's it! You can now send requests to the recommendation engine by passing it a comma-separated list of `ids`–in this case, of objects in the Met Museum's collection–and the desired number of recommendations, `recs`:

```
curl -s https://edgeml-recommender-engine.edgecompute.app/?ids=84948,97843,85035,753076,569378&recs=50
```
