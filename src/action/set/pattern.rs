use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::util::only_allow_numbers_as_action_parameter;
use crate::{api::pattern_action_type::*, api::pattern_enum_type::*, error::RytmExternalError};
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::Pattern;
use std::convert::TryInto;

pub struct PatternSetAction<'a> {
    pub pattern: &'a mut Pattern,
    pub action: SymbolRef,
    pub parameter: &'a Atom,
}

pub fn handle_pattern_set_action(action: PatternSetAction) -> Result<(), RytmExternalError> {
    let PatternSetAction {
        action,
        pattern,
        parameter,
    } = action;

    only_allow_numbers_as_action_parameter(parameter)?;

    match action.to_string()?.as_str() {
        MASTER_LENGTH => Ok(pattern.set_master_length(parameter.get_int() as usize)?),
        MASTER_CHANGE => Ok(pattern.set_master_change(parameter.get_int() as usize)?),
        KIT_NUMBER => Ok(pattern.set_kit_number(parameter.get_int() as usize)?),
        SWING_AMOUNT => Ok(pattern.set_swing_amount(parameter.get_int() as usize)?),
        GLOBAL_QUANTIZE => Ok(pattern.set_global_quantize(parameter.get_int() as usize)?),
        BPM => Ok(pattern.set_bpm(parameter.get_float() as f32)?),

        other => Err(InvalidActionType(other.to_string()).into()),
    }
}

pub fn handle_pattern_enum_set_action(
    pattern: &mut Pattern,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    match enum_type {
        SPEED => pattern.set_speed(enum_value.try_into()?),
        TIME_MODE => pattern.set_time_mode(enum_value.try_into()?),
        other => return Err(InvalidEnumType(other.to_string()).into()),
    }

    Ok(())
}
