use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct Db {
    inner: RwLock<HashMap<String, String>>
}

impl Db {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new())
        }
    }

    pub async fn set(&self, key: String, val: String) {
        let mut map = self.inner.write().await;
        map.insert(key, val);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let map = self.inner.read().await;
        map.get(key).cloned()
    }
}