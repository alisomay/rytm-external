use crate::error::ActionError::InvalidActionParameter;
use crate::error::RytmExternalError;
use median::atom::{Atom, AtomType, AtomValue};

pub fn get_bool_from_0_or_1(value: &Atom) -> Result<bool, RytmExternalError> {
    match value
        .get_value()
        .ok_or_else(|| RytmExternalError::from("Value must be 0 or 1"))?
    {
        AtomValue::Int(value) => match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Value must be 0 or 1".into()),
        },
        _ => Err("Value must be 0 or 1".into()),
    }
}

pub fn try_get_atom_value_assuming_action_or_index_or_enum_value(
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

pub fn try_get_action_value_from_atom_slice(
    index: usize,
    atoms: &[Atom],
) -> Result<&Atom, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            Ok(atom)
        } else {
            Err("Invalid format: An action must be followed by a parameter.".into())
        }
    } else {
        Err("Invalid format: An action must be followed by a parameter".into())
    }
}

pub fn only_allow_numbers(atom: &Atom) -> Result<(), RytmExternalError> {
    match atom.get_type() {
        Some(AtomType::Object) | Some(AtomType::Symbol) | None => Err(InvalidActionParameter(
            "Action parameters can be only integers or floats.".to_owned(),
        )
        .into()),
        _ => Ok(()),
    }
}
