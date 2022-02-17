use wasmedge_http_req::request;

fn main() {
    let res = request::head("http://doc.rust-lang.org/std/string/index.html").unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("{:?}", res.headers());
}
