use std::ffi::CString;

pub struct Output {
    // the received data
    pub rcv_vec: Vec<u8>,
}

pub mod sslwrapper {
    use std::os::raw::c_char;
    #[link(wasm_import_module = "httpsreq")]
    extern "C" {
        pub fn send_data(
            host: *const c_char,
            hostlen: u32,
            port: u32,
            body: *const c_char,
            bodylen: u32,
        );
        pub fn get_rcv(Rcv: *mut u8);
        pub fn get_rcv_len() -> u32;
    }
}

pub fn send_data<S: AsRef<str>>(host: S, port: u32, body: S) {
    let host = CString::new((host.as_ref()).as_bytes()).expect("");
    let body = CString::new((body.as_ref()).as_bytes()).expect("");
    unsafe {
        sslwrapper::send_data(
            host.as_ptr(),
            host.as_bytes().len() as u32,
            port,
            body.as_ptr(),
            body.as_bytes().len() as u32,
        );
    }
}

pub fn get_receive() -> Output {
    let rcv_len: u32;
    unsafe {
        rcv_len = sslwrapper::get_rcv_len();
    }
    let mut rcv: Vec<u8> = vec![0; rcv_len as usize];
    let rev_ptr = rcv.as_mut_ptr();
    unsafe {
        sslwrapper::get_rcv(rev_ptr);
    }
    Output { rcv_vec: rcv }
}
