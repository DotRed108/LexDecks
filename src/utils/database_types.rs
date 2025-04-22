use::core::str::FromStr;
use core::ops::{Deref, DerefMut};
use struct_field_names::StructFieldNames;
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};
use serde::{de::Error, Deserialize, Serialize, Serializer};

use super::{shared_truth::{DECK_ID_LENGTH, MAX_LEVELS, SEPARATOR, SEPARATOR3, SEPARATOR4, SEPARATOR5}, user_types::UserInfo};

pub const ASSET_HEADER: &str = "asset";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeckId {
    pub id: [char; DECK_ID_LENGTH],
}

impl Serialize for DeckId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DeckId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let str: String = Deserialize::deserialize(deserializer)?;
        
        match DeckId::from_str(&str) {
            Ok(deck_id) => Ok(deck_id),
            Err(_) => Err(Error::custom("invalid deck id")),
        }
    }
}

impl FromStr for DeckId {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let chars = input.chars();
        let mut deck_id = Self::default();
        for (i, char) in chars.into_iter().enumerate() {
            if i >= DECK_ID_LENGTH {
                return Err(());
            }
            deck_id.id[i] = char;
        }
        Ok(deck_id)
    }
}

impl ToString for DeckId {
    fn to_string(&self) -> String {
        String::from_iter(self.id)
    }
}

impl Default for DeckId {
    fn default() -> Self {
        Self {id: ['0'; DECK_ID_LENGTH]}
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeckList {
    pub decks: Vec<DeckId>
}

impl DeckList {
    pub fn from_str_vec(deck_vec: &Vec<String>) -> Option<DeckList> {
        let mut deck_list = DeckList::default();
        for id in deck_vec.iter() {
            let Ok(deck_id) = DeckId::from_str(id) else {return None};
            deck_list.push(deck_id);
        }

        Some(deck_list)
    }

    pub fn add_decks_wo_dupes(&mut self, mut deck_list: Self) {
        deck_list.retain(|deck_id| {
            for other_deck_id in self.iter() {
                if deck_id == other_deck_id {
                    return false;
                }
            }
            true
        });

        self.append(&mut deck_list);
    }

    pub fn remove_decks(&mut self, deck_list: Self) {
        self.retain(|deck_id| {
            for other_deck_id in deck_list.iter() {
                if deck_id == other_deck_id {
                    return false;
                }
            }
            true
        })
    }

    pub fn strip_default_decks(&mut self) {
        self.retain(|deck_id| {
            if deck_id == &DeckId::default() {
                return false;
            }
            true
        })
    }
}

impl Deref for DeckList {
    type Target = Vec<DeckId>;

    fn deref(&self) -> &Self::Target {
        &self.decks
    }
}

impl DerefMut for DeckList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.decks
    }
}

impl Serialize for DeckList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        self.decks.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DeckList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let decks: Vec<DeckId> = Deserialize::deserialize(deserializer)?;

        Ok(DeckList {
            decks,
        })
    }
}

impl FromStr for DeckList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deck_list = match serde_json::from_str(s) {
            Ok(decks) => decks,
            Err(_) => return Err(()),
        };
        Ok(deck_list)
    }
}

impl ToString for DeckList {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct S3Address {
    pub bucket: String,
    pub key: String,
}

impl FromStr for S3Address {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut address_iterator = input.split(SEPARATOR);
        let Some(bucket) = address_iterator.next() else {return Err(())};
        let Some(key) = address_iterator.next() else {return Err(())};

        Ok(Self {
            bucket: bucket.to_string(),
            key: key.to_string(),
        })
    }
}

impl ToString for S3Address {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.bucket, SEPARATOR, self.key)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Asset {
    #[default]
    None,
    PFP(S3Address),
    DeckImage(S3Address),
    CachedPFP(String /* Should be asset as string */, String /* URL */),
}

impl ToString for Asset {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for Asset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asset = match serde_json::from_str(s) {
            Ok(a) => a,
            Err(_) => return Err(()),
        };
        Ok(asset)
    }
}

