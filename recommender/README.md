# Similarity search recommendation engine for Fastly Compute

This is a short [Fastly Compute](https://www.fastly.com/products/edge-compute) program written in Rust 🦀. It provides ML-powered content recommendations based on a given set of content IDs.

## Running locally
To test this Compute program locally, open this folder and run:

```sh
fastly compute serve
```

Then, use `curl` in a separate shell (or use your browser) to make HTTP requests to the local server. Watch the log output in the first shell.

```sh
curl -s http://127.0.0.1:7676/\?ids\=84948,97843,85035,753076,569378
# ids - comma-separated ids of objects in the Met Collection
```

The local development version of this program generates HNSW graphs on the fly ([`src/recommender_otf.rs`](./src/recommender_src.rs)). Inference latency increases proportionally with the number of embeddings in a target cluster. 

The production version ([`src/recommender_kv.rs`](./src/recommender_src.rs)) uses precompiled search graphs stored in a Fastly [KV Store](https://www.fastly.com/products/kv-store), and consistently achieves 🚀 **sub-100ms** 🚀 response times for searches on the Met Museum's entire dataset (480K objects).

To build and publish your recommendation system, follow the instructions in [../README.md](../README.md#experiment-with-your-own-data--model).
