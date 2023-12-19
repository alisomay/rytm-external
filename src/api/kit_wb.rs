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
        try_get_atom_value_as_kit_element_or_action_or_enum_value, try_get_index_with_range,
        KitElementOrActionOrEnumTypeAndValue,
    },
};
use median::atom::Atom;

use crate::util::try_get_action_value_from_atom_slice;

// kit_wb fx... val
// kit_wb tracklevel 0 0
// kit wb trackretrigrate 0 0
// kit wb sound 0 ..

// When parsing

// action_or_enum_value and param
// kit_element element_index action_or_enum_value and param

// if sound (maybe special handling with slice of new atoms passed in?) // currently omit it.

// Then the strategy is
// 1 check for a symbol first and it needs to be either a kit element or action/enumvalue, if not error
// 2 if action/enumvalue look for a param, if not error
// 2 if kit element, treat the next one as index over the element check range
// 3 (only after kit elem) treat it as the param for the chosen element

pub fn handle_kit_wb_set(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_as_kit_element_or_action_or_enum_value(1, atoms)? {
        KitElementOrActionOrEnumTypeAndValue::Action(action) => {
            // Send for handling..  // Next value should be a param
            handle_kit_set_action(
                guard.work_buffer_mut().kit_mut(),
                action,
                try_get_action_value_from_atom_slice(2, atoms)?,
            )
        }
        KitElementOrActionOrEnumTypeAndValue::EnumTypeAndValue(t, v) => {
            // Send for handling..
            handle_kit_set_enum_value(guard.work_buffer_mut().kit_mut(), &t, &v)
        }
        KitElementOrActionOrEnumTypeAndValue::KitElement(element_type) => {
            let element_index = try_get_index_with_range(
                atoms,
                2,
                0,
                12,
                &format!("kit element ({element_type})"),
            )?;
            let element_parameter = try_get_action_value_from_atom_slice(3, atoms)?;

            handle_kit_set_kit_element(
                guard.work_buffer_mut().kit_mut(),
                &element_type,
                element_index,
                element_parameter,
            )
        }
        KitElementOrActionOrEnumTypeAndValue::KitSound => {
            let sound_index = try_get_index_with_range(atoms, 2, 0, 11, "kit element (sound)")?;
            // Send to sound handling with a slice of atoms
            // For the sound we'll again try getting the index but then slice the atoms here and send it to the sound handler.
            handle_kit_set_kit_sound(
                &mut guard.work_buffer_mut().kit_mut().sounds_mut()[sound_index],
                atoms,
                3,
            )
        }
    }
}

pub fn handle_kit_wb_get(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;
    match try_get_atom_value_as_kit_element_or_action_or_enum_value(1, atoms)? {
        KitElementOrActionOrEnumTypeAndValue::Action(action) => {
            // Send for handling..  // Next value should be a param
            handle_kit_get_action(guard.work_buffer().kit(), action, out)
        }
        KitElementOrActionOrEnumTypeAndValue::EnumTypeAndValue(t, _) => {
            // Send for handling..
            handle_kit_get_enum_value(guard.work_buffer().kit(), &t, out)
        }
        KitElementOrActionOrEnumTypeAndValue::KitElement(element_type) => {
            let element_index = try_get_index_with_range(
                atoms,
                2,
                0,
                12,
                &format!("kit element ({element_type})"),
            )?;

            handle_kit_get_kit_element(guard.work_buffer().kit(), &element_type, element_index, out)
        }
        KitElementOrActionOrEnumTypeAndValue::KitSound => {
            let sound_index = try_get_index_with_range(atoms, 2, 0, 11, "kit element (sound)")?;
            // Send to sound handling with a slice of atoms
            // For the sound we'll again try getting the index but then slice the atoms here and send it to the sound handler.
            handle_kit_get_kit_sound(&guard.work_buffer().kit().sounds()[sound_index], atoms, 3)
        }
    }
}
