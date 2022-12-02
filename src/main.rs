// Source code example

use status_bindings::{CompressPublicKey, BIG_ENDIAN};
use std::os::raw::c_char;
use waku_bindings::ProtocolId;

fn main() {
    let _be = BIG_ENDIAN;
    let _pi = ProtocolId::Store.to_string();

    let pk = "0x029f196bbfef4fa6a5eb81dd802133a63498325445ca1af1d154b1bb4542955133".as_ptr()
        as *mut c_char;
    let _cpk = unsafe { CompressPublicKey(pk) };

    println!("end");
}
