# leptos_shopify

*⚠️ This package is fresh, untested and under heavy development. It should not be used in production.*

Integration layer for using `shopify_storefront` within [Leptos](https://leptos.dev/) applications.
Currently it only works in SSR, however preparations have been made in the `shopify_storefront` package to work within WASM in the future.


## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
leptos_shopify = "0.1.0"
```

Alternatively, with the CLI:

```
cargo add leptos_shopify
```


## Disclaimer & License

`shopify_storefront` is an unofficial, community-maintained Rust client for the Shopify Storefront API. It is not affiliated with, endorsed by, or supported by Shopify Inc.

This project is dual-licensed under the MIT License and Apache License 2.0. See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.
