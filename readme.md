<center>

## udp

[![](https://img.shields.io/crates/v/udp.svg)](https://crates.io/crates/udp)
[![](https://img.shields.io/crates/d/udp.svg)](https://img.shields.io/crates/d/udp.svg)
[![](https://docs.rs/udp/badge.svg)](https://docs.rs/udp)
[![](https://github.com/ltpp-universe/udp/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/udp/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/udp.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/udp/)

[Api Docs](https://docs.rs/udp/latest/udp/)

> A lightweight and efficient Rust library for building UDP servers with request-response handling.

## Installation

To use this crate, you can run cmd:

```shell
cargo add udp
```

## Use

```rust
use udp::*;

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
