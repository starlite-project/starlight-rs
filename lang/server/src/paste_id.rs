use rand::{self, Rng};
use rocket::{http::uri::fmt, request::FromParam, UriDisplayPath};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

impl PasteId<'_> {
    pub fn new(size: usize) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteId(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a str;

    fn from_param(params: &'a str) -> Result<Self, Self::Error> {
        params
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(params.into()))
            .ok_or(params)
    }
}

impl<'a> fmt::FromUriParam<fmt::Path, &'a str> for PasteId<'_> {
    type Target = PasteId<'a>;

    fn from_uri_param(param: &'a str) -> Self::Target {
        PasteId(param.into())
    }
}
