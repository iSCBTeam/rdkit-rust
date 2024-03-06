use std::borrow::Borrow;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::ffi;

pub mod prelude {
	pub use super::RDstr;
	pub use super::AsForeignOptionRDStr;
	pub use super::RDString;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RDstr<'s> {
	obj: ffi::rdkit_string,
	_obj_lifetime: PhantomData<&'s ffi::rdkit_string>,
}

impl<'s> RDstr<'s> {
	pub fn as_bytes(&self) -> &[u8] {
		self.into()
	}

	pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
		std::str::from_utf8(self.as_bytes())
	}

	pub(crate) fn as_foreign(&self) -> &ffi::rdkit_string {
		&self.obj
	}
}

impl<'s> Into<&'s [u8]> for &RDstr<'s> {
	fn into(self) -> &'s [u8] {
		unsafe { std::slice::from_raw_parts(self.obj.s as *const u8, self.obj.len) }
	}
}

impl<'s> From<&'s [u8]> for RDstr<'s> {
	fn from(value: &[u8]) -> Self {
		RDstr {
			obj: ffi::rdkit_string {
				s: value.as_ptr() as *const i8,
				len: value.len(),
			},
			_obj_lifetime: PhantomData,
		}
	}
}

impl<'s> From<&'s str> for RDstr<'s> {
	fn from(value: &'s str) -> Self {
		Self::from(value.as_bytes())
	}
}

impl<'s> ToOwned for RDstr<'s> {
	type Owned = RDString;

	fn clone_into(&self, target: &mut Self::Owned) {
		unsafe {
			ffi::rdkit_string_owned_dtor(&mut target.foreign);
			ffi::rdkit_string_to_owned(&self.obj, &mut target.foreign);
		}
	}
	fn to_owned(&self) -> Self::Owned {
		let mut uninit = MaybeUninit::<Self::Owned>::uninit();
		let ptr = uninit.as_mut_ptr();
		unsafe {
			ffi::rdkit_string_to_owned(&self.obj, &mut (*ptr).foreign);
			uninit.assume_init()
		}
	}
}

pub trait AsForeignOptionRDStr {
	fn as_foreign(self) -> *const ffi::rdkit_string;
}

impl AsForeignOptionRDStr for Option<RDstr<'_>> {
	fn as_foreign(self) -> *const ffi::rdkit_string {
		match self {
			None => std::ptr::null(),
			Some(x) => x.as_foreign(),
		}
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RDString {
	foreign: ffi::rdkit_string_owned,
}
impl RDString {
	pub fn as_rdstr(&self) -> &RDstr {
		self.borrow()
	}
	pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
		self.as_rdstr().as_str()
	}

	pub(crate) fn as_foreign_ptr(ptr: *const Self) -> *const ffi::rdkit_string_owned {
		unsafe { std::ptr::addr_of!((*ptr).foreign) }
	}
	pub(crate) fn as_foreign_ptr_mut(ptr: *mut Self) -> *mut ffi::rdkit_string_owned {
		unsafe { std::ptr::addr_of_mut!((*ptr).foreign) }
	}
}

impl Drop for RDString {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_string_owned_dtor(&mut self.foreign); }
	}
}

impl<'s> Borrow<RDstr<'s>> for RDString {
	fn borrow(&self) -> &RDstr<'s> {
		unsafe { std::mem::transmute(&self.foreign) }
	}
}

impl Clone for RDString {
	fn clone(&self) -> Self {
		self.as_rdstr().to_owned()
	}
}

impl<'s> From<&'s RDString> for &'s ffi::rdkit_string_owned {
	fn from(value: &'s RDString) -> Self {
		&value.foreign
	}
}
impl<'s> From<&'s mut RDString> for &'s mut ffi::rdkit_string_owned {
	fn from(value: &'s mut RDString) -> Self {
		&mut value.foreign
	}
}

impl<'s> From<&'s ffi::rdkit_string_owned> for &'s RDString {
	fn from(value: &'s ffi::rdkit_string_owned) -> Self {
		unsafe { std::mem::transmute(value) }
	}
}
impl<'s> From<&'s mut ffi::rdkit_string_owned> for &'s mut RDString {
	fn from(value: &'s mut ffi::rdkit_string_owned) -> Self {
		unsafe { std::mem::transmute(value) }
	}
}

impl From<&RDstr<'_>> for RDString {
	fn from(value: &RDstr) -> Self {
		value.to_owned()
	}
}
impl From<&mut RDstr<'_>> for RDString {
	fn from(value: &mut RDstr) -> Self {
		value.to_owned()
	}
}
