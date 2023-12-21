use crate::error::{GetError, SendError, SetError};
use crate::{
    error::{QueryError, RytmExternalError},
    traits::*,
};
use median::atom::AtomValue;
use median::outlet::OutAnything;
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
    pub query_out: OutAnything,
}

// The main trait for your object
impl median::wrapper::ObjWrapped<Self> for Rytm {
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
    const SELECTOR_DEBUG: &'static str = "debug";

    fn debug_mode(_sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        if let Some(atom) = atoms.get(0) {
            if let Some(AtomValue::Int(value)) = atom.get_value() {
                // Check lib.rs for safety.
                // In addition debug post should be never used in this function.
                unsafe {
                    if value == 1 {
                        crate::RYTM_EXTERNAL_DEBUG = true;
                        return Ok(());
                    } else if value == 0 {
                        crate::RYTM_EXTERNAL_DEBUG = false;
                        return Ok(());
                    }
                    return Err(RytmExternalError::from(
                        "Invalid value: Only 0 or 1 are allowed for setting the debug mode.",
                    ));
                }
            }
            return Err(RytmExternalError::from(
                "Invalid value: Only 0 or 1 are allowed for setting the debug mode.",
            ));
        }
        Err(RytmExternalError::from(
            "Invalid format: 0 or 1 should follow the debug keyword.",
        ))
    }

    /// Utility to register your wrapped class with Max
    pub(crate) unsafe fn register() {
        median::wrapper::MaxObjWrapper::<Self>::register(false);
    }

    pub fn int(&self, value: t_atom_long) -> Result<(), RytmExternalError> {
        // The sysexin object sends the data serially.
        // We need to buffer it until we get the end of the sysex message.
        let _inlet_index = median::inlet::Proxy::get_inlet(self.max_obj());

        if value == 0xF0 || self.buffering_sysex.load(Relaxed) {
            self.buffering_sysex.store(true, Relaxed);
            let mut sysex_in_buffer = self.sysex_in_buffer.lock().unwrap();
            sysex_in_buffer.push(value as u8);
            if value == 0xF7 {
                self.buffering_sysex.store(false, Relaxed);

                self.project
                    .lock()
                    .unwrap()
                    .update_from_sysex_response(&sysex_in_buffer)
                    .map_err(RytmExternalError::from)?;

                sysex_in_buffer.clear();
            }
            return Ok(());
        }

        Err(RytmExternalError::from(
            "Invalid input: rytm only understands sysex messages. Please connect sysexin object to the rytm inlet to make sure you pass in only sysex messages.",
        ))
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
            Self::SELECTOR_DEBUG => Self::debug_mode(sel, atoms),
            _ => Err(format!("Invalid selector: {selector}. Possible selectors are query, send, set, get, debug.").into()),
        }
    }

    fn query(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_) | None) => Err(QueryError::InvalidFormat),
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
            (None, Some(_) | None) => Err(SendError::InvalidFormat),
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
        let indexable = ObjectTypeSelector::try_from((
            atoms.get(0).ok_or_else(|| {
                SetError::InvalidFormat(
                    "Setter is incomplete. No other elements follow the get call.".to_owned(),
                )
            })?,
            None,
        ))
        .is_err();

        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_) | None) => Err(SetError::InvalidFormat(
                "Setter is incomplete. No other elements follow the get call.".to_owned(),
            )),
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
            ObjectTypeSelector::Kit(index) => crate::api::kit::handle_kit_set(self, atoms, index),
            ObjectTypeSelector::KitWorkBuffer => crate::api::kit_wb::handle_kit_wb_set(self, atoms),
            ObjectTypeSelector::Sound(index) => {
                crate::api::sound::handle_sound_set(self, atoms, index)
            }
            ObjectTypeSelector::SoundWorkBuffer(index) => {
                crate::api::sound_wb::handle_sound_wb_set(self, atoms, index)
            }
            ObjectTypeSelector::Global(index) => {
                crate::api::global::handle_global_set(self, atoms, index)
            }
            ObjectTypeSelector::GlobalWorkBuffer => {
                crate::api::global_wb::handle_global_wb_set(self, atoms)
            }
            ObjectTypeSelector::Settings => crate::api::settings::handle_settings_set(self, atoms),
        }
    }

    fn get(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        // Indexable objects look for an index as the second atom thus they'd throw an error here.
        let indexable = ObjectTypeSelector::try_from((
            atoms.get(0).ok_or_else(|| {
                GetError::InvalidFormat(
                    "Getter is incomplete. No other elements follow the get call.".to_owned(),
                )
            })?,
            None,
        ))
        .is_err();

        let atom_pair = match (atoms.get(0), atoms.get(1)) {
            (None, Some(_) | None) => Err(GetError::InvalidFormat(
                "Getter is incomplete. No other elements follow the get call.".to_owned(),
            )),
            _ => {
                if indexable {
                    Ok((atoms.get(0).unwrap(), atoms.get(1)))
                } else {
                    Ok((atoms.get(0).unwrap(), None))
                }
            }
        }?;

        match ObjectTypeSelector::try_from(atom_pair)? {
            ObjectTypeSelector::Pattern(index) => {
                crate::api::pattern::handle_pattern_get(self, atoms, index)
            }
            ObjectTypeSelector::PatternWorkBuffer => {
                crate::api::pattern_wb::handle_pattern_wb_get(self, atoms)
            }
            ObjectTypeSelector::Kit(index) => crate::api::kit::handle_kit_set(self, atoms, index),
            ObjectTypeSelector::KitWorkBuffer => crate::api::kit_wb::handle_kit_wb_get(self, atoms),
            ObjectTypeSelector::Sound(index) => {
                crate::api::sound::handle_sound_get(self, atoms, index)
            }
            ObjectTypeSelector::SoundWorkBuffer(index) => {
                crate::api::sound_wb::handle_sound_wb_get(self, atoms, index)
            }
            ObjectTypeSelector::Global(index) => {
                crate::api::global::handle_global_get(self, atoms, index)
            }
            ObjectTypeSelector::GlobalWorkBuffer => {
                crate::api::global_wb::handle_global_wb_get(self, atoms)
            }
            ObjectTypeSelector::Settings => crate::api::settings::handle_settings_get(self, atoms),
        }
    }
}
