<center>

## udp

[![](https://img.shields.io/crates/v/udp.svg)](https://crates.io/crates/udp)
[![](https://img.shields.io/crates/d/udp.svg)](https://img.shields.io/crates/d/udp.svg)
[![](https://docs.rs/udp/badge.svg)](https://docs.rs/udp)
[![](https://github.com/eastspire/udp/workflows/Rust/badge.svg)](https://github.com/eastspire/udp/actions?query=workflow:Rust)
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

async fn test_func(ctx: Context) {
    ctx.send("udp: 1").await.unwrap();
}

fn error_handle(error: String) {
    eprint!("{}", error);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}

#[tokio::main]
async fn main() {
    let mut server: Server = Server::new();
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
