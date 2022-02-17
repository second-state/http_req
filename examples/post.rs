use wasmedge_http_req::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    const BODY: &[u8; 42] = b"{\"field1\" : \"value1\", \"field2\" : \"value2\"}";
    // dns is supported
    // tls is not supported
    // Use tests/server.py for testing
    let res = request::post("http://localhost:1234/post", BODY, &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
}
