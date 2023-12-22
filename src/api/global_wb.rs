use crate::{
    action::{
        get::global::{handle_global_get_action, handle_global_get_enum_value},
        set::global::{handle_global_set_action, handle_global_set_enum_value},
    },
    error::{GetError, RytmExternalError, SetError},
    rytm::Rytm,
    util::{string_from_atom_slice, try_get_identifier_value_from_atom_slice},
};
use median::atom::{Atom, AtomValue};

use crate::util::try_get_atom_value_assuming_identifier_or_index_or_enum_value;

pub fn handle_global_wb_set(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_identifier_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let global_mut = guard.work_buffer_mut().global_mut();

            if let Some((enum_type, enum_value)) = enum_pair {
                // Some set calls might require an additional argument,
                let maybe_next_atom = atoms.get(2);
                handle_global_set_enum_value(global_mut, enum_type, enum_value, maybe_next_atom)
            } else {
                let parameter_atom = try_get_identifier_value_from_atom_slice(2, atoms)?;
                handle_global_set_action(global_mut, &action_or_enum_value_str, parameter_atom)
            }
        }
        _ => Err(SetError::InvalidGlobalWbSetterFormat(string_from_atom_slice(atoms)).into()),
    }
}

pub fn handle_global_wb_get(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_assuming_identifier_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let global = guard.work_buffer().global();

            if let Some((enum_type, enum_value)) = enum_pair {
                handle_global_get_enum_value(global, enum_type, enum_value, out)
            } else {
                // Some get calls might require an additional argument,
                let maybe_next_atom = atoms.get(2);
                handle_global_get_action(global, &action_or_enum_value_str, maybe_next_atom, out)
            }
        }
        _ => Err(GetError::InvalidGlobalWbGetterFormat(string_from_atom_slice(atoms)).into()),
    }
}
