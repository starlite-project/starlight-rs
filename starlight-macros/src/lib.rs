pub mod model;

#[doc(inline)]
pub use model::{ModelError, ModelInput};

mod private {
	use twilight_model::{
		channel::{Channel, Message},
		guild::{Emoji, Member, Role},
		oauth::current_application_info::CurrentApplicationInfo,
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
	impl Sealed for CurrentApplicationInfo {}

	impl<T: Sealed> Sealed for Option<T> {}
	impl<T: Sealed> Sealed for Vec<T> {}
}

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
macro_rules! status {
	($request:expr) => {
		$crate::finish_request!($request, status)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, status)
	};
}

#[macro_export]
macro_rules! text {
	($request:expr) => {
		$crate::finish_request!($request, text)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, text)
	};
}

#[macro_export]
macro_rules! bytes {
	($request:expr) => {
		$crate::finish_request!($request, bytes)
	};
	(@diagnostic $request:expr) => {
		$crate::finish_request!(@diagnostic $request, bytes)
	}
}

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
