use std::convert::TryFrom;

use crate::api::track_action_type::*;
use crate::api::track_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
    symbol::SymbolRef,
};
use rytm_rs::object::pattern::track::Track;

use super::handle_get_action;
use super::GetAction;

pub struct TrackGetAction<'a> {
    pub track: &'a Track,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}

pub fn track_get(
    action_or_enum_type: SymbolRef,
    track: &rytm_rs::object::pattern::track::Track,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, _)) = action_or_enum_type.to_string()?.split_once(':') {
        handle_track_enum_get_action(track, enum_type, out)
    } else {
        handle_get_action(GetAction::Track(TrackGetAction {
            track,
            action: action_or_enum_type,
            out,
        }))
    }
}

pub fn handle_track_get_action(action: TrackGetAction) -> Result<(), RytmExternalError> {
    let TrackGetAction { action, track, out } = action;

    let value: usize = match action.to_string()?.as_str() {
        INDEX => track.index(),
        OWNER_INDEX => track.owner_pattern_index(),
        DEF_TRIG_NOTE => track.default_trig_note(),
        DEF_TRIG_VELOCITY => track.default_trig_velocity(),
        DEF_TRIG_PROB => track.default_trig_probability(),
        NUMBER_OF_STEPS => track.number_of_steps(),
        QUANTIZE_AMOUNT => track.quantize_amount(),
        SENDS_MIDI => track.sends_midi().into(),
        EUCLIDEAN_MODE => track.euclidean_mode().into(),
        EUCLIDEAN_PL1 => track.euclidean_pl1(),
        EUCLIDEAN_PL2 => track.euclidean_pl2(),
        EUCLIDEAN_RO1 => track.euclidean_ro1(),
        EUCLIDEAN_RO2 => track.euclidean_ro2(),
        EUCLIDEAN_TRO => track.euclidean_tro(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let action_atom = Atom::from(action);
    let index_atom = Atom::from(AtomValue::Int(track.index() as isize));
    let pattern_index_atom = Atom::from(AtomValue::Int(track.owner_pattern_index() as isize));
    // No range problems here.
    let value_atom = Atom::from(AtomValue::Int(value as isize));

    if let Err(_stack_overflow_err) =
        out.send(&[action_atom, pattern_index_atom, index_atom, value_atom][..])
    {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_track_enum_get_action(
    track: &Track,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        ROOT_NOTE => track.root_note().into(),
        PAD_SCALE => track.pad_scale().into(),
        DEFAULT_NOTE_LENGTH => track.default_trig_note_length().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(track.index() as isize));
    let pattern_index_atom = Atom::from(AtomValue::Int(track.owner_pattern_index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(
        &[
            enum_type_atom,
            pattern_index_atom,
            index_atom,
            enum_value_atom,
        ][..],
    ) {
        // Stack overflow ignore
    }

    Ok(())
}
