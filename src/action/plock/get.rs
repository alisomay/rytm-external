use std::convert::TryFrom;
use std::ffi::CString;

use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use median::atom::{Atom, AtomValue};
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::pattern::Trig;

pub fn handle_trig_plock_get_action(
    trig: &Trig,
    action: SymbolRef,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;

    use crate::api::kit_action_type;
    use crate::api::sound_action_type;
    let value_atom: Option<Atom> = match action_str.as_str() {
        kit_action_type::FX_DELAY_TIME => trig
            .plock_get_fx_delay_time()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_PING_PONG => trig
            .plock_get_fx_delay_ping_pong()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_STEREO_WIDTH => {
            trig.plock_get_fx_delay_stereo_width()?.map(Atom::from)
        }
        kit_action_type::FX_DELAY_FEEDBACK => trig
            .plock_get_fx_delay_feedback()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_HPF => trig
            .plock_get_fx_delay_hpf()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_LPF => trig
            .plock_get_fx_delay_lpf()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_REVERB_SEND => trig
            .plock_get_fx_delay_reverb_send()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_DELAY_VOLUME => trig
            .plock_get_fx_delay_volume()?
            .map(|val| Atom::from(val as isize)),

        kit_action_type::FX_REVERB_PRE_DELAY => trig
            .plock_get_fx_reverb_pre_delay()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_DECAY => trig
            .plock_get_fx_reverb_decay()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_FREQ => trig
            .plock_get_fx_reverb_freq()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_GAIN => trig
            .plock_get_fx_reverb_gain()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_HPF => trig
            .plock_get_fx_reverb_hpf()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_LPF => trig
            .plock_get_fx_reverb_lpf()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_REVERB_VOLUME => trig
            .plock_get_fx_reverb_volume()?
            .map(|val| Atom::from(val as isize)),

        kit_action_type::FX_COMP_THRESHOLD => trig
            .plock_get_fx_compressor_threshold()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_COMP_GAIN => trig
            .plock_get_fx_compressor_gain()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_COMP_MIX => trig
            .plock_get_fx_compressor_mix()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_COMP_VOLUME => trig
            .plock_get_fx_compressor_volume()?
            .map(|val| Atom::from(val as isize)),

        kit_action_type::FX_LFO_SPEED => trig.plock_get_fx_lfo_speed()?.map(Atom::from),
        kit_action_type::FX_LFO_FADE => trig.plock_get_fx_lfo_fade()?.map(Atom::from),
        kit_action_type::FX_LFO_START_PHASE_OR_SLEW => trig
            .plock_get_fx_lfo_start_phase()?
            .map(|val| Atom::from(val as isize)),
        kit_action_type::FX_LFO_DEPTH => trig
            .plock_get_fx_lfo_depth()?
            .map(|val| Atom::from(val as f64)),
        //
        // TODO: Do the dist setters after fixing the dist in the SDK
        // TODO: Do Machine plocks
        //
        sound_action_type::AMP_ATTACK => trig
            .plock_get_amplitude_attack()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_HOLD => trig
            .plock_get_amplitude_hold()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_DECAY => trig
            .plock_get_amplitude_decay()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_OVERDRIVE => trig
            .plock_get_amplitude_overdrive()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_DELAY_SEND => trig
            .plock_get_amplitude_delay_send()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_REVERB_SEND => trig
            .plock_get_amplitude_reverb_send()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::AMP_PAN => trig.plock_get_amplitude_pan()?.map(Atom::from),
        sound_action_type::AMP_VOLUME => trig
            .plock_get_amplitude_volume()?
            .map(|val| Atom::from(val as isize)),

        sound_action_type::FILT_ATTACK => trig
            .plock_get_filter_attack()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_HOLD => trig
            .plock_get_filter_sustain()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_DECAY => trig
            .plock_get_filter_decay()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_RELEASE => trig
            .plock_get_filter_release()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_CUTOFF => trig
            .plock_get_filter_cutoff()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_RESONANCE => trig
            .plock_get_filter_resonance()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::FILT_ENVELOPE_AMOUNT => {
            trig.plock_get_filter_envelope_amount()?.map(Atom::from)
        }

        sound_action_type::LFO_SPEED => trig.plock_get_lfo_speed()?.map(Atom::from),
        sound_action_type::LFO_FADE => trig.plock_get_lfo_fade()?.map(Atom::from),
        sound_action_type::LFO_START_PHASE_OR_SLEW => trig
            .plock_get_lfo_start_phase()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::LFO_DEPTH => trig
            .plock_get_lfo_depth()?
            .map(|val| Atom::from(val as f64)),

        sound_action_type::SAMP_TUNE => trig.plock_get_sample_tune()?.map(Atom::from),
        sound_action_type::SAMP_FINE_TUNE => trig.plock_get_sample_fine_tune()?.map(Atom::from),
        sound_action_type::SAMP_NUMBER => trig
            .plock_get_sample_number()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::SAMP_BIT_REDUCTION => trig
            .plock_get_sample_bit_reduction()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::SAMP_START => trig
            .plock_get_sample_start()?
            .map(|val| Atom::from(val as f64)),
        sound_action_type::SAMP_END => trig
            .plock_get_sample_end()?
            .map(|val| Atom::from(val as f64)),
        sound_action_type::SAMP_LOOP_FLAG => trig
            .plock_get_sample_loop_flag()?
            .map(|val| Atom::from(val as isize)),
        sound_action_type::SAMP_VOLUME => trig
            .plock_get_sample_volume()?
            .map(|val| Atom::from(val as isize)),

        other => return Err(InvalidActionType(other.to_string()).into()),
    };

    if let Some(value_atom) = value_atom {
        let action_atom = Atom::from(action);
        let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));

        if let Err(_stack_overflow_err) = out.send(&[action_atom, index_atom, value_atom][..]) {
            // Stack overflow ignore
        }
    } else {
        // Send the value as "unset" for a plock which is not set.

        let action_atom = Atom::from(action);
        let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));

        if let Err(_stack_overflow_err) = out.send(
            &[
                action_atom,
                index_atom,
                Atom::from(SymbolRef::from(CString::new("unset").unwrap())),
            ][..],
        ) {
            // Stack overflow ignore
        }
    }

    Ok(())
}

