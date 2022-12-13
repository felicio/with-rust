// Source code example

use status_bindings::{CompressPublicKey, HexToNumber};
use std::ffi::{CStr, CString};
/* note?!:
> it's not possible to have two go runtimes running at the same time
>
> – https://discord.com/channels/864066763682218004/1047103093541711892/1052215301766397962
> – https://discord.com/channels/864066763682218004/1047103093541711892/1052274810438303794

> We have 2 independent go library and If I bind two libraries into two different xcframework, It causes a crash.
>
> – https://github.com/golang/go/issues/50594#issuecomment-1230146376

system info:
macOS 12.4 Darwin 21.5.0 arm64

error when calling HexToNumber():
   Compiling status-bindings v0.1.0 (https://github.com/felicio/status-rust-bindings.git?rev=3b45688#3b45688a)
   Compiling with-rust v0.1.0 (/Users/felicio/Playground/with-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 21.46s
     Running `target/debug/with-rust`
minpc= 0x10093a960 min= 0x10093a960 maxpc= 0x10122b770 max= 0x10122b8c0
fatal error: minpc or maxpc invalid
runtime: panic before malloc heap initialized

runtime stack:
runtime.throw({0x102aab8f9?, 0x0?})
        /opt/homebrew/Cellar/go/1.18.3/libexec/src/runtime/panic.go:992 +0x50 fp=0x16f5dae60 sp=0x16f5dae30 pc=0x100970d50
runtime.moduledataverify1(0x105706b60)
        /opt/homebrew/Cellar/go/1.18.3/libexec/src/runtime/symtab.go:635 +0x41c fp=0x16f5daf40 sp=0x16f5dae60 pc=0x10098d3fc
runtime.moduledataverify(...)
        /opt/homebrew/Cellar/go/1.18.3/libexec/src/runtime/symtab.go:592
runtime.schedinit()
        /opt/homebrew/Cellar/go/1.18.3/libexec/src/runtime/proc.go:689 +0x44 fp=0x16f5dafa0 sp=0x16f5daf40 pc=0x1009743f4
runtime.rt0_go()
        /opt/homebrew/Cellar/go/1.18.3/libexec/src/runtime/asm_arm64.s:86 +0xd0 fp=0x16f5dafd0 sp=0x16f5dafa0 pc=0x10099e1a0
*/
// use waku_bindings::ProtocolId;

fn main() {
    // let _pi = ProtocolId::Store.to_string(); // "/vac/waku/store"

    let _number = unsafe {
        CStr::from_ptr(HexToNumber(
            CString::new(String::from("f"))
                .expect("CString::new failed")
                .into_raw(),
        ))
    }
    .to_str(); // "15"

    let _cpk = unsafe {
        CStr::from_ptr(CompressPublicKey(
            CString::new(String::from(
                "0x049f196bbfef4fa6a5eb81dd802133a63498325445ca1af1d154b1bb454295513305b23fcf11d005ee622144fc402b713a8928f80d705781e2e78d701c6e01bfc4"
            ))
            .expect("CString::new failed")
            .into_raw(),
        ))
    }
    .to_str(); // "0x029f196bbfef4fa6a5eb81dd802133a63498325445ca1af1d154b1bb4542955133"

    println!("end");
}
