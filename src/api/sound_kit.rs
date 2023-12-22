use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
};
use rytm_rs::object::Sound;

use crate::{
    action::{
        get::sound::{handle_sound_get_action, handle_sound_get_enum_value},
        set::sound::{handle_sound_set_action, handle_sound_set_enum_value},
    },
    error::{GetError, RytmExternalError, SetError},
    util::{
        string_from_atom_slice, try_get_atom_value_assuming_identifier_or_index_or_enum_value,
        try_get_identifier_value_from_atom_slice,
    },
};

pub fn handle_sound_kit_set(
    sound_mut: &mut Sound,
    atoms: &[Atom],
) -> Result<(), RytmExternalError> {
    match try_get_atom_value_assuming_identifier_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

            if let Some((enum_type, enum_value)) = enum_pair {
                let maybe_next_atom = atoms.get(3);
                handle_sound_set_enum_value(sound_mut, enum_type, enum_value, maybe_next_atom)
            } else {
                let parameter_atom = try_get_identifier_value_from_atom_slice(3, atoms)?;
                let maybe_next_atom = atoms.get(4);
                handle_sound_set_action(
                    sound_mut,
                    &action_or_enum_value_str,
                    parameter_atom,
                    maybe_next_atom,
                )
            }
        }
        _ => Err(SetError::InvalidSoundSetterFormat(string_from_atom_slice(atoms)).into()),
    }
}

pub fn handle_sound_kit_get(
    sound: &Sound,
    atoms: &[Atom],
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    match try_get_atom_value_assuming_identifier_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            let action_or_enum_value_str = action_or_enum_value.to_string()?;
            let enum_pair = action_or_enum_value_str.split_once(':');

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
