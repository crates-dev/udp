use crate::*;

/// Default implementation for Server.
impl Default for Server {
    /// Creates a default server instance with empty configuration.
    ///
    /// # Returns
    ///
    /// - `Server` - New server instance with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(ServerConfig::default())),
            func_list: Arc::new(RwLock::new(vec![])),
        }
    }
}

/// Implementation of Server methods.
///
/// Provides server configuration and runtime operations.
impl Server {
    /// Creates a new server instance.
    ///
    /// # Returns
    ///
    /// - `Server` - New server instance.
    pub async fn new() -> Self {
        Self::default()
    }

    /// Sets the server host address.
    ///
    /// # Arguments
    ///
    /// - `T` - Host address convertible to String.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for chaining.
    pub async fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.config.write().await.host = host.into();
        self
    }

    /// Sets the server port number.
    ///
    /// # Arguments
    ///
    /// - `usize` - Port number.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for chaining.
    pub async fn port(&mut self, port: usize) -> &mut Self {
        self.config.write().await.port = port;
        self
    }

    /// Sets the buffer size for incoming packets.
    ///
    /// # Arguments
    ///
    /// - `usize` - Buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for chaining.
    pub async fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.config.write().await.buffer_size = buffer_size;
        self
    }

    /// Sets the error handler function.
    ///
    /// # Arguments
    ///
    /// - `F` - Error handler function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for chaining.
    pub async fn error_handle<F>(&self, func: F) -> &Self
    where
        F: ErrorHandle + Send + Sync + 'static,
    {
        self.config.write().await.error_handle = Arc::new(func);
        self
    }

    /// Registers a handler function.
    ///
    /// # Arguments
    ///
    /// - `F` - Async handler function.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for chaining.
    pub async fn func<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.func_list
            .write()
            .await
            .push(Box::new(move |ctx| Box::pin(func(ctx))));
        self
    }

    /// Starts the server and begins processing requests.
    pub async fn run(&mut self) {
        self.init().await;
        let config: ServerConfig = self.config.read().await.clone();
        let host: String = config.host.clone();
        let port: usize = config.port;
        let addr: String = format!("{host}{COLON_SPACE_SYMBOL}{port}");
        let socket: UdpSocket = UdpSocket::bind(&addr)
            .await
            .map_err(|e| ServerError::TcpBindError(e.to_string()))
            .unwrap();
        let socket_arc_lock: ArcRwLockUdpSocket = ArcRwLockUdpSocket::from_socket(socket);
        loop {
            let mut buf: Vec<u8> = vec![0u8; config.buffer_size];
            let socket_arc_lock: ArcRwLockUdpSocket = socket_arc_lock.clone();
            let socket_lock: RwLockReadGuardUdpSocket = socket_arc_lock.get_read_lock().await;
            let (data_len, client_addr) = socket_lock.recv_from(&mut buf).await.unwrap();
            let func_list_arc_lock: ArcRwLockVecFuncBox = Arc::clone(&self.func_list);
            let socket_clone: ArcRwLockUdpSocket = socket_arc_lock.clone();
            let handle_request = move || async move {
                let request: Vec<u8> = buf[..data_len].to_vec();
                let mut ctx: InnerContext = InnerContext::new();
                ctx.socket = Some(socket_clone);
                ctx.socket_addr = Some(client_addr);
                ctx.request = request;
                let ctx: Context = Context::from_inner_context(ctx);
                for func in func_list_arc_lock.read().await.iter() {
                    func(ctx.clone()).await;
                }
            };
            tokio::spawn(handle_request());
        }
    }

    /// Initializes the panic hook with error handler.
    async fn init_panic_hook(&self) {
        let error_handle: ArcErrorHandle = self.config.read().await.error_handle.clone();
        set_hook(Box::new(move |err| {
            let data: String = err.to_string();
            error_handle(data);
        }));
    }

    /// Initializes server components.
    async fn init(&self) {
        self.init_panic_hook().await;
    }
}
