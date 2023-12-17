use crate::{
    action::set::{
        handle_set_action, pattern::PatternSetAction, track::TrackSetAction, trig::TrigSetAction,
        SetAction,
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

pub fn handle_pattern_set(
    rytm: &Rytm,
    atoms: &[Atom],
    pattern_index: usize,
) -> Result<(), RytmExternalError> {
    if !(0..=127).contains(&pattern_index) {
        "Pattern index must be an integer between 0 and 127".obj_error(rytm.max_obj());
    }
    match try_get_atom_value_assuming_action_or_index(2, atoms)? {
        AtomValue::Symbol(action) => handle_set_action(SetAction::Pattern(PatternSetAction {
            pattern: rytm
                .project
                .lock()
                .unwrap()
                .patterns_mut()
                .get_mut(pattern_index)
                .unwrap(),
            action,
            parameter: try_get_action_value_from_atom_slice(3, atoms)?,
        })),
        AtomValue::Int(track_index) => {
            if !(0..=12).contains(&track_index) {
                "Track index must be an integer between 0 and 12".obj_error(rytm.max_obj());
            }
            match try_get_atom_value_assuming_action_or_index(3, atoms)? {
                AtomValue::Symbol(action) => handle_set_action(SetAction::Track(TrackSetAction {
                    track: &mut rytm.project.lock().unwrap().patterns_mut()[pattern_index]
                        .tracks_mut()[track_index as usize],
                    action,
                    parameter: try_get_action_value_from_atom_slice(4, atoms)?,
                })),
                AtomValue::Int(trig_index) => {
                    if !(0..=63).contains(&track_index) {
                        "Trig index must be an integer between 0 and 63".obj_error(rytm.max_obj());
                    }
                    match try_get_atom_value_assuming_action_or_index(4, atoms)? {
                        AtomValue::Symbol(action) => {
                            handle_set_action(SetAction::Trig(TrigSetAction {
                                trig: &mut rytm.project.lock().unwrap().patterns_mut()
                                    [pattern_index]
                                    .tracks_mut()[track_index as usize]
                                    .trigs_mut()[trig_index as usize],
                                action,
                                parameter: try_get_action_value_from_atom_slice(5, atoms)?,
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
