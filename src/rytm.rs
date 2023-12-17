use crate::rytm::QueryError::InvalidFormat;
use crate::{
    const_sym::trig_action_type::TrigActionType,
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
    pub get_out: OutAnything,
}

// The main trait for your object
impl median::wrapper::ObjWrapped<Rytm> for Rytm {
    fn class_name() -> &'static str {
        "rytm"
    }

    // You can modify the object here such as adding assists etc.
}

impl Rytm {
    const SELECTOR_QUERY: &str = "query";
    const SELECTOR_SEND: &str = "send";
    const SELECTOR_SET: &str = "set";
    const SELECTOR_GET: &str = "get";

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

        // We need to inspect atoms slowly.
        // First lets check the pair since they're always the same.

        let first_atom = atoms.get(0).ok_or(InvalidFormat)?;
        // let value = first_atom.get_value().ok_or(InvalidFormat)?;
        // let symbol = match value {
        //     AtomValue::Symbol(symbol) => symbol,
        //     _ => Err(InvalidFormat)?,
        // };

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
                if !(0..=127).contains(&pattern_index) {
                    "Pattern index must be an integer between 0 and 127".obj_error(self.max_obj());
                }
                match try_get_atom_value_assuming_action_or_index(2, atoms)? {
                    AtomValue::Symbol(action) => {
                        handle_set_action(SetAction::Pattern(PatternSetAction {
                            pattern: self
                                .project
                                .lock()
                                .unwrap()
                                .patterns_mut()
                                .get_mut(pattern_index)
                                .unwrap(),
                            action,
                            parameter: try_get_action_value_from_atom_slice(3, atoms)?,
                        }))
                    }
                    AtomValue::Int(track_index) => {
                        if !(0..=12).contains(&track_index) {
                            "Track index must be an integer between 0 and 12"
                                .obj_error(self.max_obj());
                        }
                        match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                            AtomValue::Symbol(action) => {
                                handle_set_action(SetAction::Track(TrackSetAction {
                                    track: &mut self.project.lock().unwrap().patterns_mut()
                                        [pattern_index]
                                        .tracks_mut()
                                        [track_index as usize],
                                    action,
                                    parameter: try_get_action_value_from_atom_slice(4, atoms)?,
                                }))
                            }
                            AtomValue::Int(trig_index) => {
                                if !(0..=63).contains(&track_index) {
                                    "Trig index must be an integer between 0 and 63"
                                        .obj_error(self.max_obj());
                                }
                                match try_get_atom_value_assuming_action_or_index(4, atoms)? {
                                    AtomValue::Symbol(action) => {
                                        handle_set_action(SetAction::Trig(TrigSetAction {
                                            trig: &mut self.project.lock().unwrap().patterns_mut()
                                                [pattern_index]
                                                .tracks_mut()
                                                [track_index as usize]
                                                .trigs_mut()
                                                [trig_index as usize],
                                            action,
                                            parameter: try_get_action_value_from_atom_slice(
                                                5, atoms,
                                            )?,
                                        }))
                                    }
                                    _ => {
                                        Err("Only symbols and integers are allowed in setters."
                                            .into())
                                    }
                                }
                            }
                            _ => Err("Only symbols and integers are allowed in setters.".into()),
                        }
                    }
                    _ => Err("Only symbols and integers are allowed in setters.".into()),
                }
            }

            ObjectTypeSelector::PatternWorkBuffer => {
                match try_get_atom_value_assuming_action_or_index(1, atoms)? {
                    AtomValue::Symbol(action) => {
                        handle_set_action(SetAction::Pattern(PatternSetAction {
                            pattern: self.project.lock().unwrap().work_buffer_mut().pattern_mut(),
                            action,
                            parameter: try_get_action_value_from_atom_slice(2, atoms)?,
                        }))
                    }
                    AtomValue::Int(track_index) => {
                        if !(0..=12).contains(&track_index) {
                            "Track index must be an integer between 0 and 12"
                                .obj_error(self.max_obj());
                        }
                        match try_get_atom_value_assuming_action_or_index(2, atoms)? {
                            AtomValue::Symbol(action) => {
                                handle_set_action(SetAction::Track(TrackSetAction {
                                    track: &mut self
                                        .project
                                        .lock()
                                        .unwrap()
                                        .work_buffer_mut()
                                        .pattern_mut()
                                        .tracks_mut()
                                        [track_index as usize],
                                    action,
                                    parameter: try_get_action_value_from_atom_slice(3, atoms)?,
                                }))
                            }
                            AtomValue::Int(trig_index) => {
                                if !(0..=63).contains(&track_index) {
                                    "Trig index must be an integer between 0 and 63"
                                        .obj_error(self.max_obj());
                                }
                                match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                                    AtomValue::Symbol(action) => {
                                        handle_set_action(SetAction::Trig(TrigSetAction {
                                            trig: &mut self
                                                .project
                                                .lock()
                                                .unwrap()
                                                .work_buffer_mut()
                                                .pattern_mut()
                                                .tracks_mut()
                                                [track_index as usize]
                                                .trigs_mut()
                                                [trig_index as usize],
                                            action,
                                            parameter: try_get_action_value_from_atom_slice(
                                                4, atoms,
                                            )?,
                                        }))
                                    }
                                    _ => {
                                        Err("Only symbols and integers are allowed in setters."
                                            .into())
                                    }
                                }
                            }
                            _ => Err("Only symbols and integers are allowed in setters.".into()),
                        }
                    }
                    _ => Err("Only symbols and integers are allowed in setters.".into()),
                }
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

    // TODO:
    fn get(&self, _sel: &SymbolRef, atoms: &[Atom]) -> Result<(), RytmExternalError> {
        // Format is set <object type> ...

        // Patterns
        // Format is set pattern <pattern-index> <track-index> <trig-index> <action> <value>
        // Other formats are set pattern <pattern-index> <action> <value>
        // set pattern <pattern-index> <track-index> <action> <value>
        // set pattern <pattern-index> <track-index> <trig-index> <action> <value>

        // We need to inspect atoms slowly.
        // First lets check the pair since they're always the same.

        let first_atom = atoms.get(0).ok_or(InvalidFormat)?;
        // let value = first_atom.get_value().ok_or(InvalidFormat)?;
        // let symbol = match value {
        //     AtomValue::Symbol(symbol) => symbol,
        //     _ => Err(InvalidFormat)?,
        // };

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
                match try_get_atom_value_assuming_action_or_index(1, atoms)? {
                    AtomValue::Symbol(action) => {
                        handle_get_action(GetAction::Pattern(PatternGetAction {
                            pattern: self.project.lock().unwrap().work_buffer_mut().pattern_mut(),
                            action,
                            out: &self.get_out,
                        }))
                    }
                    AtomValue::Int(track_index) => {
                        if !(0..=12).contains(&track_index) {
                            "Track index must be an integer between 0 and 12"
                                .obj_error(self.max_obj());
                        }
                        match try_get_atom_value_assuming_action_or_index(2, atoms)? {
                            AtomValue::Symbol(action) => {
                                handle_get_action(GetAction::Track(TrackGetAction {
                                    track: &self
                                        .project
                                        .lock()
                                        .unwrap()
                                        .work_buffer()
                                        .pattern()
                                        .tracks()[track_index as usize],
                                    action,
                                    out: &self.get_out,
                                }))
                            }
                            AtomValue::Int(trig_index) => {
                                if !(0..=63).contains(&track_index) {
                                    "Trig index must be an integer between 0 and 63"
                                        .obj_error(self.max_obj());
                                }
                                match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                                    AtomValue::Symbol(action) => {
                                        handle_get_action(GetAction::Trig(TrigGetAction {
                                            trig: &self
                                                .project
                                                .lock()
                                                .unwrap()
                                                .work_buffer()
                                                .pattern()
                                                .tracks()
                                                [track_index as usize]
                                                .trigs()
                                                [trig_index as usize],
                                            action,
                                            out: &self.get_out,
                                        }))
                                    }
                                    _ => {
                                        Err("Only symbols and integers are allowed in setters."
                                            .into())
                                    }
                                }
                            }
                            _ => Err("Only symbols and integers are allowed in setters.".into()),
                        }
                    }
                    _ => Err("Only symbols and integers are allowed in setters.".into()),
                }
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

