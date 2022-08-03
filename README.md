# wasmedge_http_req

Simple and lightweight HTTP client for the low level [wasmedge_wasi_socket](https://github.com/second-state/wasmedge_wasi_socket) library. It is to be compiled into WebAssembly bytecode targets and run on the [WasmEdge Runtime](https://github.com/WasmEdge/WasmEdge).

> This project is forked and derived from the [http_req](https://github.com/jayjamesjay/http_req) project created by [jayjamesjay](https://github.com/jayjamesjay).

## Example

Basic GET request

```rust
use wasmedge_http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://127.0.0.1/", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
}
```

## How to use:

```toml
[dependencies]
wasmedge_http_req  = "0.8.1"
```

## HTTPS support
The HTTP and HTTPS APIs are the same. The Err messages are presented differently because the HTTP uses the rust code while the HTTPS request uses a wasmedge host function. 

build the wasmedge
```
sudo apt-get install libssl-dev
cmake -DCMAKE_BUILD_TYPE=Release -DWASMEDGE_BUILD_TESTS=OFF -DWASMEDGE_PLUGIN_HTTPSREQ=true  .. && make -j4
```

Basic HTTPS GET request

```rust
use wasmedge_http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("https://127.0.0.1/", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
}
```