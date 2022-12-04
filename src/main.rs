// Source code example

use status_bindings::HexToNumber;
// use status_bindings::{CompressPublicKey, HexToNumber, IsAddress, BIG_ENDIAN};
use std::os::raw::c_char;
use waku_bindings::ProtocolId;

fn main() {
    // let _be = BIG_ENDIAN;
    let _pi = ProtocolId::Store.to_string();

    // fe70104261c55675e55ff25edb50b345cfb3a3f35f60712d251cbaaab97bd50054c6ebc3cd4e22200c68daf7493e1f8da6a190a68a671e2d3977809612424c7c3888bc6
    // 029f196bbfef4fa6a5eb81dd802133a63498325445ca1af1d154b1bb4542955133
    // 0x029f196bbfef4fa6a5eb81dd802133a63498325445ca1af1d154b1bb4542955133
    // let pk = "fe70104261c55675e55ff25edb50b345cfb3a3f35f60712d251cbaaab97bd50054c6ebc3cd4e22200c68daf7493e1f8da6a190a68a671e2d3977809612424c7c3888bc6";
    // let pk_ptr = pk.as_ptr() as *mut c_char;
    let hex = "9".to_string().as_mut_ptr() as *mut c_char;

    // let _cpk = unsafe { CompressPublicKey(pk_ptr) };
    // let _r = unsafe { IsAddress(pk) };
    let _n = unsafe { HexToNumber(hex) };

    println!("end");
}
