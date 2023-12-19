use std::convert::TryFrom;

use crate::api::pattern_action_type::*;
use crate::api::pattern_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
    symbol::SymbolRef,
};
use rytm_rs::object::Pattern;

pub struct PatternGetAction<'a> {
    pub pattern: &'a Pattern,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}

pub fn handle_pattern_get_action(action: PatternGetAction) -> Result<(), RytmExternalError> {
    let PatternGetAction {
        action,
        pattern,
        out,
    } = action;

    let value_atom: Atom = match action.to_string()?.as_str() {
        IS_WORK_BUFFER => Atom::from(pattern.is_work_buffer_pattern() as isize),
        VERSION => Atom::from(pattern.structure_version() as isize),
        INDEX => Atom::from(pattern.index() as isize),
        MASTER_LENGTH => Atom::from(pattern.master_length() as isize),
        MASTER_CHANGE => Atom::from(pattern.master_change() as isize),
        KIT_NUMBER => Atom::from(pattern.kit_number() as isize),
        SWING_AMOUNT => Atom::from(pattern.swing_amount() as isize),
        GLOBAL_QUANTIZE => Atom::from(pattern.global_quantize() as isize),
        BPM => Atom::from(pattern.bpm() as f64),

        other => return Err(InvalidActionType(other.to_string()).into()),
    };

    let action_atom = Atom::from(action);
    let index_atom = Atom::from(AtomValue::Int(pattern.index() as isize));

    if let Err(_stack_overflow_err) = out.send(&[action_atom, index_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_pattern_enum_get_action(
    pattern: &Pattern,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        SPEED => pattern.speed().into(),
        TIME_MODE => pattern.time_mode().into(),

        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(pattern.index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(&[enum_type_atom, index_atom, enum_value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
