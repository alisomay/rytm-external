use crate::rytm::QueryError::InvalidFormat;
use crate::{
    error::{QueryError, RytmExternalError},
    traits::*,
};
use median::outlet::OutAnything;
use median::{
    atom::{Atom, AtomValue},
    max_sys::t_atom_long,
    object::MaxObj,
    outlet::OutInt,
    symbol::SymbolRef,
};
use rytm_rs::{
    object::{
        pattern::{track::Track, Trig},
        Pattern,
    },
    prelude::*,
};
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
    pub query_out: OutAnything,
}

// The main trait for your object
impl median::wrapper::ObjWrapped<Rytm> for Rytm {
    fn class_name() -> &'static str {
        "rytm"
    }

    // You can modify the object here such as adding assists etc.
    // TODO: Maybe add notification handling.
}

impl Rytm {
    const SELECTOR_QUERY: &'static str = "query";
    const SELECTOR_SEND: &'static str = "send";
    const SELECTOR_SET: &'static str = "set";
    const SELECTOR_GET: &'static str = "get";

    /// Utility to register your wrapped class with Max
    pub(crate) unsafe fn register() {
        median::wrapper::MaxObjWrapper::<Rytm>::register(false)
    }

    pub fn int(&self, value: t_atom_long) -> Result<(), RytmExternalError> {
        // The sysexin object sends the data serially.
        // We need to buffer it until we get the end of the sysex message.
        let _inlet_index = median::inlet::Proxy::get_inlet(self.max_obj());

        // TODO: Throw error if not a sysex message..

        if value == 0xF0 || self.buffering_sysex.load(Relaxed) {
            self.buffering_sysex.store(true, Relaxed);
            let mut sysex_in_buffer = self.sysex_in_buffer.lock().unwrap();
            sysex_in_buffer.push(value as u8);
            if value == 0xF7 {
                self.buffering_sysex.store(false, Relaxed);
                let mut project = self.project.lock().unwrap();
                project
                    .update_from_sysex_response(&sysex_in_buffer)
                    .map_err(RytmExternalError::from)?;
                sysex_in_buffer.clear();
            }
        }
        Ok(())
    }

    pub fn anything_with_selector(
        &self,
        sel: &SymbolRef,
        atoms: &[Atom],
    ) -> Result<(), RytmExternalError> {
        let selector = sel
            .to_string()
            .map_err(|err| RytmExternalError::Custom(err.to_string()))?;

        match selector.as_str() {
            Self::SELECTOR_QUERY => self.query(sel, atoms),
            Self::SELECTOR_SEND => self.send(sel, atoms),
            Self::SELECTOR_SET => self.set(sel, atoms),
            Self::SELECTOR_GET => self.get(sel, atoms),
            _ => Err(format!("rytm does not understand {selector}").into()),
        }
    }

    fn query(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => Err(InvalidFormat),
            _ => Ok((atoms.get(0).unwrap(), atoms.get(1))),
        }?;

        match ObjectTypeSelector::try_from(atom_pair)? {
            ObjectTypeSelector::Pattern(index) => {
                PatternQuery::new(index).unwrap().as_sysex().unwrap()
            }
            ObjectTypeSelector::PatternWorkBuffer => PatternQuery::new_targeting_work_buffer()
                .as_sysex()
                .unwrap(),
            ObjectTypeSelector::Kit(index) => KitQuery::new(index).unwrap().as_sysex().unwrap(),
            ObjectTypeSelector::KitWorkBuffer => {
                KitQuery::new_targeting_work_buffer().as_sysex().unwrap()
            }
            ObjectTypeSelector::Sound(index) => SoundQuery::new(index).unwrap().as_sysex().unwrap(),
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
        }
        .serial_send_int(&self.sysex_out);

