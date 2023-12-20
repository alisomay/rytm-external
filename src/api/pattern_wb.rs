use crate::{
    action::{
        get::{pattern::pattern_get, track::track_get, trig::trig_get},
        plock::{handle_trig_plock_getter_action, handle_trig_plock_setter_action},
        set::{pattern::pattern_set, track::track_set, trig::trig_set},
    },
    error::RytmExternalError,
    rytm::Rytm,
    traits::Post,
};
use median::{
    atom::{Atom, AtomValue},
    object::MaxObj,
};

use super::plock_type::ALL_PLOCK_TYPES;
use crate::util::try_get_atom_value_assuming_action_or_index_or_enum_value;

const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

// Format is set <object type> ...

// Patterns
// Format is set pattern <pattern-index> <track-index> <trig-index> <action> <value>
// Other formats are set pattern <pattern-index> <action> <value>
// set pattern <pattern-index> <track-index> <action> <value>
// set pattern <pattern-index> <track-index> <trig-index> <action> <value>

pub fn handle_pattern_wb_set(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let mut guard = rytm.project.lock().unwrap();

    match try_get_atom_value_assuming_action_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_value) => {
            // Check the next value and finish the list.
            pattern_set(
                action_or_enum_value,
                guard.work_buffer_mut().pattern_mut(),
                atoms,
                2,
            )
        }
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
                AtomValue::Symbol(action_or_enum_value) => {
                    // Check the next value and finish the list.
                    track_set(
                        action_or_enum_value,
                        &mut guard.work_buffer_mut().pattern_mut().tracks_mut()
                            [track_index as usize],
                        atoms,
                        3,
                    )
                }
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index_or_enum_value(3, atoms)? {
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
                                    4,
                                );
                            }

                            trig_set(action_or_enum_value, trig_mut, atoms, 4)
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

pub fn handle_pattern_wb_get(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    let guard = rytm.project.lock().unwrap();
    let out = &rytm.query_out;

    match try_get_atom_value_assuming_action_or_index_or_enum_value(1, atoms)? {
        AtomValue::Symbol(action_or_enum_type) => {
            pattern_get(action_or_enum_type, guard.work_buffer().pattern(), out)
        }
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index_or_enum_value(2, atoms)? {
                AtomValue::Symbol(action_or_enum_type) => track_get(
                    action_or_enum_type,
                    &guard.work_buffer().pattern().tracks()[track_index as usize],
                    out,
                ),
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index_or_enum_value(3, atoms)? {
                        AtomValue::Symbol(action_or_enum_type) => {
                            let trig = &guard.work_buffer().pattern().tracks()
                                [track_index as usize]
                                .trigs()[trig_index as usize];

                            // Check if it is a plock action first
                            let action_or_enum_type_str = action_or_enum_type.to_string()?;
                            if ALL_PLOCK_TYPES.contains(&action_or_enum_type_str.as_str()) {
                                return handle_trig_plock_getter_action(
                                    trig,
                                    &action_or_enum_type_str,
                                    atoms,
                                    4,
                                    out,
                                );
                            }

                            trig_get(action_or_enum_type, trig, out)
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
