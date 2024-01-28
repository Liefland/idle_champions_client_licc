# licc

Liefland Idle Champions Codes

[![Build Status](https://github.com/zarthus/codes_idlechampions_client/actions/workflows/rust.yml/badge.svg)](https://github.com/zarthus/codes_idlechampions_client/actions)
[![Docs.rs](https://docs.rs/codes_idlechampions_client/badge.svg)](https://docs.rs/codes_idlechampions_client/latest/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README#license)

Simple HTTP that helps you obtain codes that can be redeemed for [Idle Champions of the Forgotten Realms](https://www.idlechampions.com/)

This interfaces with [idle_champions_codes_api](https://github.com/Liefland/idle_champions_codes_api) hosted repositories, of which
the official one maintained by Liefland is hosted at [codes.idlechampions.liefland.net](https://codes.idlechampions.liefland.net/)

All repositories we maintain: [GitHub](https://github.com/Liefland?q=idle_champions)

## Installation

Add as a dependency: 

- `cargo add licc --features="readonly"`

We recommend enabling the `readonly` feature, as only very few people have access to the write API.

## Examples

```rust
use licc::client::{CodesClient, ClientError};
use licc::Code;

async fn list_codes() -> Result<(), ClientError> {
    let client = CodesClient::default();

    let response: Vec<Code> = client.get_codes().await?;

    response.map(|code| println!("{}", code.code));
    
    Ok(())
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.