impl Asset {
    pub fn get_url(&self) -> String {
        match self {
            Asset::CachedPFP(_, url) => url.to_string(),
            _ => String::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeckMeta {
    pub name: String,
    pub owner: String,
    pub public: bool,
    pub price: f32,
    #[serde(with = "serde_arrays")]
    pub note_count_by_level: [usize; MAX_LEVELS],
    pub note_count_by_type: [usize; NoteType::COUNT],
    pub total_notes: usize,
}

impl DeckMeta {
    pub fn new_deck_meta(email: &str, total_notes: usize) -> DeckMeta {
        let mut type_array = [0; NoteType::COUNT];
        type_array[0] = total_notes as usize;
        let note_count_by_level = get_level_map(total_notes);

        DeckMeta {
            name: "New Deck".to_string(),
            owner: email.to_string(),
            public: false,
            price: 0.0,
            note_count_by_level,
            note_count_by_type: type_array,
            total_notes,
        }
    }
}

impl ToString for DeckMeta {
    fn to_string(&self) -> String {
        let mut meta_str = String::from("");
        meta_str.push_str(&self.name);
        meta_str.push(SEPARATOR);
        meta_str.push_str(&self.owner);
        meta_str.push(SEPARATOR);
        meta_str.push_str(&self.public.to_string());
        meta_str.push(SEPARATOR);
        meta_str.push_str(&self.price.to_string());
        meta_str.push(SEPARATOR);
        for count in self.note_count_by_level {
            let level_count_str = format!("{}|", count);
            meta_str.push_str(&level_count_str);
        }
        meta_str.pop();
        meta_str.push(SEPARATOR);
        for count in self.note_count_by_type {
            let type_count_str = format!("{}|", count);
            meta_str.push_str(&type_count_str);
        }
        meta_str.pop();
        meta_str.push(SEPARATOR);
        meta_str.push_str(&self.total_notes.to_string());


        meta_str
    }
}

impl FromStr for DeckMeta {

    type Err = ();

    fn from_str(input: &str) -> Result<DeckMeta, Self::Err> {
        let mut metas = input.split(SEPARATOR);
        let name = metas.next().unwrap_or_default().to_string();
        let owner = metas.next().unwrap_or_default().to_string();
        let public = bool::from_str(metas.next().unwrap_or_default()).unwrap_or_default();
        let price = f32::from_str(metas.next().unwrap_or_default()).unwrap_or_default();

        let note_count_by_level = metas.next().unwrap_or_default();
        let note_count_by_level_splitter = note_count_by_level.split('|');
        let mut note_count_by_level = [0; MAX_LEVELS];
        for (i, count) in note_count_by_level_splitter.enumerate() {
            let Ok(count) = usize::from_str(count) else {return Err(())};

            note_count_by_level[i] = count;
        }

        let note_count_by_type = metas.next().unwrap_or_default();
        let note_count_by_type_splitter = note_count_by_type.split('|');
        let mut note_count_by_type = [0; NoteType::COUNT];
        for (i, count) in note_count_by_type_splitter.enumerate() {
            let Ok(count) = usize::from_str(count) else {return Err(())};

            note_count_by_type[i] = count;
        }
        let total_notes = usize::from_str(metas.next().unwrap_or_default()).unwrap_or_default();

        Ok(DeckMeta {
            name,
            owner,
            public,
            price,
            note_count_by_level,
            note_count_by_type,
            total_notes,
        })
    }
}

pub fn get_notes_per_level(total_notes: usize) -> usize {
    (total_notes as f32 / MAX_LEVELS as f32).ceil().max(20.0) as usize
}

pub fn get_note_level(note_id: u64, notes_per_level: usize) -> usize {
    (note_id as f32 / notes_per_level as f32).ceil() as usize
}

pub fn get_level_map(total_notes: usize) -> [usize; 100] {
    let notes_per_level = get_notes_per_level(total_notes);

    let mut level_map = [0; MAX_LEVELS];
    for note_id in 1..=total_notes {
        let level = get_note_level(note_id as u64, notes_per_level);
        level_map[level - 1] = level_map[level - 1] + 1;
    }

    level_map
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumIter, Serialize, Deserialize)]
pub enum UpdateType {
    Add,
    Subtract,
    Swap,
}

impl FromStr for UpdateType {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        for variant in Self::iter() {
            if input == &variant.to_string() {
                return Ok(variant);
            }
        }
        Err(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DBItem {
    User(String),
    Note(DeckId, u64),
}

impl ToString for DBItem {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for DBItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match serde_json::from_str(s) {
            Ok(i) => i,
            Err(_) => return Err(()),
        };
        Ok(item)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRecipe {
    pub update_type: UpdateType,
    pub update_key: String,
    pub update_item: DBItem,
    pub value: UpdateValues,
}

impl ToString for UpdateRecipe {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for UpdateRecipe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let recipe = match serde_json::from_str(s) {
            Ok(r) => r,
            Err(_) => return Err(()),
        };
        Ok(recipe)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateValues {
    Float64(f64),
    String(String),
    DeckId(DeckId),
    UserInfo(UserInfo),
    Note(Note),
    DeckList(DeckList),
    Unsigned64(u64),
}

impl ToString for UpdateValues {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for UpdateValues {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let update_value = match serde_json::from_str(s) {
            Ok(u) => u,
            Err(_) => return Err(()),
        };
        Ok(update_value)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateRecipes {
    pub recipes: Vec<UpdateRecipe>,
}

impl ToString for UpdateRecipes {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for UpdateRecipes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let recipes = match serde_json::from_str(s) {
            Ok(r) => r,
            Err(_) => return Err(()),
        };
        Ok(recipes)
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, StructFieldNames)]
pub struct Note {
    pub note_id: u64,
    pub deck_id: DeckId,
    pub fields: Vec<Field>,
    pub note_type: NoteType,
    pub version: u64,
    pub reviews_per_stage: u8,
    pub level: u32,
    pub meta: Option<DeckMeta>,
}

impl Note {
    pub fn new_from_function(fields: String, separator: Option<char>, func: &dyn Fn(String, Option<char>) -> Self) -> Self {
        func(fields, separator)
    }

    pub fn new(note_id: u64, fields: Vec<Field>) -> Self {
        
        Self {
            note_id,
            fields,
            ..Note::default()
        }
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for field in self.fields.iter() {
            string.push_str(&field.to_string());
            string.push(SEPARATOR4);
        }
        string.push_str(&self.note_id.to_string());
        string.push(SEPARATOR3);
        string.push_str(&self.deck_id.to_string());
        string.push(SEPARATOR3);
        string.push_str(&self.note_type.to_string());
        string.push(SEPARATOR3);
        string.push_str(&self.version.to_string());
        string.push(SEPARATOR3);
        string.push_str(&self.reviews_per_stage.to_string());
        string.push(SEPARATOR3);
        string.push_str(&self.level.to_string());
        string.push(SEPARATOR3);
        match &self.meta {
            Some(meta) => string.push_str(&meta.to_string()),
            None => string.push_str("None"),
        }
        
        string
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut splitter = input.split(SEPARATOR4);

        let Some(universal_fields) = splitter.next_back() else {return Err(())};

        let mut fields = Vec::new();
        for field in splitter {
            let Ok(fld) = Field::from_str(field) else {return Err(())};
            
            fields.push(fld);
        }

        let mut universal_fields = universal_fields.split(SEPARATOR3);

        let Some(str_id) = universal_fields.next() else {return Err(())};
        let Ok(note_id) = u64::from_str(str_id) else {return Err(())};

        let Some(deck_id_str) = universal_fields.next() else {return Err(())};
        let Ok(deck_id) = DeckId::from_str(deck_id_str) else {return Err(())};

        let Some(note_type) = universal_fields.next() else {return Err(())};
        let Ok(note_type) = NoteType::from_str(note_type) else {return Err(())};

        let Some(version) = universal_fields.next() else {return Err(())};
        let Ok(version) = u64::from_str(version) else {return Err(())};

        let Some(reviews_per_stage) = universal_fields.next() else {return Err(())};
        let Ok(reviews_per_stage) = u8::from_str(reviews_per_stage) else {return Err(())};

        let Some(level) = universal_fields.next() else {return Err(())};
        let Ok(level) = u32::from_str(level) else {return Err(())};

        let Some(meta) = universal_fields.next() else {return Err(())};
        let meta = if let Ok(meta) = DeckMeta::from_str(meta) {
            Some(meta)
        } else {
            None
        };

        


        Ok(Self {
            note_id,
            deck_id,
            fields,
            note_type,
            version,
            reviews_per_stage,
            level,
            meta,
        })
    }
}

impl Note {
    pub const FULL_NOTE_CACHE_KEY: &'static str = "FullNote";
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub text: Option<String>,
    pub asset: Option<Asset>
}

impl Field {
    pub fn new_from_html(name: String, data: String, stripper: &dyn Fn(&String) -> String) -> Self {
        let text = stripper(&data);
        Self {
            name,
            text: Some(text),
            asset: None,
        }
    }
    pub fn new(name: String, text: Option<String>, asset: Option<Asset>) -> Self {
        Self {
            name,
            text,
            asset,
        }
    }
    pub fn to_database_string(&self) -> String {
        if self.asset.is_none() {
            let Some(text) = &self.text else {return "".to_string()};
            return text.to_string();
        } else {
            let Some(asset) = &self.asset else {return "".to_string()};
            return asset.to_string();
        }
    }
    pub fn from_database_string(db_entry: String, name: String) -> Self {
        let asset = match Asset::from_str(&db_entry) {
            Ok(asset) => Some(asset),
            Err(_) => None,
        };

        let mut text = None;
        if asset.is_none() {
            text = Some(db_entry.to_string());
        }

        Self {
            name,
            text,
            asset,
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let data = match &self.asset {
            Some(asset) => asset.to_string(),
            None => match &self.text {
                Some(text) => text.to_owned(),
                None => "".to_owned(),
            },
        };
        format!("{}{}{}", self.name, SEPARATOR3, data)
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut splitter = input.split(SEPARATOR3);
        let Some(name) = splitter.next() else {return Err(())};
        let data = splitter.next().unwrap_or_else(|| "");
        match splitter.next() {
            Some(_) => return Err(()),
            None => (),
        }

        let asset = match Asset::from_str(data) {
            Ok(asset) => Some(asset),
            Err(_) => None,
        };

        let mut text = None;
        if asset.is_none() {
            text = Some(data.to_string());
        }

        Ok(Self::new(name.to_string(), text, asset))
    }
}

#[derive(Debug, Clone, Default)]
pub struct NoteList {
    pub notes: Vec<Note>
}

impl ToString for NoteList {
    fn to_string(&self) -> String {
        let mut notes_str = String::new();
        for (i, note) in self.iter().enumerate() {
            if i == 0 {
                notes_str.push_str(&note.to_string());
            } else {
                notes_str.push_str(SEPARATOR5);
                notes_str.push_str(&note.to_string());
            }
        }
        notes_str
    }
}

impl FromStr for NoteList {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let note_strs = input.split(SEPARATOR5);
        let mut note_list = Self::default();
        for note_str in note_strs{
            let Ok(note) = Note::from_str(note_str) else {return Err(())};
            note_list.push(note);
        }
        Ok(note_list)
    }
}

impl Deref for NoteList {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.notes
    }
}

impl DerefMut for NoteList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notes
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, EnumIter, EnumCount, Display, Serialize, Deserialize)]
pub enum NoteType {
    #[default] WhatDa,
    HellNah,
}

impl FromStr for NoteType {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        for variant in Self::iter() {
            if input == &variant.to_string() {
                return Ok(variant);
            }
        }
        Err(())
    }
}
