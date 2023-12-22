use crate::api::kit_action_type;
use crate::api::sound_action_type;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use median::symbol::SymbolRef;
use rytm_rs::object::pattern::Trig;

pub fn handle_trig_plock_clear_action(
    trig: &Trig,
    action: &SymbolRef,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;

    match action_str.as_str() {
        kit_action_type::FX_DELAY_TIME => Ok(trig.plock_clear_fx_delay_time()?),
        kit_action_type::FX_DELAY_PING_PONG => Ok(trig.plock_clear_fx_delay_ping_pong()?),
        kit_action_type::FX_DELAY_STEREO_WIDTH => Ok(trig.plock_clear_fx_delay_stereo_width()?),
        kit_action_type::FX_DELAY_FEEDBACK => Ok(trig.plock_clear_fx_delay_feedback()?),
        kit_action_type::FX_DELAY_HPF => Ok(trig.plock_clear_fx_delay_hpf()?),
        kit_action_type::FX_DELAY_LPF => Ok(trig.plock_clear_fx_delay_lpf()?),
        kit_action_type::FX_DELAY_REVERB_SEND => Ok(trig.plock_clear_fx_delay_reverb_send()?),
        kit_action_type::FX_DELAY_VOLUME => Ok(trig.plock_clear_fx_delay_volume()?),

        kit_action_type::FX_REVERB_PRE_DELAY => Ok(trig.plock_clear_fx_reverb_pre_delay()?),
        kit_action_type::FX_REVERB_DECAY => Ok(trig.plock_clear_fx_reverb_decay()?),
        kit_action_type::FX_REVERB_FREQ => Ok(trig.plock_clear_fx_reverb_freq()?),
        kit_action_type::FX_REVERB_GAIN => Ok(trig.plock_clear_fx_reverb_gain()?),
        kit_action_type::FX_REVERB_HPF => Ok(trig.plock_clear_fx_reverb_hpf()?),
        kit_action_type::FX_REVERB_LPF => Ok(trig.plock_clear_fx_reverb_lpf()?),
        kit_action_type::FX_REVERB_VOLUME => Ok(trig.plock_clear_fx_reverb_volume()?),

        kit_action_type::FX_COMP_THRESHOLD => Ok(trig.plock_clear_fx_compressor_threshold()?),
        kit_action_type::FX_COMP_GAIN => Ok(trig.plock_clear_fx_compressor_gain()?),
        kit_action_type::FX_COMP_MIX => Ok(trig.plock_clear_fx_compressor_mix()?),
        kit_action_type::FX_COMP_VOLUME => Ok(trig.plock_clear_fx_compressor_volume()?),

        kit_action_type::FX_LFO_SPEED => Ok(trig.plock_clear_fx_lfo_speed()?),
        kit_action_type::FX_LFO_FADE => Ok(trig.plock_clear_fx_lfo_fade()?),
        kit_action_type::FX_LFO_START_PHASE_OR_SLEW => Ok(trig.plock_clear_fx_lfo_start_phase()?),
        kit_action_type::FX_LFO_DEPTH => Ok(trig.plock_clear_fx_lfo_depth()?),
        //
        // TODO: Do the dist setters after fixing the dist in the SDK
        // TODO: MACHINE Do Machine plocks
        //
        sound_action_type::AMP_ATTACK => Ok(trig.plock_clear_amplitude_attack()?),
        sound_action_type::AMP_HOLD => Ok(trig.plock_clear_amplitude_hold()?),
        sound_action_type::AMP_DECAY => Ok(trig.plock_clear_amplitude_decay()?),
        sound_action_type::AMP_OVERDRIVE => Ok(trig.plock_clear_amplitude_overdrive()?),
        sound_action_type::AMP_DELAY_SEND => Ok(trig.plock_clear_amplitude_delay_send()?),
        sound_action_type::AMP_REVERB_SEND => Ok(trig.plock_clear_amplitude_reverb_send()?),
        sound_action_type::AMP_PAN => Ok(trig.plock_clear_amplitude_pan()?),
        sound_action_type::AMP_VOLUME => Ok(trig.plock_clear_amplitude_volume()?),

        sound_action_type::FILT_ATTACK => Ok(trig.plock_clear_filter_attack()?),
        sound_action_type::FILT_HOLD => Ok(trig.plock_clear_filter_sustain()?),
        sound_action_type::FILT_DECAY => Ok(trig.plock_clear_filter_decay()?),
        sound_action_type::FILT_RELEASE => Ok(trig.plock_clear_filter_release()?),
        sound_action_type::FILT_CUTOFF => Ok(trig.plock_clear_filter_cutoff()?),
        sound_action_type::FILT_RESONANCE => Ok(trig.plock_clear_filter_resonance()?),
        sound_action_type::FILT_ENVELOPE_AMOUNT => Ok(trig.plock_clear_filter_envelope_amount()?),

        sound_action_type::LFO_SPEED => Ok(trig.plock_clear_lfo_speed()?),
        sound_action_type::LFO_FADE => Ok(trig.plock_clear_lfo_fade()?),
        sound_action_type::LFO_START_PHASE_OR_SLEW => Ok(trig.plock_clear_lfo_start_phase()?),
        sound_action_type::LFO_DEPTH => Ok(trig.plock_clear_lfo_depth()?),

        sound_action_type::SAMP_TUNE => Ok(trig.plock_clear_sample_tune()?),
        sound_action_type::SAMP_FINE_TUNE => Ok(trig.plock_clear_sample_fine_tune()?),
        sound_action_type::SAMP_NUMBER => Ok(trig.plock_clear_sample_number()?),
        sound_action_type::SAMP_BIT_REDUCTION => Ok(trig.plock_clear_sample_bit_reduction()?),
        sound_action_type::SAMP_START => Ok(trig.plock_clear_sample_start()?),
        sound_action_type::SAMP_END => Ok(trig.plock_clear_sample_end()?),
        sound_action_type::SAMP_LOOP_FLAG => Ok(trig.plock_clear_sample_loop_flag()?),
        sound_action_type::SAMP_VOLUME => Ok(trig.plock_clear_sample_volume()?),

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

pub fn handle_trig_plock_clear_enum_value(
    trig: &Trig,
    enum_type: &str,
) -> Result<(), RytmExternalError> {
    use crate::api::kit_enum_type;
    use crate::api::sound_enum_type;

    match enum_type {
        kit_enum_type::FX_COMP_ATTACK => Ok(trig.plock_clear_fx_compressor_attack()?),
        kit_enum_type::FX_COMP_RELEASE => Ok(trig.plock_clear_fx_compressor_release()?),
        kit_enum_type::FX_COMP_RATIO => Ok(trig.plock_clear_fx_compressor_ratio()?),
        kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => Ok(trig.plock_clear_fx_compressor_side_chain_eq()?),
        kit_enum_type::FX_LFO_DESTINATION => Ok(trig.plock_clear_fx_lfo_destination()?),

        // TODO: MACHINE
        // sound_enum_type::MACHINE_PARAMETERS => todo!(),
        sound_enum_type::LFO_DESTINATION => Ok(trig.plock_clear_lfo_destination()?),
        sound_enum_type::FILTER_TYPE => Ok(trig.plock_clear_filter_type()?),
        sound_enum_type::LFO_MULTIPLIER => Ok(trig.plock_clear_lfo_multiplier()?),
        sound_enum_type::LFO_WAVEFORM => Ok(trig.plock_clear_lfo_waveform()?),
        sound_enum_type::LFO_MODE => Ok(trig.plock_clear_lfo_mode()?),

        other => Err(InvalidEnumType(other.to_owned()).into()),
    }
}
