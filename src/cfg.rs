#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let res: ResponseData = arc_lock_controller_data.send("udp").await.unwrap();
        arc_lock_controller_data
            .log_debug(
                format!("Response => {:?}\n", String::from_utf8_lossy(&res)),
                log_debug_format_handler,
            )
            .await;
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.log_dir("./logs").await;
        server.log_size(100_024_000).await;
        server.buffer(100_024_000).await;
        server.log_interval_millis(360).await;
        server.func(test_func).await;
        let test_string: String = "test".to_owned();
        server
            .func(async_func!(test_string, |data| {
                println_success!(&test_string);
                println_success!(&format!("{:?}", data));
            }))
            .await;
        server.listen().await;
    }
    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
