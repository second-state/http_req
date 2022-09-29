use std::convert::TryFrom;
use http_req_wasi::{request::{Request, HttpVersion}, response::Headers, uri::Uri};

fn main() {
    let mut writer = Vec::new();
    let uri = Uri::try_from("http://eu.httpbin.org/get?msg=WasmEdge").unwrap();
    // let uri = Uri::try_from("https://httpbin.org/get").unwrap(); // uncomment the line for https request

    // add headers to the request
    let mut headers = Headers::new();
    headers.insert("Accept-Charset", "utf-8");
    headers.insert("Accept-Language", "en-US");
    headers.insert("Host", "rust-lang.org");
    headers.insert("Connection", "Close");
    
    let mut response = Request::new(&uri)
        .headers(headers)
        .send(&mut writer)
        .unwrap();

    println!("{}", String::from_utf8_lossy(&writer));

    // set version
    response = Request::new(&uri)
        .version(HttpVersion::Http10)
        .send(&mut writer)
        .unwrap();
    
    println!("{}", String::from_utf8_lossy(&writer));
}
