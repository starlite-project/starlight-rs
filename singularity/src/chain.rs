use std::{error::Error, vec};

pub struct Chain<'a> {
	state: ChainState<'a>,
}

impl<'a> Chain<'a> {
	pub fn new(head: &'a (dyn Error + 'static)) -> Self {
		Self {
			state: ChainState::Linked { next: Some(head) },
		}
	}
}

impl<'a> Iterator for Chain<'a> {
	type Item = &'a (dyn Error + 'static);

	fn next(&mut self) -> Option<Self::Item> {
		match &mut self.state {
			ChainState::Linked { next } => {
				let error = (*next)?;
				*next = error.source();
				Some(error)
			}
			ChainState::Buffered { rest } => rest.next(),
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl DoubleEndedIterator for Chain<'_> {
	fn next_back(&mut self) -> Option<Self::Item> {
		match &mut self.state {
			ChainState::Linked { mut next } => {
				let mut rest = Vec::new();
				while let Some(cause) = next {
					next = cause.source();
					rest.push(cause);
				}

				let mut rest = rest.into_iter();
				let last = rest.next_back();
				self.state = ChainState::Buffered { rest };
				last
			}
			ChainState::Buffered { rest } => rest.next_back(),
		}
	}
}

impl ExactSizeIterator for Chain<'_> {
	fn len(&self) -> usize {
		match &self.state {
			ChainState::Linked { mut next } => {
				let mut len = 0;
				while let Some(cause) = next {
					next = cause.source();
					len += 1;
				}
				len
			}
			ChainState::Buffered { rest } => rest.len(),
		}
	}
}

impl Default for Chain<'_> {
	fn default() -> Self {
		Self {
			state: ChainState::Buffered {
				rest: Vec::new().into_iter(),
			},
		}
	}
}

#[derive(Clone)]
pub(crate) enum ChainState<'a> {
	Linked {
		next: Option<&'a (dyn Error + 'static)>,
	},
	Buffered {
		rest: vec::IntoIter<&'a (dyn Error + 'static)>,
	},
}
