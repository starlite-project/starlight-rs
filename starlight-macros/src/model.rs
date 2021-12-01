use serde::de::DeserializeOwned;
use thiserror::Error;
use twilight_http::{
	response::{marker::ListBody, DeserializeBodyError, ResponseFuture},
	Error as HttpError,
};
use twilight_model::{
	channel::{Channel, Message},
	guild::{Emoji, Member, Role},
	user::{CurrentUser, User},
};

#[derive(Debug, Error)]
pub enum ModelError {
	#[error(transparent)]
	Http(#[from] HttpError),
	#[error(transparent)]
	DeserializeBody(#[from] DeserializeBodyError),
}

#[macro_export]
macro_rules! model {
	($input:ident) => {{
		$crate::model!($input as _)
	}};
	($input:ident as list) => {{
		$crate::model!($input as list of _)
	}};
	($input:ident as $ty:ty) => {{
		let response_future: ::twilight_http::response::ResponseFuture<$ty> = $input.exec();
		$crate::model::unravel::<$ty>(response_future)
	}};
	($input:ident as list of $ty:ty) => {{
		let response_future: ::twilight_http::response::ResponseFuture<
			twilight_http::response::marker::ListBody<$ty>,
		> = $input.exec();
		$crate::model::unravel_many::<$ty>(response_future)
	}};
}

pub async fn __unravel<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<T>
) -> Result<T, ModelError> {
	Ok(future.await?.model().await?)
}

pub async fn __unravel_many<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<ListBody<T>>,
) -> Result<Vec<T>, ModelError> {
	Ok(future.await?.models().await?)
}

pub trait ModelInput: super::private::Sealed + DeserializeOwned + Sized {}

impl ModelInput for Message {}
impl ModelInput for Role {}
impl ModelInput for CurrentUser {}
impl ModelInput for Emoji {}
impl ModelInput for User {}
impl ModelInput for Channel {}
impl ModelInput for Member {}

impl<T: ModelInput> ModelInput for Option<T> {}
impl<T: ModelInput> ModelInput for Vec<T> {}
