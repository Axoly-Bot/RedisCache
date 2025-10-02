use bb8_redis::{bb8::{self, Pool, PooledConnection}, RedisConnectionManager};
use redlight::RedisCache;
use std::sync::Arc;

use crate::config::Config;

mod config;

#[derive(Clone)]
pub struct Cache {
    pub twilight_cache: Arc<RedisCache<Config>>,
    pub pool: Pool<RedisConnectionManager>
}

impl Cache {
    pub async fn new() -> Self {
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();
        
        let cache = RedisCache::<Config>::new_with_pool(pool.clone())
            .await
            .unwrap();

        Self {
            twilight_cache: Arc::new(cache),
            pool
        }
    }

    pub fn get_twilight_cache(&self) -> Arc<RedisCache<Config>> {
        Arc::clone(&self.twilight_cache)
    }

    pub async fn get_conn(&self) -> PooledConnection<'_, RedisConnectionManager> {
        self.pool.get().await.expect("Failed Connection to Redis Server")
    }
}