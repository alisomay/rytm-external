use crate::api::sound_action_type::*;
use crate::api::sound_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use crate::util::get_bool_from_0_or_1;
use crate::util::only_allow_numbers_as_action_parameter;
use median::atom::Atom;
use median::atom::AtomValue;
use median::symbol::SymbolRef;
use rytm_rs::object::Sound;
use std::convert::TryFrom;
use std::convert::TryInto;

pub fn handle_sound_set_enum_value(
    sound: &mut Sound,
    enum_type: &str,
    enum_value: &str,
    maybe_parameter_atom: Option<&Atom>,
) -> Result<(), RytmExternalError> {
    match enum_type {
        // TODO:
        // MACHINE_PARAMETERS => {
        //     sound.set_selected_machine_parameter(enum_value.try_into()?);
        //     Ok(())
        // }
        MACHINE_TYPE => {
            sound.set_machine_type(enum_value.try_into()?);
            Ok(())
        }
        LFO_DESTINATION => {
            sound.lfo_mut().set_destination(enum_value.try_into()?);
            Ok(())
        }

        VELOCITY_MOD_TARGET => {
            if let Some(atom) = maybe_parameter_atom {
                if let Some(AtomValue::Int(index)) = atom.get_value() {
                    match index {
                        0 => sound
                            .settings_mut()
                            .set_velocity_modulation_target_1(enum_value.try_into()?),
                        1 => sound
                            .settings_mut()
                            .set_velocity_modulation_target_2(enum_value.try_into()?),
                        2 => sound
                            .settings_mut()
                            .set_velocity_modulation_target_3(enum_value.try_into()?),
                        3 => sound
                            .settings_mut()
                            .set_velocity_modulation_target_4(enum_value.try_into()?),
                        _ => {
                            return Err(
                                "velmod index parameter index must be between 0 and 3.".into()
                            )
                        }
                    }
                    return Ok(());
                } else {
                    return Err("Invalid setter format: velmodtarget should be followed by an integer velmod index. Format: velmodtarget:<target> <velmod index>. Example: velmodtarget:lfophase 2".into());
                }
            }
            return Err("Invalid setter format: velmodtarget should be followed by an integer velmod index. Format: velmodtarget:<target> <velmod index>. Example: velmodtarget:lfophase 2".into());
        }
        AFTER_TOUCH_MOD_TARGET => {
            if let Some(atom) = maybe_parameter_atom {
                if let Some(AtomValue::Int(index)) = atom.get_value() {
                    match index {
                        0 => sound
                            .settings_mut()
                            .set_after_touch_modulation_target_1(enum_value.try_into()?),
                        1 => sound
                            .settings_mut()
                            .set_after_touch_modulation_target_2(enum_value.try_into()?),
                        2 => sound
                            .settings_mut()
                            .set_after_touch_modulation_target_3(enum_value.try_into()?),
                        3 => sound
                            .settings_mut()
                            .set_after_touch_modulation_target_4(enum_value.try_into()?),
                        _ => {
                            return Err(
                                "atmod index parameter index must be between 0 and 3.".into()
                            )
                        }
                    }
                    return Ok(());
                } else {
                    return Err("Invalid setter format: atmodtarget should be followed by an integer atmod index. Format: atmodtarget:<target> <atmod index>. Example: atmodtarget:lfophase 2".into());
                }
            }
            return Err("Invalid setter format: atmodtarget should be followed by an integer atmod index. Format: atmodtarget:<target> <atmod index>. Example: atmodtarget:lfophase 2".into());
        }
        FILTER_TYPE => {
            sound.filter_mut().set_filter_type(enum_value.try_into()?);
            Ok(())
        }
        LFO_MULTIPLIER => {
            sound.lfo_mut().set_multiplier(enum_value.try_into()?);
            Ok(())
        }
        LFO_WAVEFORM => {
            sound.lfo_mut().set_waveform(enum_value.try_into()?);
            Ok(())
        }
        LFO_MODE => {
            sound.lfo_mut().set_mode(enum_value.try_into()?);
            Ok(())
        }
        SOUND_SETTINGS_CHROMATIC_MODE => {
            sound
                .settings_mut()
                .set_chromatic_mode(enum_value.try_into()?);
            Ok(())
        }
        other => Err(InvalidEnumType(other.to_string()).into()),
    }
}

