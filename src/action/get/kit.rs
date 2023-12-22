use crate::api::kit_action_type::*;
use crate::api::kit_element_type::*;
use crate::api::kit_enum_type::*;
use crate::api::sound_kit::handle_sound_kit_get;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use crate::error::RytmExternalError::NotYetImplemented;
use median::atom::{Atom, AtomValue};
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::Kit;
use rytm_rs::object::Sound;
use std::convert::TryFrom;
use std::ffi::CString;

pub fn handle_kit_get_action(
    kit: &Kit,
    action: SymbolRef,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;

    let value_atom: Atom = match action_str.as_str() {
        VERSION => (kit.structure_version() as isize).into(),
        INDEX => (kit.index() as isize).into(),
        NAME => SymbolRef::from(CString::new(kit.name()).unwrap()).into(),
        FX_DELAY_TIME => (kit.fx_delay().time() as isize).into(),
        FX_DELAY_PING_PONG => isize::from(kit.fx_delay().ping_pong()).into(),
        FX_DELAY_STEREO_WIDTH => (kit.fx_delay().stereo_width()).into(),
        FX_DELAY_FEEDBACK => (kit.fx_delay().feedback() as isize).into(),
        FX_DELAY_HPF => (kit.fx_delay().hpf() as isize).into(),
        FX_DELAY_LPF => (kit.fx_delay().lpf() as isize).into(),
        FX_DELAY_REVERB_SEND => (kit.fx_delay().reverb_send() as isize).into(),
        FX_DELAY_VOLUME => (kit.fx_delay().volume() as isize).into(),

        FX_REVERB_PRE_DELAY => (kit.fx_reverb().pre_delay() as isize).into(),
        FX_REVERB_DECAY => (kit.fx_reverb().decay() as isize).into(),
        FX_REVERB_FREQ => (kit.fx_reverb().freq() as isize).into(),
        FX_REVERB_GAIN => (kit.fx_reverb().gain() as isize).into(),
        FX_REVERB_HPF => (kit.fx_reverb().hpf() as isize).into(),
        FX_REVERB_LPF => (kit.fx_reverb().lpf() as isize).into(),
        FX_REVERB_VOLUME => (kit.fx_reverb().volume() as isize).into(),

        FX_COMP_THRESHOLD => (kit.fx_compressor().threshold() as isize).into(),
        FX_COMP_GAIN => (kit.fx_compressor().gain() as isize).into(),
        FX_COMP_MIX => (kit.fx_compressor().mix() as isize).into(),
        FX_COMP_VOLUME => (kit.fx_compressor().volume() as isize).into(),

        FX_LFO_SPEED => (kit.fx_lfo().speed()).into(),
        FX_LFO_FADE => (kit.fx_lfo().fade()).into(),
        FX_LFO_START_PHASE_OR_SLEW => (kit.fx_lfo().start_phase_or_slew() as isize).into(),
        FX_LFO_DEPTH => f64::from(kit.fx_lfo().depth()).into(),

        FX_DISTORTION_DELAY_OVERDRIVE => (kit.fx_distortion().delay_overdrive() as isize).into(),
        FX_DISTORTION_DELAY_POST => (kit.fx_distortion().delay_post() as isize).into(),
        FX_DISTORTION_REVERB_POST => (kit.fx_distortion().reverb_post() as isize).into(),
        FX_DISTORTION_AMOUNT => (kit.fx_distortion().amount() as isize).into(),
        FX_DISTORTION_SYMMETRY => (kit.fx_distortion().symmetry() as isize).into(),

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let action_atom = Atom::from(action);
    let kit_index_atom = Atom::from(AtomValue::Int(kit.index() as isize));

    if let Err(_stack_overflow_err) = out.send(&[kit_index_atom, action_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_kit_get_enum_value(
    kit: &Kit,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let enum_value: &str = match enum_type {
        CONTROL_IN_MOD_TARGET => return Err(NotYetImplemented),
        FX_LFO_DESTINATION => (*kit.fx_lfo().destination()).into(),
        FX_COMP_ATTACK => (*kit.fx_compressor().attack()).into(),
        FX_COMP_RELEASE => (*kit.fx_compressor().release()).into(),
        FX_COMP_RATIO => (*kit.fx_compressor().ratio()).into(),
        FX_COMP_SIDE_CHAIN_EQ => (*kit.fx_compressor().side_chain_eq()).into(),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let kit_index_atom = Atom::from(AtomValue::Int(kit.index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(enum_value).unwrap());

    if let Err(_stack_overflow_err) =
        out.send(&[kit_index_atom, enum_type_atom, enum_value_atom][..])
    {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_kit_get_kit_element(
    kit: &Kit,
    element_type: &str,
    element_index: usize,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value_atom: Atom = match element_type {
        TRACK_LEVEL => (kit.track_level(element_index)? as isize).into(),
        TRACK_RETRIG_RATE => (SymbolRef::from(
            CString::new(Into::<&str>::into(
                kit.track_retrig_settings(element_index)?.rate(),
            ))
            .unwrap(),
        ))
        .into(),
        TRACK_RETRIG_LENGTH => (SymbolRef::from(
            CString::new(Into::<&str>::into(
                kit.track_retrig_settings(element_index)?.length(),
            ))
            .unwrap(),
        ))
        .into(),
        TRACK_RETRIG_VEL_OFFSET => {
            (kit.track_retrig_settings(element_index)?.velocity_curve() as isize).into()
        }
        TRACK_RETRIG_ALWAYS_ON => {
            isize::from(kit.track_retrig_settings(element_index)?.always_on()).into()
        }

        other => return Err(IdentifierError::InvalidType(other.to_owned()).into()),
    };

    let element_type_atom = Atom::from(SymbolRef::try_from(element_type).unwrap());
    let kit_index_atom = Atom::from(AtomValue::Int(kit.index() as isize));
    let element_index_atom = Atom::from(AtomValue::Int(element_index as isize));

    if let Err(_stack_overflow_err) = out.send(
        &[
            kit_index_atom,
            element_type_atom,
            element_index_atom,
            value_atom,
        ][..],
    ) {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_kit_get_kit_sound(
    sound: &Sound,
    atoms: &[Atom],
    slice_from_index: usize,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    handle_sound_kit_get(sound, &atoms[slice_from_index..], out)
}
