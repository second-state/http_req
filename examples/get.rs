use http_req_wasi::request;

fn main() {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://httpbin.org/get", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    //println!("{}", String::from_utf8_lossy(&writer));
}
