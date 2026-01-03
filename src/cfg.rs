#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_func(ctx: Context) {
        ctx.send("udp: 1").await.unwrap();
    }

    fn error_handle(error: String) {
        eprintln!("{error}");
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }

    async fn main() {
        let mut server: Server = Server::new().await;
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.buffer(100_024_000).await;
        server.error_handle(error_handle).await;
        server.func(test_func).await;
        server
            .func(|ctx: Context| async move {
                ctx.send("udp: 2").await.unwrap();
            })
            .await;
        server.run().await;
    }

    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), main()).await;
}
