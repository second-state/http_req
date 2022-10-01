# http_req_wasi

Simple and lightweight HTTP client for the low level [wasmedge_wasi_socket](https://github.com/second-state/wasmedge_wasi_socket) library. 
It is to be compiled into WebAssembly bytecode targets and runs in the [WasmEdge Runtime](https://github.com/WasmEdge/WasmEdge) as a lightweight and secure alternative to natively compiled apps in Linux container.

> This project is forked and derived from the [http_req](https://github.com/jayjamesjay/http_req) project created by [jayjamesjay](https://github.com/jayjamesjay).

## Example

Basic GET request

```rust
use http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://eu.httpbin.org/get?msg=WasmEdge", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
}
```

## How to use:

```toml
[dependencies]
http_req_wasi  = "0.10"
```

## HTTPS support

The HTTP and HTTPS APIs are the same. But you will need to get the [WasmEdge https_req plugin](https://github.com/WasmEdge/WasmEdge/actions/runs/3126746485) and unzip it into the `plugin` directory of your WasmEdge install.

See [examples here](examples).
