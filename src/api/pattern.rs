use crate::{
    action::{
        get::{pattern::pattern_get, track::track_get, trig::trig_get},
        plock::{handle_trig_plock_getter_action, handle_trig_plock_setter_action},
        set::{pattern::pattern_set, track::track_set, trig::trig_set},
    },
    error::{GetError, RytmExternalError},
    rytm::Rytm,
    traits::Post, util::string_from_atom_slice,
};
use median::{
    atom::{Atom, AtomValue},
    object::MaxObj,
};

use super::plock_type::ALL_PLOCK_TYPES;
use crate::util::try_get_atom_value_assuming_action_or_index_or_enum_value;

const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

pub fn handle_pattern_set(
    rytm: &Rytm,
    atoms: &[Atom],
    pattern_index: usize,
) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            // Check the next value and finish the list.
            pattern_set(
                action_or_enum_value,
                &mut guard.patterns_mut()[pattern_index],
                atoms,
                3,
            )
        }
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index_or_enum_value(3, atoms)? {
                AtomValue::Symbol(action_or_enum_value) => {
                    // Check the next value and finish the list.
                    track_set(
                        action_or_enum_value,
                        &mut guard.patterns_mut()[pattern_index].tracks_mut()[track_index as usize],
                        atoms,
                        4,
                    )
                }
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index_or_enum_value(4, atoms)? {
                        AtomValue::Symbol(action_or_enum_value) => {
                            let trig_mut = &mut guard.work_buffer_mut().pattern_mut().tracks_mut()
                                [track_index as usize]
                                .trigs_mut()[trig_index as usize];

                            // Check if it is a plock action first
                            let action_or_enum_value_str = action_or_enum_value.to_string()?;
                            if ALL_PLOCK_TYPES.contains(&action_or_enum_value_str.as_str()) {
                                return handle_trig_plock_setter_action(
                                    trig_mut,
                                    &action_or_enum_value_str,
                                    atoms,
                                    5,
                                );
                            }

                            trig_set(action_or_enum_value, trig_mut, atoms, 5)
                        }
                        _ => Err(ERR.into()),
                    }
                }
                _ => Err(ERR.into()),
            }
        }
        _ => Err(ERR.into()),
    }
}

pub fn handle_pattern_get(
    rytm: &Rytm,
    atoms: &[Atom],
    pattern_index: usize,
) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;

    match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
        AtomValue::Symbol(action_or_enum_type) => {
            pattern_get(action_or_enum_type, &guard.patterns()[pattern_index], out)
        }
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index_or_enum_value(3, atoms)? {
                AtomValue::Symbol(action_or_enum_type) => track_get(
                    action_or_enum_type,
                    &guard.patterns()[pattern_index].tracks()[track_index as usize],
                    out,
                ),
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index_or_enum_value(4, atoms)? {
                        AtomValue::Symbol(action_or_enum_type) => {
                            let trig = &guard.patterns()[pattern_index].tracks()
                                [track_index as usize]
                                .trigs()[trig_index as usize];

                            // Check if it is a plock action first
                            let action_or_enum_type_str = action_or_enum_type.to_string()?;
                            if ALL_PLOCK_TYPES.contains(&action_or_enum_type_str.as_str()) {
                                return handle_trig_plock_getter_action(
                                    trig,
                                    &action_or_enum_type_str,
                                    atoms,
                                    5,
                                    out,
                                );
                            }

                            trig_get(action_or_enum_type, trig, out)
                        }
                        _ => Err(GetError::InvalidPatternGetterFormat(string_from_atom_slice(
                            atoms,
                        ))
                        .into()),
                    }
                }
                _ => {
                    Err(GetError::InvalidPatternGetterFormat(string_from_atom_slice(atoms)).into())
                }
            }
        }
        _ => Err(GetError::InvalidPatternGetterFormat(string_from_atom_slice(atoms)).into()),
    }
}
