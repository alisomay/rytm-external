use crate::api::settings_action_type::*;
use crate::api::settings_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use median::atom::Atom;
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::Settings;
use std::convert::TryFrom;
use std::ffi::CString;

pub fn handle_settings_get_enum_value(
    settings: &Settings,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        PARAMETER_MENU_ITEM => settings.selected_parameter_menu_item().into(),
        FX_PARAMETER_MENU_ITEM => settings.selected_fx_menu_item().into(),
        SEQUENCER_MODE => settings.selected_mode().into(),
        PATTERN_MODE => settings.selected_pattern_mode().into(),
        SAMPLE_RECORDER_SOURCE => settings.sample_recorder_source().into(),
        SAMPLE_RECORDER_RECORDING_LENGTH => settings.sample_recorder_recording_length().into(),
        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) =
        out.send(&[Atom::from(0isize), enum_type_atom, enum_value_atom][..])
    {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_settings_get_action(
    settings: &Settings,
    action: &str,
    maybe_next_atom: Option<&Atom>,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value_atom: Atom = match action {
        BPM_PROJECT => Atom::from(f64::from(settings.bpm())),
        SELECTED_TRACK => Atom::from(settings.selected_track() as isize),
        SELECTED_PAGE => Atom::from(settings.selected_page() as isize),
        MUTE => {
            let index = maybe_next_atom
                .ok_or("Invalid getter format: mute should be followed by an integer sound index.")?
                .get_int();
            Atom::from(isize::from(settings.is_sound_muted(index as usize)?))
        }
        FIXED_VELOCITY_ENABLE => Atom::from(isize::from(settings.fixed_velocity_enabled())),
        FIXED_VELOCITY_AMOUNT => Atom::from(settings.fixed_velocity_amount() as isize),
        SAMPLE_RECORDER_THR => Atom::from(settings.sample_recorder_threshold() as isize),
        SAMPLE_RECORDER_MONITOR_ENABLE => {
            Atom::from(isize::from(settings.sample_recorder_monitor_enabled()))
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let action_atom = Atom::from(SymbolRef::from(CString::new(action).unwrap()));
    if let Err(_stack_overflow_err) = out.send(&[Atom::from(0isize), action_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
