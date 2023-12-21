use median::{
    atom::{Atom, AtomValue},
    max_sys,
    outlet::SendValue,
    symbol::SymbolRef,
};

use crate::RYTM_EXTERNAL_DEBUG;

// TODO: Revise debugging with right files maybe in a macro.
// This is a WIP for now.
pub trait DebugPost {
    fn dbg_post(&self);
}

impl DebugPost for &str {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for String {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for &String {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for SymbolRef {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for &SymbolRef {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                median::post!("Debug Symbol [{}:{}]: {self}", std::file!(), std::line!());
            }
        }
    }
}

impl DebugPost for AtomValue {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                match self {
                    AtomValue::Float(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Float: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    AtomValue::Int(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Int: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    AtomValue::Symbol(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Symbol: {}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                    AtomValue::Object(value) => {
                        median::post!(
                            "Debug Atom [{}:{}]: Object Pointer: {:?}",
                            std::file!(),
                            std::line!(),
                            value
                        );
                    }
                }
            }
        }
    }
}

impl DebugPost for Atom {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                match self.get_value() {
                    Some(value) => value.dbg_post(),
                    None => {
                        median::post!("Debug Atom [{}:{}]: None", std::file!(), std::line!());
                    }
                }
            }
        }
    }
}

impl DebugPost for &Atom {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                match self.get_value() {
                    Some(value) => value.dbg_post(),
                    None => {
                        median::post!("Debug Atom [{}:{}]: None", std::file!(), std::line!());
                    }
                }
            }
        }
    }
}

impl DebugPost for &[Atom] {
    fn dbg_post(&self) {
        unsafe {
            if RYTM_EXTERNAL_DEBUG {
                if self.is_empty() {
                    median::post!("Debug Atoms [{}:{}]: []", std::file!(), std::line!());
                } else {
                    median::post!(
                        "Debug List of Atoms Start [{}:{}]",
                        std::file!(),
                        std::line!()
                    );
                    for atom in self.iter() {
                        atom.dbg_post();
                    }
                    median::post!("Debug List of Atoms End");
                }
            }
        }
    }
}

pub trait Post {
    fn obj_post(&self, obj: *mut max_sys::t_object);
    fn obj_error(&self, obj: *mut max_sys::t_object);
    fn post(&self);
    fn error(&self);
}

impl Post for &str {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for &String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for SymbolRef {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn post(&self) {
        median::post(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }

    fn error(&self) {
        median::error(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }
}

pub trait SerialSend {
    fn serial_send_int(&self, out: &Box<dyn SendValue<isize> + Sync>);
}

impl SerialSend for Vec<u8> {
    fn serial_send_int(&self, outlet: &Box<dyn SendValue<isize> + Sync>) {
        for byte in self {
            // This is what I could think of :) Since it needs a long :).
            match outlet.send(max_sys::t_atom_long::from(*byte)) {
                Ok(_) => {}
                Err(_stack_overflow_err) => {
                    // Let's just ignore this for now.
                    // Since when there is a stack overflow max crashes anyway.
                }
            }
        }
    }
}

// use std::fmt::Debug;

// use median::atom::{Atom, AtomValue};

// // Define a trait for our custom debug behavior
// trait ExternalDebug {
//     fn external_dbg(&self);
// }

// // Specific implementations for certain types
// impl ExternalDebug for Atom {
//     fn external_dbg(&self) {
//         match self.get_value() {
//             AtomValue::Float(value) => println!("Float: {}", value),
//             //..
//         }
//     }
// }