fn try_get_atom_value_assuming_action_or_index(
    index: usize,
    atoms: &[Atom],
) -> Result<AtomValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            Ok(value)
        } else {
            Err("Invalid format: The list must be followed by an action or an index.".into())
        }
    } else {
        Err("Invalid format: The list must be followed by an action or an index.".into())
    }
}

fn try_get_action_value_from_atom_slice(
    index: usize,
    atoms: &[Atom],
) -> Result<AtomValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            Ok(value)
        } else {
            Err("Invalid format: An action must be followed by a parameter.".into())
        }
    } else {
        Err("Invalid format: An action must be followed by a parameter".into())
    }
}

pub enum SetAction<'a> {
    Pattern(PatternSetAction<'a>),
    Track(TrackSetAction<'a>),
    Trig(TrigSetAction<'a>),
}

pub enum GetAction<'a> {
    Pattern(PatternGetAction<'a>),
    Track(TrackGetAction<'a>),
    Trig(TrigGetAction<'a>),
}
pub struct PatternSetAction<'a> {
    pattern: &'a mut Pattern,
    action: SymbolRef,
    parameter: AtomValue,
}
pub struct PatternGetAction<'a> {
    pattern: &'a Pattern,
    action: SymbolRef,
    out: &'a OutAnything,
}

pub struct TrackSetAction<'a> {
    track: &'a mut Track,
    action: SymbolRef,
    parameter: AtomValue,
}

pub struct TrackGetAction<'a> {
    track: &'a Track,
    action: SymbolRef,
    out: &'a OutAnything,
}

