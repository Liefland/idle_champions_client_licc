# licc

Liefland Idle Champions Codes - https://crates.io/crates/licc

[![Build Status](https://github.com/liefland/idle_champions_client_licc/actions/workflows/rust.yml/badge.svg)](https://github.com/liefland/idle_champions_client_licc/actions)
[![Docs.rs](https://docs.rs/licc/badge.svg)](https://docs.rs/licc/latest/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README#license)

Simple Async HTTP client that helps you obtain codes that can be redeemed for [Idle Champions of the Forgotten Realms](https://www.idlechampions.com/)

This interfaces with [idle_champions_codes_api](https://github.com/Liefland/idle_champions_codes_api) hosted repositories, of which
the official one maintained by Liefland is hosted at [codes.idlechampions.liefland.net](https://codes.idlechampions.liefland.net/)

All repositories we maintain: [GitHub](https://github.com/Liefland?q=idle_champions)

## Installation

Add as a dependency: 

- `cargo add licc`
- `cargo add licc --features="write"` 
  - Enables write operations of the API 
    This functionality will only be helpful to you if you have an API Key.

## Examples

```rust
use licc::{Code, client::{CodesClient, ClientError}};

async fn list_codes() -> Result<(), ClientError> {
    let client = CodesClient::default();

    let response: Vec<Code> = client.get_codes().await?;

    response.for_each(|code| println!("{}", code.code));
    
    Ok(())
}
```

For more examples, see the `examples/` directory.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.
