use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            func_list: Arc::new(RwLock::new(vec![])),
            tmp: Arc::new(RwLock::new(Tmp::default())),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_cfg().write().await.set_host(host.into());
        self
    }

    pub async fn port(&mut self, port: usize) -> &mut Self {
        self.get_cfg().write().await.set_port(port);
        self
    }

    pub async fn log_dir<T>(&mut self, log_dir: T) -> &mut Self
    where
        T: Into<String> + Clone,
    {
        self.get_cfg()
            .write()
            .await
            .set_log_dir(log_dir.clone().into());
        self.get_tmp()
            .write()
            .await
            .log
            .set_path(log_dir.clone().into());
        self
    }

    pub async fn log_size(&mut self, log_size: usize) -> &mut Self {
        self.get_cfg().write().await.set_log_size(log_size);
        self.get_tmp()
            .write()
            .await
            .log
            .set_limit_file_size(log_size);
        self
    }

    pub async fn enable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DEFAULT_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DEFAULT_LOG_FILE_SIZE);
        self
    }

    pub async fn disable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DISABLE_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DISABLE_LOG_FILE_SIZE);
        self
    }

    pub async fn print(&mut self, print: bool) -> &mut Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn enable_print(&mut self) -> &mut Self {
        self.print(true).await;
        self
    }

    pub async fn disable_print(&mut self) -> &mut Self {
        self.print(false).await;
        self
    }

    pub async fn open_print(&mut self, print: bool) -> &mut Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.get_cfg().write().await.set_buffer_size(buffer_size);
        self
    }

    pub async fn inner_print(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn inner_log(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_log(print);
        self
    }

    pub async fn enable_inner_print(&self) -> &Self {
        self.inner_print(true).await;
        self
    }

    pub async fn disable_inner_print(&self) -> &Self {
        self.inner_print(false).await;
        self
    }

    pub async fn enable_inner_log(&self) -> &Self {
        self.inner_log(true).await;
        self
    }

    pub async fn disable_inner_log(&self) -> &Self {
        self.inner_log(false).await;
        self
    }

    pub async fn func<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.func_list
            .write()
            .await
            .push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        self
    }

    pub async fn listen(&mut self) {
        self.init().await;
        let cfg: ServerConfig = self.get_cfg().read().await.clone();
        let host: String = cfg.get_host().to_owned();
        let port: usize = *cfg.get_port();
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let socket_res: Result<UdpSocket, ServerError> = UdpSocket::bind(&addr)
            .await
            .map_err(|e| ServerError::TcpBindError(e.to_string()));
        if let Err(err) = socket_res {
            self.get_tmp()
                .read()
                .await
                .get_log()
                .error(err.to_string(), common_log);
            return;
        }
        let socket: ArcRwLockUdpSocket = ArcRwLockUdpSocket::from_socket(socket_res.unwrap());
        loop {
            let mut buf: Vec<u8> = vec![0u8; *cfg.get_buffer_size()];
            let socket: ArcRwLockUdpSocket = socket.clone();
            let socket_lock: RwLockReadGuardUdpSocket = socket.get_read_lock().await;
            let (data_len, client_addr) = socket_lock.recv_from(&mut buf).await.unwrap();
            let tmp_arc_lock: ArcRwLockTmp = Arc::clone(&self.tmp);
            let func_list_arc_lock: ArcRwLockVecFuncBox = Arc::clone(self.get_func_list());
            let socket_clone: ArcRwLockUdpSocket = socket.clone();
            let handle_request = move || async move {
                let request: Vec<u8> = buf[..data_len].to_vec();
                let log: Log = tmp_arc_lock.read().await.get_log().clone();
                let mut controller_data: InnerControllerData = InnerControllerData::new();
                controller_data
                    .set_socket(Some(socket_clone))
                    .set_socket_addr(Some(client_addr))
                    .set_request(request)
                    .set_log(log);
                let controller_data: ControllerData =
                    ControllerData::from_controller_data(controller_data);
                for func in func_list_arc_lock.read().await.iter() {
                    func(controller_data.clone()).await;
                }
            };
            tokio::spawn(handle_request());
        }
    }

    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        let cfg: ServerConfig = self.get_cfg().read().await.clone();
        let enable_inner_print: bool = *cfg.get_inner_print();
        let enable_inner_log: bool = *cfg.get_inner_log() && tmp.get_log().is_enable();
        set_hook(Box::new(move |err| {
            let err_msg: String = format!("{}", err);
            if enable_inner_print {
                println_error!(err_msg);
            }
            if enable_inner_log {
                handle_error(&tmp, err_msg.clone());
            }
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
    }
}
