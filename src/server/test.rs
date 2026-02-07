use crate::*;

#[derive(Clone)]
struct EchoHandler;

impl ServerHook for EchoHandler {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: Request = ctx.get_request().await;
        let response: String = format!("Echo: {request:?}");
        let _ = ctx.send(response).await;
    }
}

#[derive(Clone)]
struct PanicHandler;

impl ServerHook for PanicHandler {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, _ctx: &Context) {
        println!("A task panic occurred");
    }
}

#[tokio::test]
async fn test_server_with_struct_handler() {
    let server_config: ServerConfig = ServerConfig::new().await;
    server_config.host("127.0.0.1").await;
    server_config.port(0).await;
    server_config.buffer_size(65535).await;
    let server: Server = Server::new().await;
    server.server_config(server_config).await;
    server.hook::<EchoHandler>().await;
    server.task_panic::<PanicHandler>().await;
}

#[tokio::test]
async fn test_server_config() {
    let config: ServerConfig = ServerConfig::new().await;
    config.host("127.0.0.1").await;
    config.port(8080).await;
    config.buffer_size(1024).await;
    let data: ServerConfigData = config.get_data().await;
    assert_eq!(data.get_host(), "127.0.0.1");
    assert_eq!(data.get_port(), 8080);
    assert_eq!(data.get_buffer_size(), 1024);
}

#[tokio::test]
async fn test_context_creation() {
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let socket: ArcRwLockUdpSocket = ArcRwLockUdpSocket::from_socket(socket);
    let request: Request = vec![1, 2, 3, 4];
    let client_addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 12345));
    let ctx: Context = Context::new(&socket, &request, client_addr);
    assert_eq!(ctx.get_request().await, request);
    assert_eq!(ctx.get_client_addr().await, client_addr);
}

#[tokio::test]
async fn test_server_run_and_shutdown() {
    #[derive(Clone)]
    struct TestHandler;

    impl ServerHook for TestHandler {
        async fn new(_ctx: &Context) -> Self {
            Self
        }

        async fn handle(self, ctx: &Context) {
            let _ = ctx.send("test response").await;
        }
    }

    let server_config: ServerConfig = ServerConfig::new().await;
    server_config.host("127.0.0.1").await;
    server_config.port(60000).await;
    server_config.buffer_size(65535).await;
    let server: Server = Server::new().await;
    server.server_config(server_config).await;
    server.hook::<TestHandler>().await;

    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}
