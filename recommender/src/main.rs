mod recommender_kv;
mod recommender_otf;

mod vec_db;
mod helpers;
mod common;

use serde::Deserialize;
use fastly::{Error, Request, Response};
use std::str::FromStr;

// How many recommendations?
const MAX_RECS: usize = 50;

// E.g, http://127.0.0.1:7676/?ids=84948,97843,85035,753076,569378&offset=0&recs=50
// https://edgeml-recommender-engine.edgecompute.app/?ids=84948,97843,85035,753076,569378&offset=0&recs=50
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let qs: QueryParams = req.get_query().unwrap();

    let fastly_service_version = std::env::var("FASTLY_SERVICE_VERSION").unwrap_or("0".to_string());
    let recs: Vec<u32> = match fastly_service_version.as_str() {
        "0" => {
            // In the local environment, build HnswMap on the fly (less performant).
            recommender_otf::get_recommendations(&qs.ids, qs.offset, qs.recs).unwrap()
        }
        _ => {
            // In the production environment, load pre-compiled HnswMaps from KV Store (very performant).
            recommender_kv::get_recommendations(&qs.ids, qs.offset, qs.recs).unwrap()
        }
    };

    Ok(Response::from_body(format!("{:?}", recs)))
}

#[derive(Debug, Deserialize, Default)]
struct QueryParams {
    #[serde(deserialize_with = "deserialize_ids")]
    #[serde(default = "default_ids")]
    ids: Vec<u32>,
    #[serde(default)]
    offset: usize,
    #[serde(default = "default_recs")]
    recs: usize,
}

fn default_recs() -> usize {
    MAX_RECS
}

fn default_ids() -> Vec<u32> {
    vec![]
}

fn deserialize_ids<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ids_string: String = Deserialize::deserialize(deserializer)?;
    let ids: Vec<u32> = ids_string.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str(s).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<u32>, _>>()?;
    Ok(ids)
}
