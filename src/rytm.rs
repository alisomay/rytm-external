use crate::{
    error::{QueryError, RytmExternalError},
    traits::*,
};
use median::{atom::Atom, max_sys::t_atom_long, object::MaxObj, outlet::OutInt, symbol::SymbolRef};
use rytm_rs::prelude::*;
use std::{
    convert::TryFrom,
    sync::{atomic::AtomicBool, atomic::Ordering::*, Arc, Mutex},
};

use crate::types::ObjectTypeSelector;

// This is the actual object (external)
pub struct Rytm {
    pub project: Arc<Mutex<RytmProject>>,
    pub sysex_in_buffer: Arc<Mutex<Vec<u8>>>,
    pub buffering_sysex: AtomicBool,
    pub sysex_out: OutInt,
}

// The main trait for your object
impl median::wrapper::ObjWrapped<Rytm> for Rytm {
    fn class_name() -> &'static str {
        "rytm"
    }

    // You can modify the object here such as adding assists etc.
}

impl Rytm {
    /// Utility to register your wrapped class with Max
    pub(crate) unsafe fn register() {
        median::wrapper::MaxObjWrapper::<Rytm>::register(false)
    }

    pub fn int(&self, value: t_atom_long) {
        // The sysexin object sends the data serially.
        // We need to buffer it until we get the end of the sysex message.

        let _inlet_index = median::inlet::Proxy::get_inlet(self.max_obj());

        if value == 0xF0 || self.buffering_sysex.load(Relaxed) {
            self.buffering_sysex.store(true, Relaxed);
            let mut sysex_in_buffer = self.sysex_in_buffer.lock().unwrap();
            sysex_in_buffer.push(value as u8);
            if value == 0xF7 {
                self.buffering_sysex.store(false, Relaxed);
                let mut project = self.project.lock().unwrap();
                match project.update_from_sysex_response(&sysex_in_buffer) {
                    Ok(_) => {
                        // Happy path.
                    }
                    Err(err) => RytmExternalError::from(err).obj_post(self.max_obj()),
                }
                sysex_in_buffer.clear();
            }
        }
    }

    pub fn query(&self, _sel: &SymbolRef, atoms: &[Atom]) {
        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => {
                RytmExternalError::from(QueryError::InvalidFormat).obj_post(self.max_obj());
                None
            }
            _ => Some((atoms.get(0).unwrap(), atoms.get(1))),
        };

        if let Some(atom_pair) = atom_pair {
            match ObjectTypeSelector::try_from(atom_pair) {
                Ok(q_type) => {
                    let query_sysex = match q_type {
                        ObjectTypeSelector::Pattern(index) => {
                            PatternQuery::new(index).unwrap().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::PatternWorkBuffer => {
                            PatternQuery::new_targeting_work_buffer()
                                .as_sysex()
                                .unwrap()
                        }
                        ObjectTypeSelector::Kit(index) => {
                            KitQuery::new(index).unwrap().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::KitWorkBuffer => {
                            KitQuery::new_targeting_work_buffer().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::Sound(index) => {
                            SoundQuery::new(index).unwrap().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::SoundWorkBuffer(index) => {
                            SoundQuery::new_targeting_work_buffer(index)
                                .unwrap()
                                .as_sysex()
                                .unwrap()
                        }
                        ObjectTypeSelector::Global(index) => {
                            GlobalQuery::new(index).unwrap().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::GlobalWorkBuffer => {
                            GlobalQuery::new_targeting_work_buffer().as_sysex().unwrap()
                        }
                        ObjectTypeSelector::Settings => SettingsQuery::new().as_sysex().unwrap(),
                    };

                    query_sysex.serial_send_int(&self.sysex_out);
                }
                Err(msg) => msg.obj_post(self.max_obj()),
            }
        }
    }

    pub fn send(&self, _sel: &SymbolRef, atoms: &[Atom]) {
        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => {
                RytmExternalError::from(QueryError::InvalidFormat).obj_post(self.max_obj());
                None
            }
            _ => Some((atoms.get(0).unwrap(), atoms.get(1))),
        };

        if let Some(atom_pair) = atom_pair {
            match ObjectTypeSelector::try_from(atom_pair) {
                Ok(obj_type) => {
                    let obj_sysex = match obj_type {
                        ObjectTypeSelector::Pattern(index) => {
                            self.project.lock().unwrap().patterns()[index]
                                .as_sysex()
                                .unwrap()
                        }
                        ObjectTypeSelector::PatternWorkBuffer => self
                            .project
                            .lock()
                            .unwrap()
                            .work_buffer()
                            .pattern()
                            .as_sysex()
                            .unwrap(),
                        ObjectTypeSelector::Kit(index) => self.project.lock().unwrap().kits()
                            [index]
                            .as_sysex()
                            .unwrap(),
                        ObjectTypeSelector::KitWorkBuffer => self
                            .project
                            .lock()
                            .unwrap()
                            .work_buffer()
                            .kit()
                            .as_sysex()
                            .unwrap(),
                        ObjectTypeSelector::Sound(index) => {
                            self.project.lock().unwrap().pool_sounds()[index]
                                .as_sysex()
                                .unwrap()
                        }
                        ObjectTypeSelector::SoundWorkBuffer(index) => {
                            self.project.lock().unwrap().work_buffer().sounds()[index]
                                .as_sysex()
                                .unwrap()
                        }
                        ObjectTypeSelector::Global(index) => self.project.lock().unwrap().globals()
                            [index]
                            .as_sysex()
                            .unwrap(),
                        ObjectTypeSelector::GlobalWorkBuffer => self
                            .project
                            .lock()
                            .unwrap()
                            .work_buffer()
                            .global()
                            .as_sysex()
                            .unwrap(),
                        ObjectTypeSelector::Settings => {
                            self.project.lock().unwrap().settings().as_sysex().unwrap()
                        }
                    };

                    obj_sysex.serial_send_int(&self.sysex_out);
                }
                Err(msg) => msg.obj_post(self.max_obj()),
            }
        }
    }
}
