#[macro_export]
macro_rules! debug_unreachable {
	() => {
		$crate::debug_unreachable!("entered unreachable code")
	};
	($e:expr) => {
		if cfg!(not(debug_assertions)) {
			unsafe { std::hint::unreachable_unchecked() };
		} else {
			panic!($e)
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
	}
}

#[macro_export]
macro_rules! cloned {
	(($($arg:ident),*) => $e:expr) => {{
		$( let $arg = ::std::clone::Clone::clone(&$arg); )*

		$e
	}}
}