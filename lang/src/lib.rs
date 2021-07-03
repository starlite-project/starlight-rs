#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod error;

use self::error::LanguageErrorType;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    io::BufReader,
    path::Path,
};

pub use self::error::LanguageError;

pub type LanguageResult<T> = Result<T, LanguageError>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LangMap(HashMap<String, Language>);

impl LangMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_dir<P: AsRef<Path>>(path: P) -> LanguageResult<Self> {
        Self::try_from(fs::read_dir(path)?)
    }

    pub fn get<K: Into<String>>(&self, key: K) -> Option<Language> {
        lang_get(&self.0, key)
    }

    fn insert<P: Into<String>>(&mut self, filename: P, entry: Language) -> &mut Self {
        self.0.insert(filename.into(), entry);

        self
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Language(HashMap<String, LanguageEntry>);

impl Language {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<P: Into<String>>(&self, key: P) -> Option<LanguageEntry> {
        lang_get(&self.0, key)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguageEntry {
    base: String,
    params: Vec<String>,
}

impl LanguageEntry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&self) -> LanguageResult<String> {
        if !self.params.is_empty() {
            return Err((self.params.len(), 0_usize).into());
        }

        Ok(self.base.clone())
    }

    pub fn write_params(&self, params: &[&str]) -> LanguageResult<String> {
        if params.len() != self.params.len() {
            return Err((self.params.len(), params.len()).into());
        }

        let mut base = self.base.clone();

        for (idx, value) in self.params.clone().into_iter().enumerate() {
            let formatter: LangFormatter = value.into();

            base = base.replace(formatter.to_string().as_str(), params.get(idx).unwrap());
        }

        Ok(base)
    }
}

fn lang_get<K: Into<String>, Rt: Clone>(map: &HashMap<String, Rt>, key: K) -> Option<Rt> {
    map.get(&key.into()).cloned()
}

impl TryFrom<fs::ReadDir> for LangMap {
    type Error = LanguageError;

    fn try_from(dir: fs::ReadDir) -> Result<Self, Self::Error> {
        let mut entries = Vec::new();

        for file in dir {
            let entry = file?;
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "json" {
                    entries.push(path);
                }
            }
        }

        let mut lang_map = Self::new();

        for entry in entries {
            let filename = match entry.clone().file_name() {
                Some(filename) => filename.to_string_lossy().to_string().replace(".json", ""),
                None => {
                    return Err(LanguageError {
                        kind: LanguageErrorType::DirectoryFound,
                        source: None,
                    })
                }
            };

            let file = fs::OpenOptions::new()
                .create(false)
                .write(false)
                .append(false)
                .read(true)
                .open(entry)?;
            let reader = BufReader::new(file);

            lang_map.insert(filename, serde_json::from_reader(reader)?);
        }

        Ok(lang_map)
    }
}

#[derive(Debug, Clone)]
struct LangFormatter(String);

impl Display for LangFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("[")?;

        Display::fmt(&self.0, f)?;

        f.write_str("]")
    }
}

impl From<String> for LangFormatter {
    fn from(val: String) -> Self {
        Self(val)
    }
}
