#[cfg(test)]
mod tests {
    use bb8_redis::redis;
    use tokio::time::Instant;
    use redis_cache::Cache;

    #[tokio::test]
    async fn test_redis_connection() {
        let start_time = Instant::now();
        let cache = Cache::new("redis://localhost".to_string()).await;
        
        // Usa AsyncCommands en lugar de query_async directamente
        let _conn = cache.get_conn().await;
        
        println!("Han pasado: {}ms", start_time.elapsed().as_millis())

    }

    #[tokio::test]
    async fn test_ping() {
        let start_time = Instant::now();
        let cache = Cache::new("redis://localhost".to_string()).await;
        let mut _conn = cache.get_conn().await;
        
        // Método correcto para hacer PING
        let reply: String = redis::cmd("PING")
            .query_async(&mut _conn as &mut bb8_redis::redis::aio::Connection)
            .await
            .unwrap();
            
        assert_eq!(reply, "PONG");

        println!("Han pasado: {}ms", start_time.elapsed().as_millis())
    }
}