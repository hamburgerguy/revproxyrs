use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use rocket::{Route, State};
use rocket_contrib::json::JsonValue;

struct CacheEntry {
    data: Vec<u8>,
    ttl: Instant,
}

struct ReverseProxy {
    cache: HashMap<String, CacheEntry>,
    origin: String,
}

impl ReverseProxy {
    fn new(origin: String) -> Self {
        ReverseProxy {
            cache: HashMap::new(),
            origin,
        }
    }

    fn handle_request(&mut self, request: &str) -> Vec<u8> {
        // Check if the request is in the cache.
        if let Some(entry) = self.cache.get(request) {
            // If the TTL has not expired, return the cached data.
            if entry.ttl > Instant::now() {
                return entry.data.clone();
            }
        }

        // Forward the request to the origin server.
        let response = self.forward_request(request);

        // Save the response in the cache.
        let entry = CacheEntry {
            data: response.clone(),
            ttl: Instant::now() + Duration::from_secs(30),
        };
        self.cache.insert(request.to_string(), entry);

        // Return the response.
        response
    }

    fn forward_request(&self, request: &str) -> Vec<u8> {
        // Forward the request to the origin server and return the response.
        // This function is not implemented in this example.
        Vec::new()
    }
}

/*
#[get("/api/blocks/0")]
fn proxy(info: State<ReverseProxy>) -> JsonValue {
    let response = reqwest::blocking::get(&info.origin).unwrap();
    json!([{        "id": "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",        "height": 0,        "version": 1,        "timestamp": 1231006505,        "tx_count": 1,        "size": 285,        "weight": 816,        "merkle_root": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",        "previousblockhash": null,        "mediantime": 1231006505,        "nonce": 2083236893,        "bits": 486604799,        "difficulty": 1    }])
}
*/

fn main() {
    /*
    let info = ReverseProxy {
        cache: HashMap::new(),
        origin: "https://blockstream.info".to_string(),
    };

    rocket::ignite()
        .manage(info)
        .mount("/", routes![proxy])
        .launch();
*/
}
