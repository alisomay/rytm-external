// Currently for the initial version we're working in a relatively relaxed way, later on we may want to be more strict.
// When the stabilization increases.
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::similar_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::enum_glob_use,
    clippy::missing_safety_doc,
    clippy::significant_drop_tightening,
    clippy::too_many_lines
)]
#![allow(clippy::must_use_candidate)]

pub mod action;
pub mod api;
pub mod class;
pub mod error;
pub mod rytm;
pub mod traits;
pub mod trampoline;
pub mod types;
pub mod util;

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
