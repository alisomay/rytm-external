use crate::error::QueryError::InvalidFormat;
use crate::error::QueryError::InvalidIndexType;
use crate::error::QueryError::InvalidSelector;
use crate::{api::object_type::*, error::RytmExternalError};
use median::atom::{Atom, AtomValue};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum ObjectTypeSelector {
    Pattern(usize),
    PatternWorkBuffer,
    Kit(usize),
    KitWorkBuffer,
    Sound(usize),
    SoundWorkBuffer(usize),
    Global(usize),
    GlobalWorkBuffer,
    Settings,
}

impl ObjectTypeSelector {
    pub const fn indexable(&self) -> bool {
        matches!(
            self,
            Self::Pattern(_)
                | Self::Kit(_)
                | Self::Sound(_)
                | Self::SoundWorkBuffer(_)
                | Self::Global(_)
        )
    }
}

impl TryFrom<(&Atom, Option<&Atom>)> for ObjectTypeSelector {
    type Error = RytmExternalError;

    fn try_from((sel, index): (&Atom, Option<&Atom>)) -> Result<Self, Self::Error> {
        match sel.get_value() {
            Some(AtomValue::Symbol(selector_sym)) => {
                if selector_sym == *PATTERN {
                    index.map_or_else(
                        || Err(InvalidFormat.into()),
                        |atom| match atom.get_value() {
                            Some(AtomValue::Int(index)) => match index {
                                0..=127 => Ok(Self::Pattern(index as usize)),
                                _ => {
                                    Err("Pattern index must be an integer between 0 and 127".into())
                                }
                            },
                            _ => Err(InvalidIndexType.into()),
                        },
                    )
                } else if selector_sym == *PATTERN_WORK_BUFFER {
                    Ok(Self::PatternWorkBuffer)
                } else if selector_sym == *KIT {
                    index.map_or_else(
                        || Err(InvalidFormat.into()),
                        |atom| match atom.get_value() {
                            Some(AtomValue::Int(index)) => match index {
                                0..=127 => Ok(Self::Kit(index as usize)),
                                _ => Err("Kit index must be an integer between 0 and 127".into()),
                            },
                            _ => Err(InvalidIndexType.into()),
                        },
                    )
                } else if selector_sym == *KIT_WORK_BUFFER {
                    Ok(Self::KitWorkBuffer)
                } else if selector_sym == *SOUND {
                    index.map_or_else(
                        || Err(InvalidFormat.into()),
                        |atom| match atom.get_value() {
                            Some(AtomValue::Int(index)) => match index {
                                0..=11 => Ok(Self::Sound(index as usize)),
                                _ => {
                                    Err("Pool sound index must be an integer between 0 and 11"
                                        .into())
                                }
                            },
                            _ => Err(InvalidIndexType.into()),
                        },
                    )
                } else if selector_sym == *SOUND_WORK_BUFFER {
                    index.map_or_else(
                        || Err(InvalidFormat.into()),
                        |atom| match atom.get_value() {
                            Some(AtomValue::Int(index)) => match index {
                                0..=11 => Ok(Self::SoundWorkBuffer(index as usize)),
                                _ => Err(
                                    "Work buffer sound index must be an integer between 0 and 11"
                                        .into(),
                                ),
                            },
                            _ => Err(InvalidIndexType.into()),
                        },
                    )
                } else if selector_sym == *GLOBAL {
                    index.map_or_else(
                        || Err(InvalidFormat.into()),
                        |atom| match atom.get_value() {
                            Some(AtomValue::Int(index)) => match index {
                                0..=3 => Ok(Self::Global(index as usize)),
                                _ => {
                                    Err("Global slot index must be an integer between 0 and 3"
                                        .into())
                                }
                            },
                            _ => Err(InvalidIndexType.into()),
                        },
                    )
                } else if selector_sym == *GLOBAL_WORK_BUFFER {
                    Ok(Self::GlobalWorkBuffer)
                } else if selector_sym == *SETTINGS {
                    Ok(Self::Settings)
                } else {
                    Err(InvalidSelector.into())
                }
            }
            _ => Err(InvalidSelector.into()),
        }
    }
}
