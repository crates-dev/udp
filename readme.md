<center>

## upd

[![](https://img.shields.io/crates/v/upd.svg)](https://crates.io/crates/upd)
[![](https://docs.rs/upd/badge.svg)](https://docs.rs/upd)
[![](https://github.com/ltpp-universe/upd/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/upd/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/upd.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/upd/)

[Api Docs](https://docs.rs/upd/latest/upd/)

> A lightweight and efficient Rust library for building UDP servers with request-response handling.

## Installation

To use this crate, you can run cmd:

```shell
cargo add upd
```

## Use

```rust
use upd::*;

async fn test_func(arc_lock_controller_data: ArcRwLockControllerData) {
    let res: ResponseData = arc_lock_controller_data.send("tcplane").await.unwrap();
    arc_lock_controller_data
        .get_controller_data()
        .await
        .get_log()
        .debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res)),
            log_debug_format_handler,
        );
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
