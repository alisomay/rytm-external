use crate::{
    action::{
        get::{
            handle_get_action, pattern::PatternGetAction, track::TrackGetAction,
            trig::TrigGetAction, GetAction,
        },
        set::{
            handle_set_action, pattern::PatternSetAction, track::TrackSetAction,
            trig::TrigSetAction, SetAction,
        },
    },
    error::RytmExternalError,
    rytm::Rytm,
    traits::Post,
};
use median::{
    atom::{Atom, AtomValue},
    object::MaxObj,
};

use crate::util::{
    try_get_action_value_from_atom_slice, try_get_atom_value_assuming_action_or_index,
};

pub fn handle_pattern_wb_set(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    match try_get_atom_value_assuming_action_or_index(1, atoms)? {
        AtomValue::Symbol(action) => handle_set_action(SetAction::Pattern(PatternSetAction {
            pattern: rytm.project.lock().unwrap().work_buffer_mut().pattern_mut(),
            action,
            parameter: try_get_action_value_from_atom_slice(2, atoms)?,
        })),
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index(2, atoms)? {
                AtomValue::Symbol(action) => handle_set_action(SetAction::Track(TrackSetAction {
                    track: &mut rytm
                        .project
                        .lock()
                        .unwrap()
                        .work_buffer_mut()
                        .pattern_mut()
                        .tracks_mut()[track_index as usize],
                    action,
                    parameter: try_get_action_value_from_atom_slice(3, atoms)?,
                })),
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                        AtomValue::Symbol(action) => {
                            handle_set_action(SetAction::Trig(TrigSetAction {
                                trig: &mut rytm
                                    .project
                                    .lock()
                                    .unwrap()
                                    .work_buffer_mut()
                                    .pattern_mut()
                                    .tracks_mut()[track_index as usize]
                                    .trigs_mut()[trig_index as usize],
                                action,
                                parameter: try_get_action_value_from_atom_slice(4, atoms)?,
                            }))
                        }
                        _ => Err("Only symbols and integers are allowed in setters.".into()),
                    }
                }
                _ => Err("Only symbols and integers are allowed in setters.".into()),
            }
        }
        _ => Err("Only symbols and integers are allowed in setters.".into()),
    }
}

pub fn handle_pattern_wb_get(rytm: &Rytm, atoms: &[Atom]) -> Result<(), RytmExternalError> {
    match try_get_atom_value_assuming_action_or_index(1, atoms)? {
        AtomValue::Symbol(action) => handle_get_action(GetAction::Pattern(PatternGetAction {
            pattern: rytm.project.lock().unwrap().work_buffer_mut().pattern_mut(),
            action,
            out: &rytm.query_out,
        })),
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index(2, atoms)? {
                AtomValue::Symbol(action) => handle_get_action(GetAction::Track(TrackGetAction {
                    track: &rytm
                        .project
                        .lock()
                        .unwrap()
                        .work_buffer()
                        .pattern()
                        .tracks()[track_index as usize],
                    action,
                    out: &rytm.query_out,
                })),
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                        AtomValue::Symbol(action) => {
                            handle_get_action(GetAction::Trig(TrigGetAction {
                                trig: &rytm
                                    .project
                                    .lock()
                                    .unwrap()
                                    .work_buffer()
                                    .pattern()
                                    .tracks()[track_index as usize]
                                    .trigs()[trig_index as usize],
                                action,
                                out: &rytm.query_out,
                            }))
                        }
                        _ => Err("Only symbols and integers are allowed in setters.".into()),
                    }
                }
                _ => Err("Only symbols and integers are allowed in setters.".into()),
            }
        }
        _ => Err("Only symbols and integers are allowed in setters.".into()),
    }
}
