use std::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
};

#[repr(transparent)]
pub struct MessageError<M>(pub M);

impl<M> Debug for MessageError<M>
where
	M: Display + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Debug::fmt(&self.0, f)
	}
}

impl<M> Display for MessageError<M>
where
	M: Display + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.0, f)
	}
}

impl<M> Error for MessageError<M> where M: Display + Debug + 'static {}

#[repr(transparent)]
pub struct DisplayError<M>(pub M);

impl<M> Debug for DisplayError<M> where M: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<M> Display for DisplayError<M> where M: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<M> Error for DisplayError<M> where M: Display + 'static {}

#[repr(transparent)]
pub struct BoxedError(pub Box<dyn Error + Send + Sync>);

impl Debug for BoxedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Display for BoxedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Error for BoxedError {
    fn backtrace(&self) -> Option<&crate::backtrace::Backtrace> {
        self.0.backtrace()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}