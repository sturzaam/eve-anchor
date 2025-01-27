use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::resource::CelestialResource;

pub struct Cache {
    store: Arc<Mutex<HashMap<String, (Instant, Result<Vec<(CelestialResource, f64)>, String>)>>>,
    ttl: Duration,
}

impl Cache {
    pub fn new(ttl: Duration) -> Self {
        Cache {
            store: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<Result<Vec<(CelestialResource, f64)>, String>> {
        let store = self.store.lock().unwrap();
        if let Some((timestamp, value)) = store.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn set(&self, key: String, value: Result<Vec<(CelestialResource, f64)>, String>) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, (Instant::now(), value));
    }
}

