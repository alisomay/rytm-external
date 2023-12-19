use std::convert::TryFrom;

use crate::api::trig_action_type::*;
use crate::api::trig_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
    symbol::SymbolRef,
};
use rytm_rs::object::pattern::{track::trig::HoldsTrigFlags, Trig};

pub struct TrigGetAction<'a> {
    pub trig: &'a Trig,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
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

        other => return Err(InvalidActionType(other.to_string()).into()),
    };

    let action_atom = Atom::from(action);
    let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));
    let track_index_atom = Atom::from(AtomValue::Int(trig.track_index() as isize));
    let value_atom = Atom::from(AtomValue::Int(value));

    if let Err(_stack_overflow_err) =
        out.send(&[action_atom, track_index_atom, index_atom, value_atom][..])
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

        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));
    let track_index_atom = Atom::from(AtomValue::Int(trig.track_index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(
        &[
            enum_type_atom,
            track_index_atom,
            index_atom,
            enum_value_atom,
        ][..],
    ) {
        // Stack overflow ignore
    }

    Ok(())
}
