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
    sync::Arc,
};

#[macro_export]
macro_rules! i18n {
    ($map: expr, $lang: expr, $key: expr, $($args:expr),*) => {{
        // $map.get($lang).unwrap().get($key).unwrap().run_params(&[$($args),*]).unwrap()
        let lang = match $map.get($lang) {
            Some(lang) => Ok(lang),
            None => Err($crate::error::LanguageError::from($lang.to_string())),
        }?;

        let entry = match lang.get($key) {
            Some(entry) => Ok(entry),
            None => Err($crate::error::LanguageError::from(($lang.to_string(), $key.to_string()))),
        }?;

        entry.run_params(&[$($args),*])?
    }};
    ($map: expr, $lang: expr, $key: expr) => {{
        let lang = match $map.get($lang) {
            Some(lang) => Ok(lang),
            None => Err($crate::error::LanguageError::from($lang.to_string())),
        }?;

        let entry = match lang.get($key) {
            Some(entry) => Ok(entry),
            None => Err($crate::error::LanguageError::from(($lang.to_string(), $key.to_string())))
        }?;

        entry.run()?
    }}
}

pub use self::error::LanguageError;

pub type LanguageResult<T> = Result<T, LanguageError>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LangMap(HashMap<String, Arc<Language>>);

impl LangMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_dir<P: AsRef<Path>>(path: P) -> LanguageResult<Self> {
        Self::try_from(fs::read_dir(path)?)
    }

    pub fn get<K: Into<String>>(&self, key: K) -> Option<Arc<Language>> {
        lang_get(&self.0, key)
    }

    fn insert<P: Into<String>>(&mut self, filename: P, entry: Language) -> &mut Self {
        self.0.insert(filename.into(), Arc::new(entry));

        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language(HashMap<String, LanguageEntry>);

impl Language {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<P: Into<String>>(&self, key: P) -> Option<LanguageEntry> {
        lang_get(&self.0, key)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageEntry {
    base: String,
    params: Vec<String>,
}

impl LanguageEntry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) -> LanguageResult<String> {
        if !self.params.is_empty() {
            return Err((self.params.len(), 0_usize).into());
        }

        Ok(self.base.clone())
    }

    pub fn run_params(&self, params: &[&str]) -> LanguageResult<String> {
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

#[cfg(test)]
mod tests {
    use crate::LanguageResult;

    use super::{i18n, LangMap, Language};
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{collections::HashMap, convert::TryFrom, fmt::Debug, fs::ReadDir};

    assert_impl_all!(
        LangMap: Clone,
        Debug,
        Deserialize<'static>,
        Serialize,
        TryFrom<ReadDir>
    );

    #[test]
    fn new() {
        let test_map = LangMap::new();

        assert_eq!(test_map.0, HashMap::new());

        let test_lang = Language::new();

        assert_eq!(test_lang.0, HashMap::new());
    }

    #[test]
    fn from_dir() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        assert_eq!(map.0.len(), 1);

        Ok(())
    }

    #[test]
    fn map_get() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        let lang = map.get("en_us");

        assert!(lang.is_some());

        let not_a_lang = map.get("not a language");

        assert!(not_a_lang.is_none());

        Ok(())
    }

    #[test]
    fn lang_get() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        let lang = map.get("en_us").expect("something went wrong");

        let ping = lang.get("ping");

        assert!(ping.is_some());

        let nothing = lang.get("nothing");

        assert!(nothing.is_none());

        Ok(())
    }

    #[test]
    fn lang_entry_run() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        let lang = map.get("en_us").expect("something went wrong");

        let ping = lang.get("ping").expect("something else went wrong");

        assert_eq!(ping.run()?, "Ping...");

        Ok(())
    }

    #[test]
    fn lang_entry_run_params() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        let lang = map.get("en_us").expect("something went wrong");

        let pong = lang.get("pong").expect("something else went wrong");

        assert_eq!(pong.run_params(&["10"])?, "Pong! Took 10 milliseconds");

        Ok(())
    }

    #[test]
    fn i18n() -> LanguageResult<()> {
        let map = LangMap::from_dir("./test")?;

        assert_eq!(i18n!(map, "en_us", "ping"), "Ping...");

        assert_eq!(
            i18n!(map, "en_us", "pong", "10"),
            "Pong! Took 10 milliseconds"
        );

        Ok(())
    }
}
