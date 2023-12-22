use std::convert::TryFrom;

use crate::api::kit_element_type::KIT_ELEMENTS;
use crate::api::kit_enum_type::KIT_ENUM_TYPES;
use crate::error::RytmExternalError;
use crate::{api::kit_action_type::KIT_ACTION_TYPES, error::IdentifierError::InvalidParameter};
use median::{
    atom::{Atom, AtomType, AtomValue},
    symbol::SymbolRef,
};

pub fn try_get_index_with_range(
    atoms: &[Atom],
    select: usize,
    min: usize,
    max: usize,
    identifier: &str,
) -> Result<usize, RytmExternalError> {
    atoms.get(select).map_or_else(
        || Err(format!("Invalid format: An integer index should follow the {identifier}.").into()),
        |atom| {
            atom.get_value().map_or_else(
                || {
                    Err(
                        format!("Invalid format: An integer index should follow the {identifier}.")
                            .into(),
                    )
                },
                |value| match value {
                    AtomValue::Int(index) => {
                        if index < min as isize || index > max as isize {
                            Err(format!(
                                "Invalid format: The {identifier} must be between {min} and {max}.",
                            )
                            .into())
                        } else {
                            Ok(index as usize)
                        }
                    }
                    _ => Err(format!(
                        "Invalid format: An integer index should follow the {identifier}."
                    )
                    .into()),
                },
            )
        },
    )
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
            Ok(Self::Action(value))
        } else if value_str.contains(':') {
            let (t, v) = value_str.split_once(':').ok_or_else(||RytmExternalError::from("Invalid value: A kit setter should be followed by either a kit element or action or enum."))?;
            if KIT_ENUM_TYPES.contains(&t) {
                return Ok(Self::EnumTypeAndValue(t.to_owned(), v.to_owned()));
            }
            Err(format!("Invalid value: {value}. A kit setter or getter should be followed by either a kit element, identifier or enum.").into())
        } else if KIT_ELEMENTS.contains(&value_str.as_str()) {
            if value_str == crate::api::kit_element_type::SOUND {
                return Ok(Self::KitSound);
            }
            Ok(Self::KitElement(value_str))
        } else {
            Err(format!("Invalid value: {value}. A kit setter or getter should be followed by either a kit element, identifier or enum.").into())
        }
    }
}

pub fn try_get_atom_value_as_kit_element_or_identifier_or_enum_value(
    index: usize,
    atoms: &[Atom],
) -> Result<KitElementOrActionOrEnumTypeAndValue, RytmExternalError> {
    if let Some(atom) = atoms.get(index) {
        if let Some(value) = atom.get_value() {
            match value {
                    AtomValue::Symbol(symbol) => {
                        Ok(KitElementOrActionOrEnumTypeAndValue::try_from(symbol)?)
                    },
                     _ => Err(format!("Invalid format: {}. A kit setter or getter should be followed by either a kit element, identifier or enum.", string_from_atom_slice(atoms)).into()),
                }
        } else {
            Err(format!("Invalid format: {}. The list should be followed by either a kit element, identifier or enum.", string_from_atom_slice(atoms)).into())
        }
    } else {
        Err(format!("Invalid format: {}. The list should be followed by either a kit element, identifier or enum.", string_from_atom_slice(atoms)).into())
    }
}

pub fn try_get_atom_value_assuming_identifier_or_index_or_enum_value(
    index: usize,
    atoms: &[Atom],
) -> Result<AtomValue, RytmExternalError> {
    atoms.get(index).map_or_else(
        || {
            Err(format!(
                "Invalid format: {}. The list should be followed by an identifier, an index or an enum.",
                string_from_atom_slice(atoms)
            )
            .into())
        },
        |atom| {
            atom.get_value().map_or_else(
                || {
                    Err(format!(
                        "Invalid format: {}. The list should be followed by an identifier, an index or an enum.",
                        string_from_atom_slice(atoms)
                    )
                    .into())
                },
                Ok,
            )
        },
    )
}

pub fn try_get_identifier_value_from_atom_slice(
    index: usize,
    atoms: &[Atom],
) -> Result<&Atom, RytmExternalError> {
    atoms.get(index).map_or_else(
        || {
            Err(format!(
                "Invalid format: {}. The identifier should be followed by a parameter.",
                string_from_atom_slice(atoms)
            )
            .into())
        },
        |atom| {
            if atom.get_value().is_some() {
                Ok(atom)
            } else {
                Err(format!(
                    "Invalid format: {}. The identifier should be followed by a parameter.",
                    string_from_atom_slice(atoms)
                )
                .into())
            }
        },
    )
}

/// Gets a bool from 0 or 1 as an atom.
pub fn get_bool_from_0_or_1(value: &Atom, identifier: &str) -> Result<bool, RytmExternalError> {
    match value.get_value().ok_or_else(|| {
        RytmExternalError::from(format!(
            "Invalid parameter: {identifier} must be followed by a 0 or 1"
        ))
    })? {
        AtomValue::Int(value) => match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!(
                "Invalid parameter: {value}. {identifier} must be followed by a 0 or 1"
            )
            .into()),
        },
        _ => Err(format!(
            "Invalid parameter: {}. {identifier} must be followed by a 0 or 1",
            string_from_atom(value)
        )
        .into()),
    }
}

pub fn only_allow_numbers_as_identifier_parameter(atom: &Atom) -> Result<(), RytmExternalError> {
    match atom.get_type() {
        Some(AtomType::Object | AtomType::Symbol) | None => Err(InvalidParameter(
            "Allowed parameters are integers or floats.".to_owned(),
            string_from_atom(atom),
        )
        .into()),
        _ => Ok(()),
    }
}

pub fn string_from_atom_slice(atoms: &[Atom]) -> String {
    if atoms.is_empty() {
        return String::new();
    }

    let mut string = String::new();
    for atom in atoms {
        string.push_str(&atom.get_symbol().to_cstring().to_string_lossy());
        string.push(' ');
    }

    string
}

pub fn string_from_atom(atom: &Atom) -> String {
    atom.get_symbol().to_cstring().to_string_lossy().to_string()
}