pub struct TrigSetAction<'a> {
    trig: &'a mut Trig,
    action: SymbolRef,
    parameter: AtomValue,
}

pub struct TrigGetAction<'a> {
    trig: &'a Trig,
    action: SymbolRef,
    out: &'a OutAnything,
}

pub fn handle_set_action(action: SetAction) -> Result<(), RytmExternalError> {
    match action {
        SetAction::Pattern(action) => handle_pattern_set_action(action),
        SetAction::Track(action) => handle_track_set_action(action),
        SetAction::Trig(action) => handle_trig_set_action(action),
    }
}

pub fn handle_get_action(action: GetAction) -> Result<(), RytmExternalError> {
    match action {
        GetAction::Pattern(action) => handle_pattern_get_action(action),
        GetAction::Track(action) => handle_track_get_action(action),
        GetAction::Trig(action) => handle_trig_get_action(action),
    }
}

fn handle_pattern_get_action(action: PatternGetAction) -> Result<(), RytmExternalError> {
    todo!()
}
fn handle_track_get_action(action: TrackGetAction) -> Result<(), RytmExternalError> {
    todo!()
}
fn handle_trig_get_action(action: TrigGetAction) -> Result<(), RytmExternalError> {
    let TrigGetAction { action, trig, out } = action;

    let value: isize = match TrigActionType::try_from(action.clone())? {
        TrigActionType::Enable => trig.enabled_trig().into(),
        TrigActionType::Retrig => trig.enabled_retrig().into(),
        TrigActionType::Mute => trig.enabled_mute().into(),
        TrigActionType::Accent => trig.enabled_accent().into(),
        TrigActionType::Swing => trig.enabled_swing().into(),
        TrigActionType::Slide => trig.enabled_slide().into(),
    };

    let action_atom = Atom::from(action);
    let value_atom = Atom::from(AtomValue::Int(value));

    if let Err(_stack_overflow_err) = out.send(&[action_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}

fn handle_pattern_set_action(action: PatternSetAction) -> Result<(), RytmExternalError> {
    todo!()
}
fn handle_track_set_action(action: TrackSetAction) -> Result<(), RytmExternalError> {
    todo!()
}

fn handle_trig_set_action(action: TrigSetAction) -> Result<(), RytmExternalError> {
    let TrigSetAction {
        action,
        trig,
        parameter,
    } = action;

    match TrigActionType::try_from(action)? {
        TrigActionType::Enable => {
            trig.set_trig_enable(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Retrig => {
            trig.set_retrig(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Mute => {
            trig.set_mute(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Accent => {
            trig.set_accent(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Swing => {
            trig.set_swing(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Slide => {
            trig.set_slide(get_bool_from_0_or_1(parameter)?);
        }
    }

    Ok(())
}

fn get_bool_from_0_or_1(value: AtomValue) -> Result<bool, RytmExternalError> {
    match value {
        AtomValue::Int(value) => match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Value must be 0 or 1".into()),
        },
        _ => Err("Value must be 0 or 1".into()),
    }
}
