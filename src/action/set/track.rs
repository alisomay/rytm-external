use std::convert::TryInto;

use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::util::{
    get_bool_from_0_or_1, only_allow_numbers_as_identifier_parameter,
    try_get_action_value_from_atom_slice,
};
use crate::{api::track_action_type::*, api::track_enum_type::*, error::RytmExternalError};
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::pattern::track::Track;

use super::{handle_set_action, SetAction};

pub struct TrackSetAction<'a> {
    pub track: &'a mut Track,
    pub action: SymbolRef,
    pub parameter: &'a Atom,
}

pub fn track_set(
    action_or_enum_value: SymbolRef,
    track: &mut rytm_rs::object::pattern::track::Track,
    atoms: &[Atom],
    select: usize,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, enum_variant)) = action_or_enum_value.to_string()?.split_once(':') {
        handle_track_enum_set_action(track, enum_type, enum_variant)
    } else {
        handle_set_action(SetAction::Track(TrackSetAction {
            track,
            action: action_or_enum_value,
            parameter: try_get_action_value_from_atom_slice(select, atoms)?,
        }))
    }
}

pub fn handle_track_set_action(action: TrackSetAction) -> Result<(), RytmExternalError> {
    let TrackSetAction {
        action,
        track,
        parameter,
    } = action;

    only_allow_numbers_as_identifier_parameter(parameter)?;

    match action.to_string()?.as_str() {
        DEF_TRIG_NOTE => Ok(track.set_default_trig_note(parameter.get_int() as usize)?),
        DEF_TRIG_VELOCITY => Ok(track.set_default_trig_velocity(parameter.get_int() as usize)?),
        DEF_TRIG_PROB => Ok(track.set_default_trig_probability(parameter.get_int() as usize)?),
        NUMBER_OF_STEPS => Ok(track.set_number_of_steps(parameter.get_int() as usize)?),
        QUANTIZE_AMOUNT => Ok(track.set_quantize_amount(parameter.get_int() as usize)?),
        SENDS_MIDI => {
            track.set_sends_midi(get_bool_from_0_or_1(parameter, SENDS_MIDI)?);
            Ok(())
        }
        EUCLIDEAN_MODE => {
            track.set_euclidean_mode(get_bool_from_0_or_1(parameter, EUCLIDEAN_MODE)?);
            Ok(())
        }
        EUCLIDEAN_PL1 => Ok(track.set_euclidean_pl1(parameter.get_int() as usize)?),
        EUCLIDEAN_PL2 => Ok(track.set_euclidean_pl2(parameter.get_int() as usize)?),
        EUCLIDEAN_RO1 => Ok(track.set_euclidean_ro1(parameter.get_int() as usize)?),
        EUCLIDEAN_RO2 => Ok(track.set_euclidean_ro2(parameter.get_int() as usize)?),
        EUCLIDEAN_TRO => Ok(track.set_euclidean_tro(parameter.get_int() as usize)?),

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

pub fn handle_track_enum_set_action(
    track: &mut Track,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    match enum_type {
        ROOT_NOTE => track.set_root_note(enum_value.try_into()?),
        PAD_SCALE => track.set_pad_scale(enum_value.try_into()?),
        DEFAULT_NOTE_LENGTH => track.set_default_trig_note_length(enum_value.try_into()?),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(())
}
