use http_req::request;
use wasmedge_wasi_socket::ToSocketAddrs;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    const BODY: &[u8; 27] = b"field1=value1&field2=value2";
    let ip_addr = ("eu.httpbin.org", 80)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    println!("connect {ip_addr}");
    let res = request::post(format!("http://{}/post", ip_addr), BODY, &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    println!("{}", String::from_utf8_lossy(&writer));
}
