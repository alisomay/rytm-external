use crate::error::RytmExternalError;
use median::atom::{Atom, AtomValue};

pub fn get_bool_from_0_or_1(value: AtomValue) -> Result<bool, RytmExternalError> {
    match value {
        AtomValue::Int(value) => match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Value must be 0 or 1".into()),
        },
        _ => Err("Value must be 0 or 1".into()),
    }
}

pub fn try_get_atom_value_assuming_action_or_index(
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
) -> Result<AtomValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            Ok(value)
        } else {
            Err("Invalid format: An action must be followed by a parameter.".into())
        }
    } else {
        Err("Invalid format: An action must be followed by a parameter".into())
    }
}
