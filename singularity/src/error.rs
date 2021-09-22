#[cfg(singularity_no_ptr_addr_of)]
use crate::ptr::Mut;
use crate::{
	backtrace::Backtrace,
	chain::Chain,
	ptr::{Own, Ref},
};
#[cfg(singularity_no_ptr_addr_of)]
use std::ptr;
use std::{
	any::TypeId,
	error::Error as StdError,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	mem::ManuallyDrop,
	ptr::NonNull,
    ops::{Deref, DerefMut}
};

