# http_req_wasi

Simple and lightweight HTTP client for the low level [wasmedge_wasi_socket](https://github.com/second-state/wasmedge_wasi_socket) library. 
It is to be compiled into WebAssembly bytecode targets and runs in the [WasmEdge Runtime](https://github.com/WasmEdge/WasmEdge) as a lightweight and secure alternative to natively compiled apps in Linux container.

> This project is forked and derived from the [http_req](https://github.com/jayjamesjay/http_req) project created by [jayjamesjay](https://github.com/jayjamesjay).

## Example

Basic GET request in HTTPS

```rust
use http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("https://httpbin.org/get?msg=WasmEdge", &mut writer).unwrap();

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

## Build and run

[Install WasmEdge](https://wasmedge.org/book/en/quick_start/install.html) and then install the HTTPS plugin as follows.

```bash
# Download and extract the plugin
wget https://github.com/WasmEdge/WasmEdge/releases/download/0.11.1/WasmEdge-plugin-wasmedge_httpsreq-0.11.1-manylinux2014_x86_64.tar.gz
tar -xzf WasmEdge-plugin-wasmedge_httpsreq-0.11.1-manylinux2014_x86_64.tar.gz

# Install the plugin if your wasmedge is installed in ~/.wasmedge
cp libwasmedgePluginHttpsReq.so ~/.wasmedge/plugin/

# Install the plugin if your wasmedge is installed in /usr/local
cp libwasmedgePluginHttpsReq.so /usr/local/lib/wasmedge/
```

Build the [GET HTTPS](examples/get_https.rs) example.

```bash
cargo wasi build --release --example get_https
```

Run the example.

```bash
wasmedge target/wasm32-wasi/release/examples/get_https.wasm

Status: 200 OK
Headers {
  Content-Length: 292
  Date: Tue, 04 Oct 2022 20:07:47 GMT
  Access-Control-Allow-Origin: *
  Access-Control-Allow-Credentials: true
  Server: gunicorn/19.9.0
  Content-Type: application/json
  Connection: close
}
{
  "args": {
    "msg": "WasmEdge"
  }, 
  "headers": {
    "Host": "httpbin.org", 
    "Referer": "https://httpbin.org/get?msg=WasmEdge", 
    "X-Amzn-Trace-Id": "Root=1-633c9293-390dc4cc46f268412e39a208"
  }, 
  "origin": "13.84.49.116", 
  "url": "https://httpbin.org/get?msg=WasmEdge"
}
```
