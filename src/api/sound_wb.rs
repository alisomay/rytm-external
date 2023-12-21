use crate::{
    action::{
        get::sound::{handle_sound_get_action, handle_sound_get_enum_value},
        set::sound::{handle_sound_set_action, handle_sound_set_enum_value},
    },
    error::{GetError, RytmExternalError},
    rytm::Rytm,
    util::{string_from_atom_slice, try_get_atom_value_assuming_action_or_index_or_enum_value},
};
use median::atom::{Atom, AtomValue};

use crate::util::try_get_action_value_from_atom_slice;

// sound <index> <action> <value>
// sound <index> <enum>

const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

pub fn handle_sound_wb_set(
    rytm: &Rytm,
    atoms: &[Atom],
    sound_index: usize,
) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let sound_mut = &mut guard.pool_sounds_mut()[sound_index];

            if let Some((enum_type, enum_value)) = enum_pair {
                let maybe_next_atom = atoms.get(3);
                handle_sound_set_enum_value(sound_mut, enum_type, enum_value, maybe_next_atom)
            } else {
                let parameter_atom = try_get_action_value_from_atom_slice(3, atoms)?;
                let maybe_next_atom = atoms.get(4);
                handle_sound_set_action(
                    sound_mut,
                    &action_or_enum_value_str,
                    parameter_atom,
                    maybe_next_atom,
                )
            }
        }
        _ => Err(ERR.into()),
    }
}

pub fn handle_sound_wb_get(
    rytm: &Rytm,
    atoms: &[Atom],
    sound_index: usize,
) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            let sound = &guard.pool_sounds()[sound_index];

            if let Some((enum_type, enum_value)) = enum_pair {
                handle_sound_get_enum_value(sound, enum_type, enum_value, out)
            } else {
                let maybe_next_atom = atoms.get(3);
                handle_sound_get_action(sound, &action_or_enum_value_str, maybe_next_atom, out)
            }
        }
        _ => Err(GetError::InvalidSoundGetterFormat(string_from_atom_slice(atoms)).into()),
    }
}