pub fn handle_trig_plock_get_enum_value(
    trig: &Trig,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    use crate::api::kit_enum_type;
    use crate::api::sound_enum_type;

    let enum_value: Option<&str> = match enum_type {
        kit_enum_type::CONTROL_IN_MOD_TARGET => todo!(),
        kit_enum_type::FX_COMP_ATTACK => {
            trig.plock_get_fx_compressor_attack()?.map(|val| val.into())
        }
        kit_enum_type::FX_COMP_RELEASE => trig
            .plock_get_fx_compressor_release()?
            .map(|val| val.into()),
        kit_enum_type::FX_DELAY_TIME_ON_THE_GRID => todo!(),
        kit_enum_type::FX_COMP_RATIO => trig.plock_get_fx_compressor_ratio()?.map(|val| val.into()),
        kit_enum_type::FX_COMP_SIDE_CHAIN_EQ => trig
            .plock_get_fx_compressor_side_chain_eq()?
            .map(|val| val.into()),
        kit_enum_type::FX_LFO_DESTINATION => {
            trig.plock_get_fx_lfo_destination()?.map(|val| val.into())
        }

        // TODO:
        // sound_enum_type::MACHINE_PARAMETERS => todo!(.map(|val|val.into()),
        sound_enum_type::LFO_DESTINATION => trig.plock_get_lfo_destination()?.map(|val| val.into()),
        sound_enum_type::FILTER_TYPE => trig.plock_get_filter_type()?.map(|val| val.into()),
        sound_enum_type::LFO_MULTIPLIER => trig.plock_get_lfo_multiplier()?.map(|val| val.into()),
        sound_enum_type::LFO_WAVEFORM => trig.plock_get_lfo_waveform()?.map(|val| val.into()),
        sound_enum_type::LFO_MODE => trig.plock_get_lfo_mode()?.map(|val| val.into()),

        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    if let Some(enum_value) = enum_value {
        let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
        let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));
        let enum_value_atom = Atom::from(SymbolRef::try_from(enum_value).unwrap());

        if let Err(_stack_overflow_err) =
            out.send(&[enum_type_atom, index_atom, enum_value_atom][..])
        {
            // Stack overflow ignore
        }
        //..
    } else {
        // Send the value as "unset" for a plock which is not set.

        let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
        let index_atom = Atom::from(AtomValue::Int(trig.index() as isize));

        if let Err(_stack_overflow_err) = out.send(
            &[
                enum_type_atom,
                index_atom,
                Atom::from(SymbolRef::from(CString::new("unset").unwrap())),
            ][..],
        ) {
            // Stack overflow ignore
        }
    }

    Ok(())
}