        Ok(())
    }

    fn send(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => Err(InvalidFormat),
            _ => Ok((atoms.get(0).unwrap(), atoms.get(1))),
        }?;

        match ObjectTypeSelector::try_from(atom_pair)? {
            ObjectTypeSelector::Pattern(index) => self.project.lock().unwrap().patterns()[index]
                .as_sysex()
                .unwrap(),
            ObjectTypeSelector::PatternWorkBuffer => self
                .project
                .lock()
                .unwrap()
                .work_buffer()
                .pattern()
                .as_sysex()
                .unwrap(),
            ObjectTypeSelector::Kit(index) => self.project.lock().unwrap().kits()[index]
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
            ObjectTypeSelector::Sound(index) => self.project.lock().unwrap().pool_sounds()[index]
                .as_sysex()
                .unwrap(),
            ObjectTypeSelector::SoundWorkBuffer(index) => {
                self.project.lock().unwrap().work_buffer().sounds()[index]
                    .as_sysex()
                    .unwrap()
            }
            ObjectTypeSelector::Global(index) => self.project.lock().unwrap().globals()[index]
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
        }
        .serial_send_int(&self.sysex_out);

        Ok(())
    }

    fn set(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        // Format is set <object type> ...

        // Patterns
        // Format is set pattern <pattern-index> <track-index> <trig-index> <action> <value>
        // Other formats are set pattern <pattern-index> <action> <value>
        // set pattern <pattern-index> <track-index> <action> <value>
        // set pattern <pattern-index> <track-index> <trig-index> <action> <value>

        let first_atom = atoms.get(0).ok_or(InvalidFormat)?;
        let indexable = matches!(
            ObjectTypeSelector::try_from((first_atom, None))?,
            ObjectTypeSelector::Pattern(_)
                | ObjectTypeSelector::Kit(_)
                | ObjectTypeSelector::Sound(_)
                | ObjectTypeSelector::SoundWorkBuffer(_)
                | ObjectTypeSelector::Global(_)
        );

        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => Err(InvalidFormat),
            _ => {
                if indexable {
                    Ok((atoms.get(0).unwrap(), atoms.get(1)))
                } else {
                    Ok((atoms.get(0).unwrap(), None))
                }
            }
        }?;

        match ObjectTypeSelector::try_from(atom_pair)? {
            ObjectTypeSelector::Pattern(pattern_index) => {
                crate::api::pattern::handle_pattern_set(self, atoms, pattern_index)
            }
            ObjectTypeSelector::PatternWorkBuffer => {
                crate::api::pattern_wb::handle_pattern_wb_set(self, atoms)
            }
            ObjectTypeSelector::Kit(index) => todo!(),
            ObjectTypeSelector::KitWorkBuffer => todo!(),
            ObjectTypeSelector::Sound(index) => todo!(),
            ObjectTypeSelector::SoundWorkBuffer(index) => todo!(),
            ObjectTypeSelector::Global(index) => todo!(),
            ObjectTypeSelector::GlobalWorkBuffer => todo!(),
            ObjectTypeSelector::Settings => todo!(),
        }
    }

    fn get(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let first_atom = atoms.get(0).ok_or(InvalidFormat)?;
        let indexable = matches!(
            ObjectTypeSelector::try_from((first_atom, None))?,
            ObjectTypeSelector::Pattern(_)
                | ObjectTypeSelector::Kit(_)
                | ObjectTypeSelector::Sound(_)
                | ObjectTypeSelector::SoundWorkBuffer(_)
                | ObjectTypeSelector::Global(_)
        );

        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_)) | (None, None) => Err(InvalidFormat),
            _ => {
                if indexable {
                    Ok((atoms.get(0).unwrap(), atoms.get(1)))
                } else {
                    Ok((atoms.get(0).unwrap(), None))
                }
            }
        }?;

        match ObjectTypeSelector::try_from(atom_pair)? {
            ObjectTypeSelector::Pattern(pattern_index) => todo!(),
            ObjectTypeSelector::PatternWorkBuffer => {
                crate::api::pattern_wb::handle_pattern_wb_get(self, atoms)
            }
            ObjectTypeSelector::Kit(index) => todo!(),
            ObjectTypeSelector::KitWorkBuffer => todo!(),
            ObjectTypeSelector::Sound(index) => todo!(),
            ObjectTypeSelector::SoundWorkBuffer(index) => todo!(),
            ObjectTypeSelector::Global(index) => todo!(),
            ObjectTypeSelector::GlobalWorkBuffer => todo!(),
            ObjectTypeSelector::Settings => todo!(),
        }
    }
}
