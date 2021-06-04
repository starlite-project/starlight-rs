use super::ParseMention;
use std::{iter::Iterator, marker::PhantomData, str::CharIndices};

#[derive(Debug, Clone)]
pub struct MentionIter<'a, T> {
    buf: &'a str,
    chars: CharIndices<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T> MentionIter<'a, T> {
    #[must_use]
    pub(in crate::parse) fn new(buf: &'a str) -> Self {
        let chars = buf.char_indices();

        Self {
            buf,
            chars,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.buf
    }
}

impl<'a, T:ParseMention> Iterator for MentionIter<'a, T> {
    type Item = (T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let start = match self.chars.next()? {
                (idx, '<') => idx,
                _ => continue
            };

            let mut found = false;

            for sigil in T::SIGILS {
                if self.chars.as_str().starts_with(sigil) {
                    found = true;

                    for _ in 0..sigil.chars().count() {
                        self.chars.next();
                    }
                }
            }

            if !found {
                continue;
            }

            let end = match self.chars.find(|c| c.1 == '>') {
                Some((idx, _)) => idx,
                None => continue
            };

            let buf = self.buf.get(start..=end)?;

            if let Ok(id) = T::parse(buf) {
                return Some((id, start, end));
            }
        }
    }
}