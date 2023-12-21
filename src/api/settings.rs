use crate::{
    action::{
        get::settings::{handle_settings_get_action, handle_settings_get_enum_value},
        set::settings::{handle_settings_set_action, handle_settings_set_enum_value},
    },
    error::RytmExternalError,
    rytm::Rytm,
    util::try_get_action_value_from_atom_slice,
};
use median::atom::{Atom, AtomValue};

use crate::util::try_get_atom_value_assuming_action_or_index_or_enum_value;

//TODO:
const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

pub fn handle_settings_set(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_action_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let settings_mut = guard.settings_mut();

            if let Some((enum_type, enum_value)) = enum_pair {
                handle_settings_set_enum_value(settings_mut, enum_type, enum_value)
            } else {
                let parameter_atom = try_get_action_value_from_atom_slice(2, atoms)?;
                handle_settings_set_action(settings_mut, &action_or_enum_value_str, parameter_atom)
            }
        }
        _ => Err(ERR.into()),
    }
}

pub fn handle_settings_get(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_assuming_action_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let settings = guard.settings();

            if let Some((enum_type, _)) = enum_pair {
                handle_settings_get_enum_value(settings, enum_type, out)
            } else {
                let maybe_next_atom = atoms.get(2);
                handle_settings_get_action(
                    settings,
                    &action_or_enum_value_str,
                    maybe_next_atom,
                    out,
                )
            }
        }
        _ => Err(ERR.into()),
    }
}
