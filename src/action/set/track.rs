use std::convert::TryInto;

use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::util::{get_bool_from_0_or_1, only_allow_numbers};
use crate::{api::track_action_type::*, api::track_enum_type::*, error::RytmExternalError};
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::pattern::track::Track;

pub struct TrackSetAction<'a> {
    pub track: &'a mut Track,
    pub action: SymbolRef,
    pub parameter: &'a Atom,
}

pub fn handle_track_set_action(action: TrackSetAction) -> Result<(), RytmExternalError> {
    let TrackSetAction {
        action,
        track,
        parameter,
    } = action;

    only_allow_numbers(parameter)?;

    match action.to_string()?.as_str() {
        DEF_TRIG_NOTE => Ok(track.set_default_trig_note(parameter.get_int() as usize)?),
        DEF_TRIG_VELOCITY => Ok(track.set_default_trig_velocity(parameter.get_int() as usize)?),
        DEF_TRIG_PROB => Ok(track.set_default_trig_probability(parameter.get_int() as usize)?),
        NUMBER_OF_STEPS => Ok(track.set_number_of_steps(parameter.get_int() as usize)?),
        QUANTIZE_AMOUNT => Ok(track.set_quantize_amount(parameter.get_int() as usize)?),
        SENDS_MIDI => {
            track.set_sends_midi(get_bool_from_0_or_1(parameter)?);
            Ok(())
        }
        EUCLIDEAN_MODE => {
            track.set_euclidean_mode(get_bool_from_0_or_1(parameter)?);
            Ok(())
        }
        EUCLIDEAN_PL1 => Ok(track.set_euclidean_pl1(parameter.get_int() as usize)?),
        EUCLIDEAN_PL2 => Ok(track.set_euclidean_pl2(parameter.get_int() as usize)?),
        EUCLIDEAN_RO1 => Ok(track.set_euclidean_ro1(parameter.get_int() as usize)?),
        EUCLIDEAN_RO2 => Ok(track.set_euclidean_ro2(parameter.get_int() as usize)?),
        EUCLIDEAN_TRO => Ok(track.set_euclidean_tro(parameter.get_int() as usize)?),

        other => Err(InvalidActionType(other.to_string()).into()),
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

        other => return Err(InvalidEnumType(other.to_string()).into()),
    }

    Ok(())
}
