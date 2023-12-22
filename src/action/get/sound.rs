use crate::api::sound_action_type::*;
use crate::api::sound_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use median::atom::Atom;
use median::atom::AtomValue;
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::Sound;
use std::convert::TryFrom;
use std::ffi::CString;

pub fn handle_sound_get_enum_value(
    sound: &Sound,
    enum_type: &str,
    enum_value: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        MACHINE_TYPE => sound.machine_type().into(),
        LFO_DESTINATION => sound.lfo().destination().into(),

        VELOCITY_MOD_TARGET => match enum_value.parse::<usize>().map_err(|_| {
            RytmExternalError::from("Invalid getter format: velmodtarget:<integer> is the correct format. Example: velmodtarget:2")
        })? {
            0 => sound.settings().velocity_modulation_target_1().into(),
            1 => sound.settings().velocity_modulation_target_2().into(),
            2 => sound.settings().velocity_modulation_target_3().into(),
            3 => sound.settings().velocity_modulation_target_4().into(),
            other => {
                return Err(format!(
                    "Invalid range: The index {other} is out of range for velmodtarget."
                )
                .into())
            }
        },
        AFTER_TOUCH_MOD_TARGET => match enum_value.parse::<usize>().map_err(|_| {
            RytmExternalError::from("Invalid getter format: atmodtarget:<integer> is the correct format. Example: atmodtarget:2")
        })? {
            0 => sound.settings().after_touch_modulation_target_1().into(),
            1 => sound.settings().after_touch_modulation_target_2().into(),
            2 => sound.settings().after_touch_modulation_target_3().into(),
            3 => sound.settings().after_touch_modulation_target_4().into(),
            other => {
                return Err(format!(
                    "Invalid range: The index {other} is out of range for atmodtarget."
                )
                .into())
            }
        },
        FILTER_TYPE => sound.filter().filter_type().into(),
        LFO_MULTIPLIER => sound.lfo().multiplier().into(),
        LFO_WAVEFORM => sound.lfo().waveform().into(),
        LFO_MODE => sound.lfo().mode().into(),
        SOUND_SETTINGS_CHROMATIC_MODE => sound.settings().chromatic_mode().into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(sound.index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(&[index_atom, enum_type_atom, enum_value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_sound_get_action(
    sound: &Sound,
    action: &str,
    maybe_index_atom: Option<&Atom>,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value_atom: Atom = match action {
        NAME => Atom::from(SymbolRef::from(CString::new(sound.name()).unwrap())),
        ACCENT_LEVEL => (sound.accent_level() as isize).into(),
        // TODO: MACHINE
        // MACHINE => (sound.machine() as isize).into(),
        AMP_ATTACK => (sound.amplitude().attack() as isize).into(),
        AMP_HOLD => (sound.amplitude().hold() as isize).into(),

        AMP_DECAY => (sound.amplitude().decay() as isize).into(),
        AMP_OVERDRIVE => (sound.amplitude().overdrive() as isize).into(),
        AMP_DELAY_SEND => (sound.amplitude().delay_send() as isize).into(),
        AMP_REVERB_SEND => (sound.amplitude().reverb_send() as isize).into(),
        AMP_PAN => (sound.amplitude().pan() as isize).into(),
        AMP_VOLUME => (sound.amplitude().volume() as isize).into(),
        FILT_ATTACK => (sound.filter().attack() as isize).into(),
        FILT_HOLD => (sound.filter().sustain() as isize).into(),
        FILT_DECAY => (sound.filter().decay() as isize).into(),
        FILT_RELEASE => (sound.filter().release() as isize).into(),
        FILT_CUTOFF => (sound.filter().cutoff() as isize).into(),
        FILT_RESONANCE => (sound.filter().resonance() as isize).into(),
        FILT_ENVELOPE_AMOUNT => (sound.filter().envelope_amount()).into(),
        LFO_SPEED => (sound.lfo().speed()).into(),
        LFO_FADE => (sound.lfo().fade()).into(),
        LFO_START_PHASE_OR_SLEW => (sound.lfo().start_phase_or_slew() as isize).into(),
        LFO_DEPTH => f64::from(sound.lfo().depth()).into(),
        SAMP_TUNE => (sound.sample().tune()).into(),
        SAMP_FINE_TUNE => (sound.sample().fine_tune()).into(),
        SAMP_NUMBER => (sound.sample().slice_number() as isize).into(),
        SAMP_BIT_REDUCTION => (sound.sample().bit_reduction() as isize).into(),
        SAMP_START => f64::from(sound.sample().start()).into(),
        SAMP_END => f64::from(sound.sample().end()).into(),
        SAMP_LOOP_FLAG => isize::from(sound.sample().loop_flag()).into(),
        SAMP_VOLUME => (sound.sample().volume() as isize).into(),

        VEL_MOD_AMT => {
            let index = maybe_index_atom.ok_or_else(|| {
                RytmExternalError::from(
                    "Invalid getter format: velmodamt should be followed by an index.",
                )
            })?;
            let index = index.get_int() as usize;
            match index {
                0 => (sound.settings().velocity_modulation_amt_1()).into(),
                1 => (sound.settings().velocity_modulation_amt_2()).into(),
                2 => (sound.settings().velocity_modulation_amt_3()).into(),
                3 => (sound.settings().velocity_modulation_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for velmodamt."
                    )
                    .into())
                }
            }
        }

        AT_MOD_AMT => {
            let index = maybe_index_atom.ok_or_else(|| {
                RytmExternalError::from(
                    "Invalid getter format: atmodamt should be followed by an integer index.",
                )
            })?;
            let index = index.get_int() as usize;
            match index {
                0 => (sound.settings().after_touch_modulation_amt_1()).into(),
                1 => (sound.settings().after_touch_modulation_amt_2()).into(),
                2 => (sound.settings().after_touch_modulation_amt_3()).into(),
                3 => (sound.settings().after_touch_modulation_amt_4()).into(),
                other => {
                    return Err(format!(
                        "Invalid range: The index {other} is out of range for atmodamt."
                    )
                    .into())
                }
            }
        }

        ENV_RESET_FILTER => isize::from(sound.settings().env_reset_filter()).into(),
        VELOCITY_TO_VOLUME => isize::from(sound.settings().velocity_to_volume()).into(),
        LEGACY_FX_SEND => isize::from(sound.settings().legacy_fx_send()).into(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let action_atom = Atom::from(SymbolRef::from(CString::new(action).unwrap()));
    let index_atom = Atom::from(AtomValue::Int(sound.index() as isize));
    if let Err(_stack_overflow_err) = out.send(&[index_atom, action_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
