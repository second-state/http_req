use http_req::request;

fn main() {
    let res = request::head("http://eu.httpbin.org/headers").unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("{:?}", res.headers());
}
