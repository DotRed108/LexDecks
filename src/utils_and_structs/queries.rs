use::core::str::FromStr;

use super::{database_types::{DeckId, NoteType}, shared_truth::SEPARATOR};

pub const QUERY_PARAM_FOR_QUERIES: &str = "QueryType";

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ValidQueryTypes {
    #[default] NoQuery,
    NotesById(DeckId, Vec<usize>),
    NotesByLevel(DeckId, Vec<usize>),
    NotesByType(DeckId, Vec<NoteType>),
}

impl FromStr for ValidQueryTypes {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NoQuery" => Ok(Self::NoQuery),
            "NotesById" => Ok(Self::NotesById(DeckId::default(), Vec::new())),
            "NotesByLevel"  => Ok(Self::NotesByLevel(DeckId::default(), Vec::new())),
            "NotesByType"  => Ok(Self::NotesByType(DeckId::default(), Vec::new())),
            complex_enum => {
                let carried_data_index = match complex_enum.find("(") {
                    Some(index) => index,
                    None => return Err(()),
                };
                let (enum_variant, carried_data) = complex_enum.split_at(carried_data_index);
                let mut carried_data = carried_data.replacen("(", "", 1);
                carried_data.pop();

                let query_specs_index = match carried_data.find(SEPARATOR) {
                    Some(index) => index,
                    None => return Err(()),
                };

                let (pk, query_specs) = carried_data.split_at(query_specs_index);
                let mut query_specs = query_specs.chars();
                query_specs.next();
                let query_specs = query_specs.as_str();

                let query_type: Self = match Self::from_str(enum_variant) {
                    Ok(enum_variant) => {
                        match enum_variant {
                            Self::NotesById(_, _) => {
                                let id_strs = query_specs.split(",");

                                let ids = id_strs.map(|id_str| {
                                    let id = usize::from_str(id_str).unwrap_or_default();
                                    id
                                }).collect();

                                let Ok(deck_id) = DeckId::from_str(pk) else {return Err(())};
                                Self::NotesById(deck_id, ids)
                            }
                            Self::NotesByLevel(_, _) => {
                                let level_strs = query_specs.split(",");

                                let levels = level_strs.map(|level_str| {
                                    println!("{}", level_str);
                                    let level = usize::from_str(level_str).unwrap_or_default();
                                    level
                                }).collect();

                                let Ok(deck_id) = DeckId::from_str(pk) else {return Err(())};
                                Self::NotesByLevel(deck_id, levels)
                            },
                            Self::NotesByType(_, _) => {
                                let type_strs = query_specs.split(",");

                                let types = type_strs.map(|type_str| {
                                    let tipe = NoteType::from_str(type_str).unwrap_or_default();
                                    tipe
                                }).collect();

                                let Ok(deck_id) = DeckId::from_str(pk) else {return Err(())};
                                Self::NotesByType(deck_id, types)
                            }
                            _ => return Err(()),
                        }
                    },
                    Err(_) => return Err(()),
                };


                Ok(query_type)
            },
        }
    }
}

impl ToString for ValidQueryTypes {
    fn to_string(&self) -> String {
        match self {
            Self::NoQuery => "NoQuery".to_string(),
            Self::NotesById(deck_id, query_specs) => {
                let mut ids_str = "".to_string();
                for id in query_specs {
                    ids_str.push_str(&id.to_string());
                    ids_str.push_str(",");
                }
                ids_str.pop();
                format!("NotesById({}{}{})", deck_id.to_string(), SEPARATOR, ids_str)
            },
            Self::NotesByLevel(deck_id, query_specs) => {
                let mut levels_str = "".to_string();
                for level in query_specs {
                    levels_str.push_str(&level.to_string());
                    levels_str.push_str(",");
                }
                levels_str.pop();
                format!("NotesByLevel({}{}{})", deck_id.to_string(), SEPARATOR, levels_str)
            },
            Self::NotesByType(deck_id, query_specs) => {
                let mut types_str = "".to_string();
                for tipe in query_specs {
                    types_str.push_str(&tipe.to_string());
                    types_str.push_str(",");
                }
                types_str.pop();
                format!("NotesByType({}{}{})", deck_id.to_string(), SEPARATOR, types_str)
            },
        }
    }
}
