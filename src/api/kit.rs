use crate::{
    action::{
        get::kit::{
            handle_kit_get_action, handle_kit_get_enum_value, handle_kit_get_kit_element,
            handle_kit_get_kit_sound,
        },
        set::kit::{
            handle_kit_set_action, handle_kit_set_enum_value, handle_kit_set_kit_element,
            handle_kit_set_kit_sound,
        },
    },
    error::RytmExternalError,
    rytm::Rytm,
    util::{
        try_get_atom_value_as_kit_element_or_identifier_or_enum_value, try_get_index_with_range,
        KitElementOrActionOrEnumTypeAndValue,
    },
};
use median::atom::Atom;

use crate::util::try_get_identifier_value_from_atom_slice;

pub fn handle_kit_set(
    rytm: &Rytm,
    atoms: &[Atom],
    kit_index: usize,
) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_as_kit_element_or_identifier_or_enum_value(2, atoms)? {
        KitElementOrActionOrEnumTypeAndValue::Action(action) => {
            // Send for handling..  // Next value should be a param
            let maybe_next_atom = atoms.get(4);
            handle_kit_set_action(
                &mut guard.kits_mut()[kit_index],
                &action,
                try_get_identifier_value_from_atom_slice(3, atoms)?,
                maybe_next_atom,
            )
        }
        KitElementOrActionOrEnumTypeAndValue::EnumTypeAndValue(t, v) => {
            // Send for handling..
            let maybe_next_atom = atoms.get(3);
            handle_kit_set_enum_value(&mut guard.kits_mut()[kit_index], &t, &v, maybe_next_atom)
        }
        KitElementOrActionOrEnumTypeAndValue::KitElement(element_type) => {
            let element_index = try_get_index_with_range(
                atoms,
                3,
                0,
                12,
                &format!("kit element ({element_type})"),
            )?;
            let element_parameter = try_get_identifier_value_from_atom_slice(3, atoms)?;

            handle_kit_set_kit_element(
                &mut guard.kits_mut()[kit_index],
                &element_type,
                element_index,
                element_parameter,
            )
        }
        KitElementOrActionOrEnumTypeAndValue::KitSound => {
            let sound_index = try_get_index_with_range(atoms, 3, 0, 11, "kit element (sound)")?;
            // Send to sound handling with a slice of atoms
            // For the sound we'll again try getting the index but then slice the atoms here and send it to the sound handler.
            handle_kit_set_kit_sound(
                &mut guard.kits_mut()[kit_index].sounds_mut()[sound_index],
                atoms,
                4,
            )
        }
    }
}

pub fn handle_kit_get(
    rytm: &Rytm,
    atoms: &[Atom],
    kit_index: usize,
) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_as_kit_element_or_identifier_or_enum_value(2, atoms)? {
        KitElementOrActionOrEnumTypeAndValue::Action(action) => {
            // Send for handling..  // Next value should be a param
            let maybe_next_atom = atoms.get(3);
            handle_kit_get_action(&guard.kits()[kit_index], action, out, maybe_next_atom)
        }
        KitElementOrActionOrEnumTypeAndValue::EnumTypeAndValue(t, v) => {
            // Send for handling..
            handle_kit_get_enum_value(&guard.kits()[kit_index], &t, &v, out)
        }
        KitElementOrActionOrEnumTypeAndValue::KitElement(element_type) => {
            let element_index = try_get_index_with_range(
                atoms,
                3,
                0,
                12,
                &format!("kit element ({element_type})"),
            )?;

            handle_kit_get_kit_element(&guard.kits()[kit_index], &element_type, element_index, out)
        }
        KitElementOrActionOrEnumTypeAndValue::KitSound => {
            let sound_index = try_get_index_with_range(atoms, 3, 0, 11, "kit element (sound)")?;
            // Send to sound handling with a slice of atoms
            // For the sound we'll again try getting the index but then slice the atoms here and send it to the sound handler.
            handle_kit_get_kit_sound(
                &guard.kits()[kit_index].sounds()[sound_index],
                atoms,
                4,
                out,
            )
        }
    }
}
