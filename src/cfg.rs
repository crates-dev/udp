#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_func(controller_data: ControllerData) {
        controller_data.send("Udp").await.unwrap();
        let response: Response = controller_data.get_response().await;
        let response_data: &ResponseData = response.get_response_data();
        controller_data
            .log_debug(
                format!(
                    "Response => {:?}\n",
                    String::from_utf8_lossy(&response_data)
                ),
                log_debug_format_handler,
            )
            .await;
    }

    async fn main() {
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
                println_success!(String::from_utf8_lossy(&data.get_request().await));
            }))
            .await;
        server.listen().await;
    }

    recoverable_spawn::r#async::recoverable_spawn(main);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
