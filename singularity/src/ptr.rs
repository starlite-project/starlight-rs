use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Own<T>
where
	T: ?Sized,
{
	pub ptr: NonNull<T>,
}

impl<T> Own<T>
where
	T: ?Sized,
{
	pub fn new(ptr: Box<T>) -> Self {
		Self {
			ptr: unsafe { NonNull::new_unchecked(Box::into_raw(ptr)) },
		}
	}

	pub fn cast<U: CastTo>(self) -> Own<U::Target> {
		Own {
			ptr: self.ptr.cast(),
		}
	}

	pub unsafe fn boxed(self) -> Box<T> {
		Box::from_raw(self.ptr.as_ptr())
	}

	pub fn by_ref(&self) -> Ref<T> {
		Ref {
			ptr: self.ptr,
			lifetime: PhantomData,
		}
	}

	pub fn by_mut(&mut self) -> Mut<T> {
		Mut {
			ptr: self.ptr,
			lifetime: PhantomData,
		}
	}
}

unsafe impl<T> Send for Own<T> where T: ?Sized {}
unsafe impl<T> Sync for Own<T> where T: ?Sized {}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Ref<'a, T>
where
	T: ?Sized,
{
	pub ptr: NonNull<T>,
	lifetime: PhantomData<&'a T>,
}

impl<'a, T> Ref<'a, T>
where
	T: ?Sized,
{
	pub fn new(ptr: &'a T) -> Self {
		Self {
			ptr: NonNull::from(ptr),
			lifetime: PhantomData,
		}
	}

	#[cfg(not(singularity_no_ptr_addr_of))]
	pub fn from_raw(ptr: NonNull<T>) -> Self {
		Self {
			ptr,
			lifetime: PhantomData,
		}
	}

	pub fn cast<U: CastTo>(self) -> Ref<'a, U::Target> {
		Ref {
			ptr: self.ptr.cast(),
			lifetime: PhantomData,
		}
	}

    #[cfg(not(singularity_no_ptr_addr_of))]
    pub fn by_mut(self) -> Mut<'a, T> {
        Mut {
            ptr: self.ptr,
            lifetime: PhantomData,
        }
    }

	#[cfg(not(singularity_no_ptr_addr_of))]
	pub fn as_ptr(self) -> *const T {
		self.ptr.as_ptr() as *const T
	}

	pub unsafe fn deref(self) -> &'a T {
		&*self.ptr.as_ptr()
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Mut<'a, T>
where
	T: ?Sized,
{
	pub ptr: NonNull<T>,
	lifetime: PhantomData<&'a mut T>,
}

impl<'a, T> Mut<'a, T>
where
	T: ?Sized,
{
	#[cfg(not(singularity_no_ptr_addr_of))]
	pub fn new(ptr: &'a mut T) -> Self {
		Self {
			ptr: NonNull::from(ptr),
			lifetime: PhantomData,
		}
	}

	pub fn cast<U: CastTo>(self) -> Mut<'a, U::Target> {
		Mut {
			ptr: self.ptr.cast(),
			lifetime: PhantomData,
		}
	}

	#[cfg(not(singularity_no_ptr_addr_of))]
	pub fn by_ref(self) -> Ref<'a, T> {
		Ref {
			ptr: self.ptr,
			lifetime: PhantomData,
		}
	}

	pub fn extend<'b>(self) -> Mut<'b, T> {
		Mut {
			ptr: self.ptr,
			lifetime: PhantomData,
		}
	}

	pub unsafe fn deref_mut(self) -> &'a mut T {
		&mut *self.ptr.as_ptr()
	}
}

impl<'a, T> Mut<'a, T> {
	pub unsafe fn read(self) -> T {
		self.ptr.as_ptr().read()
	}
}

pub trait CastTo {
	type Target;
}

impl<T> CastTo for T {
	type Target = T;
}
