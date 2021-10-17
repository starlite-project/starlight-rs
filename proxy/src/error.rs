use http::{uri::InvalidUri, Method};
use hyper::Error as HyperError;
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;
use twilight_http::routing::PathParseError;

#[derive(Debug, Error)]
pub enum RequestError {
	#[error("error when acquiring ratelimiting ticket: {source}")]
	AcquiringTicket {
		#[from]
		source: RecvError,
	},
	#[error("invalid method: {method}")]
	InvalidMethod { method: Method },
	#[error("invalid path: {source}")]
	InvalidPath {
		#[from]
		source: PathParseError,
	},
	#[error("generated uri for discord api is invalid: {source}")]
	InvalidURI {
		#[from]
		source: InvalidUri,
	},
	#[error("error execuring request: {source}")]
	RequestIssue {
		#[from]
		source: HyperError,
	},
}
