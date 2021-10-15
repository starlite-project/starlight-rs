#![deny(clippy::all, missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::module_name_repetitions)]
//! todo

#[doc(hidden)]
pub mod model;

#[doc(inline)]
pub use model::{ModelError, ModelInput};

mod private {
	use twilight_model::{
		channel::{Channel, Message},
		guild::{Emoji, Member, Role},
		user::{CurrentUser, User},
	};

	pub trait Sealed {}

	impl Sealed for Message {}
	impl Sealed for Role {}
	impl Sealed for CurrentUser {}
	impl Sealed for Emoji {}
	impl Sealed for User {}
	impl Sealed for Channel {}
	impl Sealed for Member {}

	impl<T: Sealed> Sealed for Option<T> {}
	impl<T: Sealed> Sealed for Vec<T> {}
}

/// todo
#[macro_export]
macro_rules! debug_unreachable {
	() => {
		$crate::debug_unreachable!("entered unreachable code")
	};
	($($arg:tt)*) => {
		if cfg!(not(debug_assertions)) {
			unsafe { std::hint::unreachable_unchecked() };
		} else {
			panic!($($arg)*)
		}
	};
}

/// todo
#[macro_export]
macro_rules! status {
	($request:expr) => {
		$crate::finish_request!($request, status)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, status)
	};
}

/// todo
#[macro_export]
macro_rules! text {
	($request:expr) => {
		$crate::finish_request!($request, text)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, text)
	};
}

/// todo
#[macro_export]
macro_rules! bytes {
	($request:expr) => {
		$crate::finish_request!($request, bytes)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, bytes)
	}
}

/// todo
#[macro_export]
macro_rules! finish_request {
	($request:expr, status) => {{
		$request.exec().await?.status()
	}};
	(@diagnostic $request:expr, status) => {{
		use ::miette::IntoDiagnostic;
		$request.exec().await.into_diagnostic()?.status()
	}};
	($request:expr, $type:ident) => {{
		$request.exec().await?.$type().await?
	}};
	(@diagnostic $request:expr, $type:ident) => {{
		use ::miette::IntoDiagnostic;
		$request
			.exec()
			.await
			.into_diagnostic()?
			.$type()
			.await
			.into_diagnostic()?
	}};
}

/// todo
#[macro_export]
macro_rules! cloned {
	(@param $n:ident) => (
		::std::clone::Clone::clone(&$n)
	);
	($n:ident => $e:expr) => (
		{
			let $n = cloned!(@param $n);

			$e
		}
	);
	(($($n:ident),+) => $e:expr) => (
		{
			$( let $n = cloned!(@param $n); )+

			$e
		}
	);
}

#[cfg(test)]
mod tests {
	use std::string::FromUtf8Error;

	use super::{bytes, cloned, debug_unreachable, status, text};
	use thiserror::Error;

	#[derive(Debug, Error, Clone, Copy)]
	#[error("test error")]
	struct TestError;

	impl From<FromUtf8Error> for TestError {
		fn from(_: FromUtf8Error) -> Self {
			Self
		}
	}

	type TestResult<T = ()> = std::result::Result<T, TestError>;
	type DiagnosticResult<T = ()> = miette::Result<T>;

	#[test]
	fn cloned() {
		let name = String::from("Ferris");

		let three_letters = cloned!(name => move || {
			name.split("").take(4).collect::<String>()
		});

		assert_eq!(three_letters(), String::from("Fer"));
	}

	#[test]
	fn cloned_with_args() {
		let value = 10;

		let add = cloned!(value => move |to_add: u32| {
			value + to_add
		});

		assert_eq!(add(10), 20);
	}

	#[test]
	fn cloned_with_multiple_args() {
		let value = 10;

		let add_and_multiply = cloned!(value => move |to_add: u32, to_multiply: u32| {
			(value + to_add) * to_multiply
		});

		assert_eq!(add_and_multiply(10, 2), 40);
	}

	#[test]
	fn cloned_with_multiple_values() {
		let first_name = "John".to_string();
		let last_name = "Doe";

		let combine_names = cloned!((first_name, last_name) => move || {
			first_name + " " + last_name
		});

		assert_eq!(combine_names(), "John Doe");
	}

	#[test]
	fn cloned_with_return_type() {
		let value = String::from("Hello, world!");

		let reverse_value = cloned!(value => move || -> String {
			value.chars().rev().collect()
		});

		assert_eq!(reverse_value(), String::from("!dlrow ,olleH"));
	}

	#[test]
	#[should_panic]
	fn debug_unreachable() {
		let _: u8 = None.unwrap_or_else(|| debug_unreachable!());
	}

	#[test]
	#[should_panic = "foo"]
	fn debug_unreachable_with_message() {
		let _: u8 = Err("foo").unwrap_or_else(|err| debug_unreachable!("{}", err));
	}

	#[derive(Debug, Clone, Copy)]
	struct ResponseFuture<const N: usize> {
		inner: [u8; N],
		code: u16,
	}

	impl<const N: usize> ResponseFuture<N> {
		const fn new(inner: [u8; N], code: u16) -> Self {
			Self { inner, code }
		}

		async fn exec(&self) -> TestResult<Response> {
			Ok(Response {
				inner: self.inner.clone().to_vec(),
				code: self.code,
			})
		}
	}

	#[derive(Debug, Clone)]
	struct Response {
		inner: Vec<u8>,
		code: u16,
	}

	impl Response {
		async fn bytes(&self) -> TestResult<Vec<u8>> {
			Ok(self.inner.clone())
		}

		async fn text(&self) -> TestResult<String> {
			Ok(String::from_utf8(self.inner.clone())?)
		}

		async fn random(&self) -> TestResult<u8> {
			Ok(10)
		}

		const fn status(&self) -> u16 {
			self.code
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
	struct Message {
		content: String,
	}

	const RESPONSE_FUTURE: ResponseFuture<13> = ResponseFuture::new(
		[72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33],
		200,
	);

	#[tokio::test]
	async fn bytes() -> TestResult {
		let decoded = bytes!(RESPONSE_FUTURE);

		assert_eq!(
			decoded,
			vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
		);

		Ok(())
	}

	#[tokio::test]
	async fn bytes_diagnostic() -> DiagnosticResult {
		let decoded = bytes!(@diagnostic RESPONSE_FUTURE);

		assert_eq!(
			decoded,
			vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
		);

		Ok(())
	}

	#[tokio::test]
	async fn text() -> TestResult {
		let decoded = text!(RESPONSE_FUTURE);

		assert_eq!(decoded, String::from("Hello, world!"));

		Ok(())
	}

	#[tokio::test]
	async fn text_diagnostic() -> DiagnosticResult {
		let decoded = text!(@diagnostic RESPONSE_FUTURE);

		assert_eq!(decoded, String::from("Hello, world!"));

		Ok(())
	}

	#[tokio::test]
	async fn status() -> TestResult {
		let decoded = status!(RESPONSE_FUTURE);

		assert_eq!(decoded, 200);

		Ok(())
	}

	#[tokio::test]
	async fn status_diagnostic() -> DiagnosticResult {
		let decoded = status!(@diagnostic RESPONSE_FUTURE);

		assert_eq!(decoded, 200);

		Ok(())
	}

	#[tokio::test]
	async fn status_failed() -> TestResult {
		let failed = ResponseFuture::new([], 404);

		let decoded = status!(failed);

		assert_eq!(decoded, 404);

		Ok(())
	}

	#[tokio::test]
	async fn finish_request() -> TestResult {
		let finished = finish_request!(RESPONSE_FUTURE, random);

		assert_eq!(finished, 10);

		Ok(())
	}
}
