use http_req::{
    request::{HttpVersion, Request},
    response::Headers,
    uri::Uri,
};
use std::convert::TryFrom;

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

    Request::new(&uri)
        .headers(headers)
        .send(&mut writer)
        .unwrap();

    println!("{}", String::from_utf8_lossy(&writer));

    // set version
    Request::new(&uri)
        .version(HttpVersion::Http10)
        .send(&mut writer)
        .unwrap();

    println!("{}", String::from_utf8_lossy(&writer));
}
