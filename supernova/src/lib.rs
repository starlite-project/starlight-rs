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
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(cloned!(@param $p),)+| $body
        }
    );
}

#[cfg(test)]
mod tests {
	use super::{cloned, debug_unreachable, text, bytes, model};
	use std::error::Error;

	type TestResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

	#[test]
	fn cloned() {
		let name = String::from("Ferris");

		let three_letters = cloned!(name => move || {
			name.split("").take(4).collect::<String>()
		});

		assert_eq!(three_letters(), String::from("Fer"));
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
	}

	impl<const N: usize> ResponseFuture<N> {
		const fn new(bytes: [u8; N]) -> Self {
			Self { inner: bytes }
		}

		async fn exec(&self) -> TestResult<Response> {
			Ok(Response {
				inner: self.inner.clone().to_vec(),
			})
		}
	}

	struct Response {
		inner: Vec<u8>,
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
	}

	struct Message {
		content: String,
	}

	const RESPONSE_FUTURE: ResponseFuture<13> = ResponseFuture::new([72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]);

	#[tokio::test]
	async fn bytes() -> Result<(), Box<dyn Error + Send + Sync>> {
		let decoded = bytes!(RESPONSE_FUTURE);

		assert_eq!(decoded, vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]);

		Ok(())
	}
}
