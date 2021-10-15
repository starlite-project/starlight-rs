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
/// An error returned from the [`model`] macro.
///
/// [`model`]: crate::model!
#[derive(Debug, Error)]
pub enum ModelError {
	/// An [`HttpError`] has occurred.
	///
	/// [`HttpError`]: twilight_http::Error
	#[error(transparent)]
	HttpError(#[from] HttpError),
	/// A [`DeserializeBodyError`] has occurred.
	///
	/// [`DeserializeBodyError`]: twilight_http::response::DeserializeBodyError
	#[error(transparent)]
	DeserializeBody(#[from] DeserializeBodyError),
}

/// Parse a [`ResponseFuture`] into a value, returning a [`ModelError`] if failing.
///
/// This macro return a [`Future<Output = Result<T, ModelError>>`].
///
/// [`ResponseFuture`]: twilight_http::response::ResponseFuture
/// [`ModelError`]: crate::ModelError
/// [`Future<Output = Result<T, ModelError>>`]: std::future::Future
/// <br>
///
/// # Explicit vs Implicit
///
/// Explicit deserializing is done with the `as` operator, as if doing primitive casting.
///
/// Implicit deserializing is done by just passing the variable into the macro.
///
/// Implicit deserializing requires assistance to infer the type properly,
/// such as via return type or explicitly typing the variable.
///
/// <br>
/// 
/// # Lists
/// 
/// If the [`ResponseFuture`] passed has a [`ListBody`], then you must deserialize into a [`Vec`] of items.
/// 
/// Deserializing into a [`Vec`] can be done with `as list( of)`.
/// 
/// [`ResponseFuture`]: twilight_http::response::ResponseFuture
/// [`ListBody`]: twilight_http::response::marker::ListBody
/// [`Vec`]: std::vec::Vec
/// 
/// <br>
/// 
/// # Examples
/// 
/// Explicit deserializing a list.
/// ```no_run
/// use supernova::model;
/// use twilight_model::{
///     channel::Message,
///     id::{ChannelId, MessageId},
/// };
/// use twilight_http::Client;
///
/// # fn get_client() -> Client {
/// # unreachable!()
/// # }
///
/// # #[tokio::main(flavor = "current_thread")] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = get_client();
///
/// let message_future = client.channel_messages(ChannelId(123));
/// 
/// let messages = model!(message_future as list of Message).await?;
/// 
/// assert!(!messages.is_empty());
///
/// # Ok(()) }
/// ```
/// 
/// Implicit deserializing.
/// ```no_run
/// use supernova::model;
/// use twilight_model::{
///     channel::Message,
///     id::{ChannelId, MessageId},
/// };
/// use twilight_http::Client;
///
/// # fn get_client() -> Client {
/// # unreachable!()
/// # }
/// # #[tokio::main(flavor = "current_thread")] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = get_client();
/// 
/// let message_future = client.message(ChannelId(456), MessageId(789));
/// 
/// let message: Message = model!(message_future).await?;
/// 
/// assert_eq!(message.content, String::from("Hello, world!"));
/// 
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! model {
	($input:ident) => {{
		$crate::model!($input as _)
	}};
	($input:ident as list) => {{
		$crate::model!($input as list of _)
	}};
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

#[doc(hidden)]
pub async fn unravel<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<T>,
) -> Result<T, ModelError> {
	Ok(future.await?.model().await?)
}

#[doc(hidden)]
pub async fn unravel_many<T: ModelInput + Unpin + Send>(
	future: ResponseFuture<ListBody<T>>,
) -> Result<Vec<T>, ModelError> {
	Ok(future.await?.models().await?)
}

/// An input for the [`model`] macro.
///
/// This trait is currently sealed and private.
///
/// [`model`]: crate::model!
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
