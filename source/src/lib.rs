extern crate libc;

mod extractor;

use std::path::Path;
use std::ffi::CStr;

#[no_mangle]
pub extern fn extract_rust(zip: *const libc::c_char, outdir: *const libc::c_char) -> bool
{
    let zippath = Path::new(rust_string(zip));
    let outdirpath = Path::new(rust_string(outdir));

    return match extractor::run(zippath, outdirpath) {
        Ok(_) => true,
        Err(e) => {
            println!("Error: {}", e);
            false
        }
    }
}

fn rust_string(r_string: *const libc::c_char) -> &'static str
{
  unsafe { CStr::from_ptr(r_string) }.to_str().unwrap()
}
