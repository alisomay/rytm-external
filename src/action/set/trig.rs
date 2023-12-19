use crate::api::trig_action_type::*;
use crate::api::trig_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::util::only_allow_numbers_as_action_parameter;
use crate::{error::RytmExternalError, util::get_bool_from_0_or_1};
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::pattern::{track::trig::HoldsTrigFlags, Trig};
use std::convert::TryInto;

pub struct TrigSetAction<'a> {
    pub trig: &'a mut Trig,
    pub action: SymbolRef,
    pub parameter: &'a Atom,
}

pub fn handle_trig_set_action(action: TrigSetAction) -> Result<(), RytmExternalError> {
    let TrigSetAction {
        action,
        trig,
        parameter,
    } = action;

    only_allow_numbers_as_action_parameter(parameter)?;

    match action.to_string()?.as_str() {
        ENABLE => trig.set_trig_enable(get_bool_from_0_or_1(parameter)?),
        RETRIG => trig.set_retrig(get_bool_from_0_or_1(parameter)?),
        MUTE => trig.set_mute(get_bool_from_0_or_1(parameter)?),
        ACCENT => trig.set_accent(get_bool_from_0_or_1(parameter)?),
        SWING => trig.set_swing(get_bool_from_0_or_1(parameter)?),
        SLIDE => trig.set_slide(get_bool_from_0_or_1(parameter)?),
        // TODO: Do the rest of the flags
        NOTE => trig.set_note(parameter.get_int() as usize)?,
        VELOCITY => trig.set_velocity(parameter.get_int() as usize)?,
        RETRIG_VELOCITY_OFFSET => trig.set_retrig_velocity_offset(parameter.get_int())?,
        SOUND_LOCK => trig.set_sound_lock(parameter.get_int() as usize)?,

        other => return Err(InvalidActionType(other.to_string()).into()),
    }

    Ok(())
}

pub fn handle_trig_enum_set_action(
    trig: &mut Trig,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    match enum_type {
        MICRO_TIME => trig.set_micro_timing(enum_value.try_into()?),
        NOTE_LENGTH => trig.set_note_length(enum_value.try_into()?),
        RETRIG_LENGTH => trig.set_retrig_length(enum_value.try_into()?),
        RETRIG_RATE => trig.set_retrig_rate(enum_value.try_into()?),
        TRIG_CONDITION => trig.set_trig_condition(enum_value.try_into()?),

        other => return Err(InvalidEnumType(other.to_string()).into()),
    }

    Ok(())
}
