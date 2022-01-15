//
// Inspired by code shared by FreeFull in Discord.
//

pub struct Wrapper<T>
{
	inner: std::cell::UnsafeCell<T>,
}

/// # Safety
/// Relies on the program being single-threaded.
unsafe impl<T: Send> Sync for Wrapper<T> {}

impl<T> Wrapper<T>
{
	pub const fn new(inner: T) -> Self
	{
		Self {
			inner: std::cell::UnsafeCell::new(inner),
		}
	}

	pub fn get_mut(&self) -> &mut T
	{
		unsafe { &mut *self.inner.get() }
	}
}
