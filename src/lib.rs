pub mod action;
pub mod api;
pub mod class;
pub mod error;
pub mod rytm;
pub mod traits;
pub mod trampoline;
pub mod types;
pub mod util;

use std::ffi::CString;

use rytm::Rytm;

// Should be only set through the debug 1 or debug 0 messages.
// Should be only set from one place in the code, no other functions or threads.
// Make sure that no other code is accessing this variable while it is being set.
// Anything other than that is undefined behavior.
pub static mut RYTM_EXTERNAL_DEBUG: bool = false;

// This is the entry point for the Max external
#[no_mangle]
pub unsafe extern "C" fn ext_main(_r: *mut ::std::ffi::c_void) {
    // Register your wrapped class with Max
    if std::panic::catch_unwind(|| Rytm::register()).is_err() {
        std::process::exit(1);
    }
}
