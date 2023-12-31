use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::util::{
    only_allow_numbers_as_identifier_parameter, try_get_identifier_value_from_atom_slice,
};
use crate::{api::pattern_action_type::*, api::pattern_enum_type::*, error::RytmExternalError};
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::Pattern;
use std::convert::TryInto;

use super::{handle_set_action, SetAction};

pub struct PatternSetAction<'a> {
    pub pattern: &'a mut Pattern,
    pub action: SymbolRef,
    pub parameter: &'a Atom,
}

pub fn pattern_set(
    action_or_enum_value: SymbolRef,
    pattern: &mut Pattern,
    atoms: &[Atom],
    select: usize,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, enum_variant)) = action_or_enum_value.to_string()?.split_once(':') {
        handle_pattern_enum_set_action(pattern, enum_type, enum_variant)
    } else {
        handle_set_action(SetAction::Pattern(PatternSetAction {
            pattern,
            action: action_or_enum_value,
            parameter: try_get_identifier_value_from_atom_slice(select, atoms)?,
        }))
    }
}

pub fn handle_pattern_set_action(action: PatternSetAction) -> Result<(), RytmExternalError> {
    let PatternSetAction {
        action,
        pattern,
        parameter,
    } = action;

    only_allow_numbers_as_identifier_parameter(parameter)?;

    match action.to_string()?.as_str() {
        MASTER_LENGTH => Ok(pattern.set_master_length(parameter.get_int() as usize)?),
        MASTER_CHANGE => Ok(pattern.set_master_change(parameter.get_int() as usize)?),
        KIT_NUMBER => Ok(pattern.set_kit_number(parameter.get_int() as usize)?),
        SWING_AMOUNT => Ok(pattern.set_swing_amount(parameter.get_int() as usize)?),
        GLOBAL_QUANTIZE => Ok(pattern.set_global_quantize(parameter.get_int() as usize)?),
        BPM => Ok(pattern.set_bpm(parameter.get_float() as f32)?),

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
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
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(())
}
