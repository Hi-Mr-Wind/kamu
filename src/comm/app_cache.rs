use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use lazy_static::lazy_static;
use tokio::time::interval;

lazy_static!{
  pub static ref CACHE_INSTANCE: Arc<MemoryCache> = Arc::new(MemoryCache::new());
}

#[derive(Debug,Clone)]
struct CacheEntry {
    value: String,
    expiration: Option<Instant>,
}

#[derive(Clone)]
pub struct MemoryCache {
    map: DashMap<String, CacheEntry>,
    counter: Arc<AtomicUsize>,
}

impl MemoryCache {
    pub fn new() -> Self {
        let cache = MemoryCache {
            map: DashMap::new(),
            counter: Arc::new(AtomicUsize::new(0)),
        };

        // 启动定期清理任务
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                cache_clone.cleanup();
            }
        });
        cache
    }

    pub fn insert(&self, key: String, value: String, ttl: Option<Duration>) {
        let expiration = ttl.map(|ttl| Instant::now() + ttl);
        let entry = CacheEntry { value, expiration };
        self.map.insert(key.clone(), entry);
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let entry = self.map.get(key).unwrap();
        let now = &Instant::now();
        let exp = &entry.expiration.unwrap();
        if now >= exp {
            None
        } else {
            let entry_clone = entry.clone();
            Some(entry_clone.value)
        }
    }

    fn cleanup(&self) {
        let now = Instant::now();
        self.map.retain(|_, entry| {
            if let Some(exp) = entry.expiration {
                if now < exp {
                    true
                } else {
                    self.counter.fetch_sub(1, Ordering::SeqCst);
                    false
                }
            } else {
                true
            }
        });
    }
}