use std::convert::TryFrom;

use crate::api::kit_element_type::KIT_ELEMENTS;
use crate::api::kit_enum_type::KIT_ENUM_TYPES;
use crate::error::RytmExternalError;
use crate::{api::kit_action_type::KIT_ACTION_TYPES, error::ActionError::InvalidActionParameter};
use median::{
    atom::{Atom, AtomType, AtomValue},
    symbol::SymbolRef,
};

pub fn get_bool_from_0_or_1(value: &Atom) -> Result<bool, RytmExternalError> {
    match value
        .get_value()
        .ok_or_else(|| RytmExternalError::from("Value must be 0 or 1"))?
    {
        AtomValue::Int(value) => match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Value must be 0 or 1".into()),
        },
        _ => Err("Value must be 0 or 1".into()),
    }
}

pub fn try_get_index_with_range(
    atoms: &[Atom],
    select: usize,
    min: usize,
    max: usize,
    err_object: &str,
) -> Result<usize, RytmExternalError> {
    if let Some(atom) = atoms.get(select) {
        if let Some(value) = atom.get_value() {
            match value {
                AtomValue::Int(index) => {
                    if index < min as isize || index > max as isize {
                        Err(format!(
                            "Invalid format: The {err_object} must be between {min} and {max}.",
                        )
                        .into())
                    } else {
                        Ok(index as usize)
                    }
                }
                _ => Err(format!(
                    "Invalid format: An integer index should follow the {err_object}."
                )
                .into()),
            }
        } else {
            Err(format!("Invalid format: An integer index should follow the {err_object}.").into())
        }
    } else {
        Err(format!("Invalid format: An integer index should follow the {err_object}.").into())
    }
}

pub enum KitElementOrActionOrEnumTypeAndValue {
    KitElement(String),
    Action(SymbolRef),
    EnumTypeAndValue(String, String),
    KitSound,
}

impl TryFrom<SymbolRef> for KitElementOrActionOrEnumTypeAndValue {
    type Error = RytmExternalError;

    fn try_from(value: SymbolRef) -> Result<Self, Self::Error> {
        let value_str = value.to_string()?;
        if KIT_ACTION_TYPES.contains(&value_str.as_str()) {
            Ok(KitElementOrActionOrEnumTypeAndValue::Action(value))
        } else if value_str.find(":").is_some() {
            let (t, v) = value_str.split_once(':').ok_or_else(||RytmExternalError::from("Invalid value: A kit setter should be followed by either a kit element or action or enum value."))?;
            if KIT_ENUM_TYPES.contains(&t) {
                return Ok(KitElementOrActionOrEnumTypeAndValue::EnumTypeAndValue(
                    t.to_owned(),
                    v.to_owned(),
                ));
            }
            Err("Invalid value: A kit setter should be followed by either a kit element or action or enum value.".into())
        } else if KIT_ELEMENTS.contains(&value_str.as_str()) {
            if value_str == crate::api::kit_element_type::SOUND {
                return Ok(KitElementOrActionOrEnumTypeAndValue::KitSound);
            }
            Ok(KitElementOrActionOrEnumTypeAndValue::KitElement(
                value_str.to_owned(),
            ))
        } else {
            Err("Invalid value: A kit setter should be followed by either a kit element or action or enum value.".into())
        }
    }
}

pub fn try_get_atom_value_as_kit_element_or_action_or_enum_value(
    index: usize,
    atoms: &[Atom],
) -> Result<KitElementOrActionOrEnumTypeAndValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            match value {
                    AtomValue::Symbol(symbol) => {
                        Ok(KitElementOrActionOrEnumTypeAndValue::try_from(symbol)?)
                    },
                     _ => Err("Invalid value: A kit setter should be followed by either a kit element or action or enum value.".into()),
                }
        } else {
            Err("Invalid format: The list must be followed by either a kit element, action or enum value.".into())
        }
    } else {
        Err("Invalid format: The list must be followed by either a kit element, action or enum value.".into())
    }
}

pub fn try_get_atom_value_assuming_action_or_index_or_enum_value(
    index: usize,
    atoms: &[Atom],
) -> Result<AtomValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            Ok(value)
        } else {
            Err("Invalid format: The list must be followed by an action or an index.".into())
        }
    } else {
        Err("Invalid format: The list must be followed by an action or an index.".into())
    }
}

pub fn try_get_action_value_from_atom_slice(
    index: usize,
    atoms: &[Atom],
) -> Result<&Atom, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(_) = atom.get_value() {
            Ok(atom)
        } else {
            Err("Invalid format: An action must be followed by a parameter.".into())
        }
    } else {
        Err("Invalid format: An action must be followed by a parameter".into())
    }
}

pub fn only_allow_numbers_as_action_parameter(atom: &Atom) -> Result<(), RytmExternalError> {
    match atom.get_type() {
        Some(AtomType::Object) | Some(AtomType::Symbol) | None => Err(InvalidActionParameter(
            "Action parameters can be only integers or floats.".to_owned(),
        )
        .into()),
        _ => Ok(()),
    }
}
