use crate::{
    action::{
        get::global::{handle_global_get_action, handle_global_get_enum_value},
        set::global::{handle_global_set_action, handle_global_set_enum_value},
    },
    error::RytmExternalError,
    rytm::Rytm,
    traits::Post,
    util::try_get_action_value_from_atom_slice,
};
use median::{
    atom::{Atom, AtomValue},
    object::MaxObj,
};

use crate::util::try_get_atom_value_assuming_action_or_index_or_enum_value;

//TODO:
const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

pub fn handle_global_set(
    rytm: &Rytm,
    atoms: &[Atom],
    global_index: usize,
) -> Result<(), RytmExternalError> {
    if !(0..=3).contains(&global_index) {
        "Global slot index must be an integer between 0 and 3".obj_error(rytm.max_obj());
    }
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let global_mut = guard.work_buffer_mut().global_mut();

            if let Some((enum_type, enum_value)) = enum_pair {
                // Some set calls might require an additional argument,
                let maybe_next_atom = atoms.get(3);
                handle_global_set_enum_value(global_mut, enum_type, enum_value, maybe_next_atom)
            } else {
                let parameter_atom = try_get_action_value_from_atom_slice(3, atoms)?;
                handle_global_set_action(global_mut, &action_or_enum_value_str, parameter_atom)
            }
        }
        _ => Err(ERR.into()),
    }
}

pub fn handle_global_get(
    rytm: &Rytm,
    atoms: &[Atom],
    global_index: usize,
) -> Result<(), RytmExternalError> {
    if !(0..=3).contains(&global_index) {
        "Global slot index must be an integer between 0 and 3".obj_error(rytm.max_obj());
    }
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let global = guard.work_buffer().global();

            if let Some((enum_type, enum_value)) = enum_pair {
                handle_global_get_enum_value(global, enum_type, enum_value, out)
            } else {
                let maybe_next_atom = atoms.get(3);
                handle_global_get_action(global, &action_or_enum_value_str, maybe_next_atom, out)
            }
        }
        _ => Err(ERR.into()),
    }
}
