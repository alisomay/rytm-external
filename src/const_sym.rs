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

pub mod object_sub_type {
    use super::*;

    lazy_static! {
        pub static ref TRACK: SymbolRef = SymbolRef::try_from("track").unwrap();
        pub static ref TRIG: SymbolRef = SymbolRef::try_from("trig").unwrap();
    }
}

pub mod trig_action_type {

    use super::*;

    lazy_static! {
        pub static ref ENABLE: SymbolRef = SymbolRef::try_from("enable").unwrap();
        pub static ref RETRIG: SymbolRef = SymbolRef::try_from("retrig").unwrap();
        pub static ref MUTE: SymbolRef = SymbolRef::try_from("mute").unwrap();
        pub static ref ACCENT: SymbolRef = SymbolRef::try_from("accent").unwrap();
        pub static ref SWING: SymbolRef = SymbolRef::try_from("swing").unwrap();
        pub static ref SLIDE: SymbolRef = SymbolRef::try_from("slide").unwrap();
    }
}
