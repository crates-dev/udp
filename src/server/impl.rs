use crate::*;

/// Provides a default implementation for ServerData.
impl Default for ServerData {
    /// Creates a new ServerData instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            server_config: ServerConfigData::default(),
            hook: vec![],
            task_panic: vec![],
            read_error: vec![],
        }
    }
}

/// Provides a default implementation for Server.
impl Default for Server {
    /// Creates a new Server instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(ServerData::default()))
    }
}

/// Implements the `PartialEq` trait for `ServerData`.
///
/// This allows for comparing two `ServerData` instances for equality.
impl PartialEq for ServerData {
    /// Checks if two `ServerData` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other `ServerData` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the instances are equal, `false` otherwise.
    fn eq(&self, other: &Self) -> bool {
        self.server_config == other.server_config
            && self.hook.len() == other.hook.len()
            && self.task_panic.len() == other.task_panic.len()
            && self.read_error.len() == other.read_error.len()
    }
}

/// Implements the `Eq` trait for `ServerData`.
///
/// This indicates that `ServerData` has a total equality relation.
impl Eq for ServerData {}

/// Implementation of methods for `ServerData`.
impl ServerData {
    /// Gets the server configuration.
    ///
    /// # Returns
    ///
    /// - `&ServerConfigData` - Reference to the server configuration.
    pub(crate) fn get_server_config(&self) -> &ServerConfigData {
        &self.server_config
    }

    /// Gets a mutable reference to the server configuration.
    ///
    /// # Returns
    ///
    /// - `&mut ServerConfigData` - Mutable reference to the server configuration.
    pub(crate) fn get_mut_server_config(&mut self) -> &mut ServerConfigData {
        &mut self.server_config
    }

    /// Gets the hook list.
    ///
    /// # Returns
    ///
    /// - `&ServerHookList` - Reference to the hook list.
    pub(crate) fn get_hook(&self) -> &ServerHookList {
        &self.hook
    }

    /// Gets a mutable reference to the hook list.
    ///
    /// # Returns
    ///
    /// - `&mut ServerHookList` - Mutable reference to the hook list.
    pub(crate) fn get_mut_hook(&mut self) -> &mut ServerHookList {
        &mut self.hook
    }

    /// Gets a mutable reference to the task panic list.
    ///
    /// # Returns
    ///
    /// - `&mut ServerHookList` - Mutable reference to the task panic list.
    pub(crate) fn get_mut_task_panic(&mut self) -> &mut ServerHookList {
        &mut self.task_panic
    }
}

/// Implements the `PartialEq` trait for `Server`.
///
/// This allows for comparing two `Server` instances for equality.
impl PartialEq for Server {
    /// Checks if two `Server` instances are equal.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The other `Server` instance to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the instances are equal, `false` otherwise.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.0, &other.0) {
            return true;
        }
        if let (Ok(s), Ok(o)) = (self.0.try_read(), other.0.try_read()) {
            *s == *o
        } else {
            false
        }
    }
}

/// Implements the `Eq` trait for `Server`.
///
/// This indicates that `Server` has a total equality relation.
impl Eq for Server {}

