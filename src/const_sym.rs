use crate::traits::*;
use lazy_static::lazy_static;
use median::{atom::Atom, max_sys::t_atom_long, object::MaxObj, outlet::OutInt, symbol::SymbolRef};
use std::convert::TryFrom;

pub mod object_type {
    use super::*;

    lazy_static! {
        pub static ref PATTERN: SymbolRef = SymbolRef::try_from("pattern").unwrap();
        pub static ref PATTERN_WORK_BUFFER: SymbolRef = SymbolRef::try_from("pattern_wb").unwrap();
        pub static ref KIT: SymbolRef = SymbolRef::try_from("kit").unwrap();
        pub static ref KIT_WORK_BUFFER: SymbolRef = SymbolRef::try_from("kit_wb").unwrap();
        pub static ref SOUND: SymbolRef = SymbolRef::try_from("sound").unwrap();
        pub static ref SOUND_WORK_BUFFER: SymbolRef = SymbolRef::try_from("sound_wb").unwrap();
        pub static ref GLOBAL: SymbolRef = SymbolRef::try_from("global").unwrap();
        pub static ref GLOBAL_WORK_BUFFER: SymbolRef = SymbolRef::try_from("global_wb").unwrap();
        pub static ref SETTINGS: SymbolRef = SymbolRef::try_from("settings").unwrap();
    }
}
