use std::convert::TryFrom;

use crate::api::trig_action_type::*;
use crate::api::trig_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
    symbol::SymbolRef,
};
use rytm_rs::object::pattern::{track::trig::HoldsTrigFlags, Trig};

use super::handle_get_action;
use super::GetAction;

pub struct TrigGetAction<'a> {
    pub trig: &'a Trig,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}

pub fn trig_get(
    action_or_enum_type: SymbolRef,
    trig: &rytm_rs::object::pattern::track::trig::Trig,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, _)) = action_or_enum_type.to_string()?.split_once(':') {
        handle_trig_enum_get_action(trig, enum_type, out)
    } else {
        handle_get_action(GetAction::Trig(TrigGetAction {
            trig,
            action: action_or_enum_type,
            out,
        }))
    }
}

pub fn handle_trig_get_action(action: TrigGetAction) -> Result<(), RytmExternalError> {
    let TrigGetAction { action, trig, out } = action;

    let value: isize = match action.to_string()?.as_str() {
        ENABLE => trig.enabled_trig().into(),
        RETRIG => trig.enabled_retrig().into(),
        MUTE => trig.enabled_mute().into(),
        ACCENT => trig.enabled_accent().into(),
        SWING => trig.enabled_swing().into(),
        SLIDE => trig.enabled_slide().into(),
        // TODO: Do the rest of the flags..
        NOTE => trig.note() as isize,
        VELOCITY => trig.velocity() as isize,
        RETRIG_VELOCITY_OFFSET => trig.retrig_velocity_offset(),
        SOUND_LOCK => trig.sound_lock() as isize,

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let action_atom = Atom::from(action);
    let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));
    let track_index_atom = Atom::from(AtomValue::Int(trig.track_index() as isize));
    let value_atom = Atom::from(AtomValue::Int(value));

    if let Err(_stack_overflow_err) =
        out.send(&[track_index_atom, index_atom, action_atom, value_atom][..])
    {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_trig_enum_get_action(
    trig: &Trig,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        MICRO_TIME => trig.micro_timing().into(),
        NOTE_LENGTH => trig.note_length().into(),
        RETRIG_LENGTH => trig.retrig_length().into(),
        RETRIG_RATE => trig.retrig_rate().into(),
        TRIG_CONDITION => trig.trig_condition().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));
    let track_index_atom = Atom::from(AtomValue::Int(trig.track_index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(
        &[
            track_index_atom,
            index_atom,
            enum_type_atom,
            enum_value_atom,
        ][..],
    ) {
        // Stack overflow ignore
    }

    Ok(())
}
