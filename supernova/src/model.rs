use thiserror::Error;
use twilight_http::{
	response::{marker::ListBody, DeserializeBodyError, ResponseFuture},
	Error as HttpError,
};

#[derive(Debug, Error)]
pub enum ModelError {
	#[error(transparent)]
	HttpError(#[from] HttpError),
	#[error(transparent)]
	DeserializeBody(#[from] DeserializeBodyError),
}

#[macro_export]
macro_rules! model {
	($input:ident as $ty:ty) => {{
		let response_future: twilight_http::response::ResponseFuture<$ty> = $input.exec();
		$crate::model::unravel::<$ty>(response_future)
	}};
	($input:ident as list of $ty:ty) => {{
		let response_future: twilight_http::response::ResponseFuture<
			twilight_http::response::marker::ListBody<$ty>,
		> = $input.exec();
		$crate::model::unravel_many::<$ty>(response_future)
	}};
}

pub async fn unravel<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<T>,
) -> Result<T, ModelError> {
	Ok(future.await?.model().await?)
}

pub async fn unravel_many<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<ListBody<T>>,
) -> Result<Vec<T>, ModelError> {
	Ok(future.await?.models().await?)
}

pub trait ModelInput: super::private::Sealed {}

impl<T: super::private::Sealed> ModelInput for T {}
