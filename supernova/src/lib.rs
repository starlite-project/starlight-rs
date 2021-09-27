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

#[macro_export]
macro_rules! model {
	($request:expr) => {
		$crate::finish_request!($request, model)
	};
}

#[macro_export]
macro_rules! status {
	($request:expr) => {
		$crate::finish_request!($request, status)
	};
}

#[macro_export]
macro_rules! text {
	($request:expr) => {
		$crate::finish_request!($request, text)
	};
}

#[macro_export]
macro_rules! bytes {
	($request:expr) => {
		$crate::finish_request!($request, bytes)
	};
}

#[macro_export]
macro_rules! finish_request {
	($request:expr, status) => {
		$request.exec().await?.status()
	};
	($request:expr, $type:ident) => {
		$request.exec().await?.$type().await?
	};
}

#[macro_export]
macro_rules! cloned {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
	($($n:ident),+ => move |$($p:tt$(: $t:ty)?),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(cloned!(@param $p)$(: $t)?,)+| $body
        }
    );
}

#[cfg(test)]
mod tests {
	use super::{bytes, cloned, debug_unreachable, model, status, text};
	use std::error::Error;

	type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

	#[test]
	fn cloned() {
		let name = String::from("Ferris");

		let three_letters = cloned!(name => move || {
			name.split("").take(4).collect::<String>()
		});

		assert_eq!(three_letters(), String::from("Fer"));
		let value = String::from("Hello, world!");

		let reverse_value = cloned!(value => move || {
			value.chars().rev().collect::<String>()
		});

		assert_eq!(reverse_value(), String::from("!dlrow ,olleH"));
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

		async fn model(&self) -> TestResult<Message> {
			Ok(Message {
				content: String::from_utf8(self.inner.clone())?,
			})
		}

		fn status(&self) -> u16 {
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
	async fn text() -> TestResult {
		let decoded = text!(RESPONSE_FUTURE);

		assert_eq!(decoded, String::from("Hello, world!"));

		Ok(())
	}

	#[tokio::test]
	async fn model() -> TestResult {
		let decoded = model!(RESPONSE_FUTURE);

		assert_eq!(
			decoded,
			Message {
				content: String::from("Hello, world!")
			}
		);

		Ok(())
	}

	#[tokio::test]
	async fn status() -> TestResult {
		let decoded = status!(RESPONSE_FUTURE);

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
}
