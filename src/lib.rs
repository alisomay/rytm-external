pub mod action;
pub mod api;
pub mod class;
pub mod const_sym;
pub mod error;
pub mod rytm;
pub mod traits;
pub mod trampoline;
pub mod types;
pub mod util;

use rytm::Rytm;

// This is the entry point for the Max external
#[no_mangle]
pub unsafe extern "C" fn ext_main(_r: *mut ::std::ffi::c_void) {
    // Register your wrapped class with Max
    if std::panic::catch_unwind(|| Rytm::register()).is_err() {
        std::process::exit(1);
    }
}
