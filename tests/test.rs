#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bb8_redis::redis::AsyncCommands;
    use bb8_redis::redis;
    use tokio::time::Instant;
    use RedisCache::Cache;

    #[tokio::test]
    async fn test_redis_connection() {
        let start_time = Instant::now();
        let cache = Cache::new().await;
        
        // Usa AsyncCommands en lugar de query_async directamente
        let mut conn = cache.get_conn().await;
        
        println!("Han pasado: {}ms", start_time.elapsed().as_millis())

    }

    #[tokio::test]
    async fn test_ping() {
        let start_time = Instant::now();
        let cache = Cache::new().await;
        let mut conn = cache.get_conn().await;
        
        // Método correcto para hacer PING
        let reply: String = redis::cmd("PING")
            .query_async(&mut conn as &mut bb8_redis::redis::aio::Connection)
            .await
            .unwrap();
            
        assert_eq!(reply, "PONG");

        println!("Han pasado: {}ms", start_time.elapsed().as_millis())
    }
}