pub fn handle_sound_set_action(
    sound: &mut Sound,
    action_or_enum_value_str: &str,
    parameter_atom: &Atom,
    maybe_next_atom: Option<&Atom>,
) -> Result<(), RytmExternalError> {
    let action_or_enum_value = SymbolRef::try_from(action_or_enum_value_str)?;
    let action_or_enum_value_str = action_or_enum_value.to_string()?;

    if action_or_enum_value_str.as_str() == NAME {
        match parameter_atom.get_value().unwrap() {
            AtomValue::Symbol(symbol) => return Ok(sound.set_name(symbol.to_string()?.as_str())?),
            _ => return Err("Invalid value: Name must be a symbol with maximum 15 characters long and use only ascii characters.".into()),
        }
    }

    only_allow_numbers_as_action_parameter(parameter_atom)?;

    match action_or_enum_value_str.as_str() {
        ACCENT_LEVEL => Ok(sound.set_accent_level(parameter_atom.get_int() as usize)?),
        // TODO:
        // MACHINE => Ok(sound.set_machine(parameter_atom.get_int() as usize)?),
        AMP_ATTACK => Ok(sound
            .amplitude_mut()
            .set_attack(parameter_atom.get_int() as usize)?),
        AMP_HOLD => Ok(sound
            .amplitude_mut()
            .set_hold(parameter_atom.get_int() as usize)?),
        AMP_DECAY => Ok(sound
            .amplitude_mut()
            .set_decay(parameter_atom.get_int() as usize)?),
        AMP_OVERDRIVE => Ok(sound
            .amplitude_mut()
            .set_overdrive(parameter_atom.get_int() as usize)?),
        AMP_DELAY_SEND => Ok(sound
            .amplitude_mut()
            .set_delay_send(parameter_atom.get_int() as usize)?),
        AMP_REVERB_SEND => Ok(sound
            .amplitude_mut()
            .set_reverb_send(parameter_atom.get_int() as usize)?),
        AMP_PAN => Ok(sound.amplitude_mut().set_pan(parameter_atom.get_int())?),
        AMP_VOLUME => Ok(sound
            .amplitude_mut()
            .set_volume(parameter_atom.get_int() as usize)?),
        FILT_ATTACK => Ok(sound
            .filter_mut()
            .set_attack(parameter_atom.get_int() as usize)?),
        FILT_HOLD => Ok(sound
            .filter_mut()
            .set_sustain(parameter_atom.get_int() as usize)?),
        FILT_DECAY => Ok(sound
            .filter_mut()
            .set_decay(parameter_atom.get_int() as usize)?),
        FILT_RELEASE => Ok(sound
            .filter_mut()
            .set_release(parameter_atom.get_int() as usize)?),
        FILT_CUTOFF => Ok(sound
            .filter_mut()
            .set_cutoff(parameter_atom.get_int() as usize)?),
        FILT_RESONANCE => Ok(sound
            .filter_mut()
            .set_resonance(parameter_atom.get_int() as usize)?),
        FILT_ENVELOPE_AMOUNT => Ok(sound
            .filter_mut()
            .set_envelope_amount(parameter_atom.get_int())?),
        LFO_SPEED => Ok(sound.lfo_mut().set_speed(parameter_atom.get_int())?),
        LFO_FADE => Ok(sound.lfo_mut().set_fade(parameter_atom.get_int())?),
        LFO_START_PHASE_OR_SLEW => Ok(sound
            .lfo_mut()
            .set_start_phase(parameter_atom.get_int() as usize)?),
        LFO_DEPTH => Ok(sound
            .lfo_mut()
            .set_depth(parameter_atom.get_float() as f32)?),
        SAMP_TUNE => Ok(sound.sample_mut().set_tune(parameter_atom.get_int())?),
        SAMP_FINE_TUNE => Ok(sound.sample_mut().set_fine_tune(parameter_atom.get_int())?),
        SAMP_NUMBER => Ok(sound
            .sample_mut()
            .set_slice_number(parameter_atom.get_int() as usize)?),
        SAMP_BIT_REDUCTION => Ok(sound
            .sample_mut()
            .set_bit_reduction(parameter_atom.get_int() as usize)?),
        SAMP_START => Ok(sound
            .sample_mut()
            .set_start(parameter_atom.get_float() as f32)?),
        SAMP_END => Ok(sound
            .sample_mut()
            .set_end(parameter_atom.get_float() as f32)?),
        SAMP_LOOP_FLAG => {
            sound
                .sample_mut()
                .set_loop_flag(get_bool_from_0_or_1(parameter_atom)?);
            Ok(())
        }
        SAMP_VOLUME => Ok(sound
            .sample_mut()
            .set_volume(parameter_atom.get_int() as usize)?),

        VEL_MOD_AMT => {
            if let Some(next_atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = next_atom.get_value() {
                    // TODO: Check if this creates a bug
                    match parameter_atom.get_int() {
                        0 => sound.settings_mut().set_velocity_modulation_amt_1(value)?,
                        1 => sound.settings_mut().set_velocity_modulation_amt_2(value)?,
                        2 => sound.settings_mut().set_velocity_modulation_amt_3(value)?,
                        3 => sound.settings_mut().set_velocity_modulation_amt_4(value)?,
                        _ => {
                            return Err(
                                "velmod index parameter index must be between 0 and 3.".into()
                            )
                        }
                    }
                    return Ok(());
                } else {
                    return Err("Invalid setter format: velmodamt should be followed by an integer velmod index. Format: velmodamt <velmod index> <amount>. Example: velmodamt 2 100".into());
                }
            }
            Err("Invalid setter format: velmodamt should be followed by an integer velmod index. Format: velmodamt <velmod index> <amount>. Example: velmodamt 2 100".into())
        }

        AT_MOD_AMT => {
            if let Some(next_atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = next_atom.get_value() {
                    // TODO: Check if this creates a bug
                    match parameter_atom.get_int() {
                        0 => sound
                            .settings_mut()
                            .set_after_touch_modulation_amt_1(value)?,
                        1 => sound
                            .settings_mut()
                            .set_after_touch_modulation_amt_2(value)?,
                        2 => sound
                            .settings_mut()
                            .set_after_touch_modulation_amt_3(value)?,
                        3 => sound
                            .settings_mut()
                            .set_after_touch_modulation_amt_4(value)?,
                        _ => {
                            return Err(
                                "atmod index parameter index must be between 0 and 3.".into()
                            )
                        }
                    }
                    return Ok(());
                } else {
                    return Err("Invalid setter format: atmodamt should be followed by an integer atmod index. Format: atmodamt <atmod index> <amount>. Example: atmodamt 2 100".into());
                }
            }
            Err("Invalid setter format: atmodamt should be followed by an integer atmod index. Format: atmodamt <atmod index> <amount>. Example: atmodamt 2 100".into())
        }

        other => Err(InvalidActionType(other.to_string()).into()),
    }
}
