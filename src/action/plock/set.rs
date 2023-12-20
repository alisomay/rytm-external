use std::convert::TryInto;

use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::{error::RytmExternalError, util::get_bool_from_0_or_1};
use median::{atom::Atom, symbol::SymbolRef};
use rytm_rs::object::pattern::Trig;

pub fn handle_trig_plock_set_action(
    trig: &mut Trig,
    action: SymbolRef,
    parameter: &Atom,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;

    use crate::api::kit_action_type;
    use crate::api::sound_action_type;
    match action_str.as_str() {
        kit_action_type::FX_DELAY_TIME => {
            Ok(trig.plock_set_fx_delay_time(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_DELAY_PING_PONG => {
            Ok(trig.plock_set_fx_delay_ping_pong(get_bool_from_0_or_1(parameter)?)?)
        }
        kit_action_type::FX_DELAY_STEREO_WIDTH => {
            Ok(trig.plock_set_fx_delay_stereo_width(parameter.get_int())?)
        }
        kit_action_type::FX_DELAY_FEEDBACK => {
            Ok(trig.plock_set_fx_delay_feedback(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_DELAY_HPF => {
            Ok(trig.plock_set_fx_delay_hpf(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_DELAY_LPF => {
            Ok(trig.plock_set_fx_delay_lpf(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_DELAY_REVERB_SEND => {
            Ok(trig.plock_set_fx_delay_reverb_send(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_DELAY_VOLUME => {
            Ok(trig.plock_set_fx_delay_volume(parameter.get_int() as usize)?)
        }

        kit_action_type::FX_REVERB_PRE_DELAY => {
            Ok(trig.plock_set_fx_reverb_pre_delay(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_DECAY => {
            Ok(trig.plock_set_fx_reverb_decay(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_FREQ => {
            Ok(trig.plock_set_fx_reverb_freq(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_GAIN => {
            Ok(trig.plock_set_fx_reverb_gain(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_HPF => {
            Ok(trig.plock_set_fx_reverb_hpf(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_LPF => {
            Ok(trig.plock_set_fx_reverb_lpf(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_REVERB_VOLUME => {
            Ok(trig.plock_set_fx_reverb_volume(parameter.get_int() as usize)?)
        }

        kit_action_type::FX_COMP_THRESHOLD => {
            Ok(trig.plock_set_fx_compressor_threshold(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_COMP_GAIN => {
            Ok(trig.plock_set_fx_compressor_gain(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_COMP_MIX => {
            Ok(trig.plock_set_fx_compressor_mix(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_COMP_VOLUME => {
            Ok(trig.plock_set_fx_compressor_volume(parameter.get_int() as usize)?)
        }

        kit_action_type::FX_LFO_SPEED => Ok(trig.plock_set_fx_lfo_speed(parameter.get_int())?),
        kit_action_type::FX_LFO_FADE => Ok(trig.plock_set_fx_lfo_fade(parameter.get_int())?),
        kit_action_type::FX_LFO_START_PHASE_OR_SLEW => {
            Ok(trig.plock_set_fx_lfo_start_phase(parameter.get_int() as usize)?)
        }
        kit_action_type::FX_LFO_DEPTH => {
            Ok(trig.plock_set_fx_lfo_depth(parameter.get_float() as f32)?)
        }
        //
        // TODO: Do the dist setters after fixing the dist in the SDK
        // TODO: Do Machine plocks
        //
        sound_action_type::AMP_ATTACK => {
            Ok(trig.plock_set_amplitude_attack(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_HOLD => {
            Ok(trig.plock_set_amplitude_hold(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_DECAY => {
            Ok(trig.plock_set_amplitude_decay(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_OVERDRIVE => {
            Ok(trig.plock_set_amplitude_overdrive(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_DELAY_SEND => {
            Ok(trig.plock_set_amplitude_delay_send(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_REVERB_SEND => {
            Ok(trig.plock_set_amplitude_reverb_send(parameter.get_int() as usize)?)
        }
        sound_action_type::AMP_PAN => Ok(trig.plock_set_amplitude_pan(parameter.get_int())?),
        sound_action_type::AMP_VOLUME => {
            Ok(trig.plock_set_amplitude_volume(parameter.get_int() as usize)?)
        }

        sound_action_type::FILT_ATTACK => {
            Ok(trig.plock_set_filter_attack(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_HOLD => {
            Ok(trig.plock_set_filter_sustain(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_DECAY => {
            Ok(trig.plock_set_filter_decay(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_RELEASE => {
            Ok(trig.plock_set_filter_release(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_CUTOFF => {
            Ok(trig.plock_set_filter_cutoff(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_RESONANCE => {
            Ok(trig.plock_set_filter_resonance(parameter.get_int() as usize)?)
        }
        sound_action_type::FILT_ENVELOPE_AMOUNT => {
            Ok(trig.plock_set_filter_envelope_amount(parameter.get_int())?)
        }

        sound_action_type::LFO_SPEED => Ok(trig.plock_set_lfo_speed(parameter.get_int())?),
        sound_action_type::LFO_FADE => Ok(trig.plock_set_lfo_fade(parameter.get_int())?),
        sound_action_type::LFO_START_PHASE_OR_SLEW => {
            Ok(trig.plock_set_lfo_start_phase(parameter.get_int() as usize)?)
        }
        sound_action_type::LFO_DEPTH => Ok(trig.plock_set_lfo_depth(parameter.get_float() as f32)?),

        sound_action_type::SAMP_TUNE => Ok(trig.plock_set_sample_tune(parameter.get_int())?),
        sound_action_type::SAMP_FINE_TUNE => {
            Ok(trig.plock_set_sample_fine_tune(parameter.get_int())?)
        }
        sound_action_type::SAMP_NUMBER => {
            Ok(trig.plock_set_sample_number(parameter.get_int() as usize)?)
        }
        sound_action_type::SAMP_BIT_REDUCTION => {
            Ok(trig.plock_set_sample_bit_reduction(parameter.get_int() as usize)?)
        }
        sound_action_type::SAMP_START => {
            Ok(trig.plock_set_sample_start(parameter.get_float() as f32)?)
        }
        sound_action_type::SAMP_END => Ok(trig.plock_set_sample_end(parameter.get_float() as f32)?),
        sound_action_type::SAMP_LOOP_FLAG => {
            Ok(trig.plock_set_sample_loop_flag(get_bool_from_0_or_1(parameter)?)?)
        }
        sound_action_type::SAMP_VOLUME => {
            Ok(trig.plock_set_sample_volume(parameter.get_int() as usize)?)
        }

        other => Err(InvalidActionType(other.to_string()).into()),
    }
}

pub fn handle_trig_plock_set_enum_value(
    trig: &mut Trig,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    use crate::api::kit_enum_type;
    use crate::api::sound_enum_type;

    match enum_type {
        kit_enum_type::CONTROL_IN_MOD_TARGET => todo!(),
        kit_enum_type::FX_COMP_ATTACK => {
            Ok(trig.plock_set_fx_compressor_attack(enum_value.try_into()?)?)
        }
        kit_enum_type::FX_COMP_RELEASE => {
            Ok(trig.plock_set_fx_compressor_release(enum_value.try_into()?)?)
        }
        kit_enum_type::FX_DELAY_TIME_ON_THE_GRID => todo!(),
        kit_enum_type::FX_COMP_RATIO => {
            Ok(trig.plock_set_fx_compressor_ratio(enum_value.try_into()?)?)
        }
        kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => {
            Ok(trig.plock_set_fx_compressor_side_chain_eq(enum_value.try_into()?)?)
        }
        kit_enum_type::FX_LFO_DESTINATION => {
            Ok(trig.plock_set_fx_lfo_destination(enum_value.try_into()?)?)
        }

        // TODO:
        // sound_enum_type::MACHINE_PARAMETERS => todo!(),
        sound_enum_type::LFO_DESTINATION => {
            Ok(trig.plock_set_lfo_destination(enum_value.try_into()?)?)
        }
        sound_enum_type::FILTER_TYPE => Ok(trig.plock_set_filter_type(enum_value.try_into()?)?),
        sound_enum_type::LFO_MULTIPLIER => {
            Ok(trig.plock_set_lfo_multiplier(enum_value.try_into()?)?)
        }
        sound_enum_type::LFO_WAVEFORM => Ok(trig.plock_set_lfo_waveform(enum_value.try_into()?)?),
        sound_enum_type::LFO_MODE => Ok(trig.plock_set_lfo_mode(enum_value.try_into()?)?),

        other => Err(InvalidEnumType(other.to_string()).into()),
    }
}
