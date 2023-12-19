use crate::{error::RytmExternalError, rytm::Rytm, traits::Post};
use median::{
    atom::{Atom, AtomValue},
    object::MaxObj,
};

use super::{pattern_get, pattern_set, track_get, track_set, trig_get, trig_set};
use crate::util::try_get_atom_value_assuming_action_or_index_or_enum_value;

const ERR: &str =
    "Invalid value: Only symbols or integers are allowed in pattern setters or getters.";

pub fn handle_pattern_set(
    rytm: &Rytm,
    atoms: &[Atom],
    pattern_index: usize,
) -> Result<(), RytmExternalError> {
    if !(0..=127).contains(&pattern_index) {
        "Pattern index must be an integer between 0 and 127".obj_error(rytm.max_obj());
    }

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
                            // Check the next value and finish the list.
                            trig_set(
                                action_or_enum_value,
                                &mut guard.patterns_mut()[pattern_index].tracks_mut()
                                    [track_index as usize]
                                    .trigs_mut()[trig_index as usize],
                                atoms,
                                5,
                            )
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
    if !(0..=127).contains(&pattern_index) {
        "Pattern index must be an integer between 0 and 127".obj_error(rytm.max_obj());
    }

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
                        AtomValue::Symbol(action_or_enum_type) => trig_get(
                            action_or_enum_type,
                            &guard.patterns()[pattern_index].tracks()[track_index as usize].trigs()
                                [trig_index as usize],
                            out,
                        ),
                        _ => Err(ERR.into()),
                    }
                }
                _ => Err(ERR.into()),
            }
        }
        _ => Err(ERR.into()),
    }
}