/// Represents the server, providing methods to configure and run it.
///
/// This struct wraps the `ServerData` configuration and handler logic,
/// offering a high-level API for setting up the UDP server.
impl Server {
    /// Creates a new Server instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    pub async fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<ServerData>` - The read guard for ServerData.
    async fn read(&self) -> RwLockReadGuard<'_, ServerData> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<ServerData>` - The write guard for ServerData.
    async fn write(&self) -> RwLockWriteGuard<'_, ServerData> {
        self.0.write().await
    }

    /// Sets the server configuration.
    ///
    /// # Arguments
    ///
    /// - `ServerConfig` - The server configuration.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn server_config(&self, config: ServerConfig) -> &Self {
        *self.write().await.get_mut_server_config() = config.get_data().await;
        self
    }

    /// Registers a hook using the `ServerHook` trait.
    ///
    /// # Arguments
    ///
    /// - `ServerHook` - The hook type that implements `ServerHook`.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn hook<H>(&self) -> &Self
    where
        H: ServerHook,
    {
        self.write()
            .await
            .get_mut_hook()
            .push(server_hook_factory::<H>());
        self
    }

    /// Registers a task panic handler using the `ServerHook` trait.
    ///
    /// # Arguments
    ///
    /// - `ServerHook` - The handler type that implements `ServerHook`.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn task_panic<H>(&self) -> &Self
    where
        H: ServerHook,
    {
        self.write()
            .await
            .get_mut_task_panic()
            .push(server_hook_factory::<H>());
        self
    }

    /// Creates and binds a UDP socket based on the server's configuration.
    ///
    /// # Returns
    ///
    /// - `Result<UdpSocket, ServerError>` - A `Result` containing the bound `UdpSocket` on success,
    ///   or a `ServerError` on failure.
    async fn create_udp_socket(&self) -> Result<UdpSocket, ServerError> {
        let config: ServerConfigData = self.read().await.get_server_config().clone();
        let host: &String = config.get_host();
        let port: u16 = config.get_port();
        let addr: String = format!("{host}:{port}");
        UdpSocket::bind(&addr)
            .await
            .map_err(|e| ServerError::UdpBind(e.to_string()))
    }

    /// Spawns a new asynchronous task to handle a single client request.
    ///
    /// # Arguments
    ///
    /// - `HandlerState` - The handler state containing the socket and config.
    /// - `Request` - The received request data.
    /// - `SocketAddr` - The client's socket address.
    async fn spawn_request_handler(
        &self,
        state: HandlerState,
        data: Request,
        client_addr: SocketAddr,
    ) {
        let server: Server = self.clone();
        tokio::spawn(async move {
            server.handle_request(state, data, client_addr).await;
        });
    }

    /// The core request handling pipeline.
    ///
    /// This function orchestrates the execution of handlers.
    ///
    /// # Arguments
    ///
    /// - `HandlerState` - The handler state.
    /// - `Request` - The request data.
    /// - `SocketAddr` - The client's socket address.
    async fn handle_request(&self, state: HandlerState, data: Request, client_addr: SocketAddr) {
        let ctx: Context = Context::new(&state.get_socket(), &data, client_addr);
        for hook in self.read().await.get_hook().iter() {
            hook(&ctx).await;
            if ctx.get_aborted().await {
                return;
            }
        }
    }

    /// Starts the server, binds to the configured address, and begins listening for requests.
    ///
    /// This is the main entry point to launch the server. It will create a UDP socket,
    /// bind to the configured address, and then enter the request acceptance loop.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ServerControlHook` on success.
    /// Returns an error if the server fails to start.
    pub async fn run(&self) -> Result<ServerControlHook, ServerError> {
        let socket: UdpSocket = self.create_udp_socket().await?;
        let socket: ArcRwLockUdpSocket = ArcRwLockUdpSocket::from_socket(socket);
        let server: Server = self.clone();
        let (wait_sender, wait_receiver) = channel(());
        let (shutdown_sender, mut shutdown_receiver) = channel(());
        let server_config: ServerConfigData = self.read().await.get_server_config().clone();
        let buffer_size: usize = server_config.get_buffer_size();
        let accept_requests: JoinHandle<()> = tokio::spawn(async move {
            let mut buf: Vec<u8> = vec![0u8; buffer_size];
            loop {
                match socket.get_read_lock().await.recv_from(&mut buf).await {
                    Ok((data_len, client_addr)) => {
                        let data: Request = buf[..data_len].to_vec();
                        let state: HandlerState = HandlerState::new(socket.clone());
                        server.spawn_request_handler(state, data, client_addr).await;
                    }
                    Err(e) => {
                        eprintln!("UDP receive error: {e}");
                    }
                }
            }
        });
        let wait_hook: SharedAsyncTaskFactory<()> = Arc::new(move || {
            let mut wait_receiver_clone: Receiver<()> = wait_receiver.clone();
            Box::pin(async move {
                let _ = wait_receiver_clone.changed().await;
            })
        });
        let shutdown_sender_arc: Arc<Sender<()>> = Arc::new(shutdown_sender);
        let shutdown_hook: SharedAsyncTaskFactory<()> = Arc::new(move || {
            let shutdown_sender_clone: Arc<Sender<()>> = shutdown_sender_arc.clone();
            Box::pin(async move {
                let _ = shutdown_sender_clone.send(());
            })
        });
        tokio::spawn(async move {
            let _ = shutdown_receiver.changed().await;
            accept_requests.abort();
            let _ = wait_sender.send(());
        });
        let mut server_control_hook: ServerControlHook = ServerControlHook::default();
        server_control_hook.set_shutdown_hook(shutdown_hook);
        server_control_hook.set_wait_hook(wait_hook);
        Ok(server_control_hook)
    }
}
