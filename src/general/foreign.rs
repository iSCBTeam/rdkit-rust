use crate::ffi;

pub mod prelude {
	pub use super::AsForeignOptionI32;
	pub use super::AsForeignOptionU32;
	pub use super::AsForeignOptionUSize;
	pub use super::AsForeignOptionBool;
}

pub trait AsForeignOptionI32 {
	fn as_foreign(self) -> i32;
}

impl AsForeignOptionI32 for Option<i32> {
	fn as_foreign(self) -> i32 {
		match self {
			None => ffi::RDKIT_DEFAULT_I32,
			Some(x) => x,
		}
	}
}

pub trait AsForeignOptionU32 {
	fn as_foreign(self) -> u32;
}

impl AsForeignOptionU32 for Option<u32> {
	fn as_foreign(self) -> u32 {
		match self {
			None => ffi::RDKIT_DEFAULT_U32,
			Some(x) => x,
		}
	}
}

pub trait AsForeignOptionUSize {
	fn as_foreign(self) -> usize;
}

impl AsForeignOptionUSize for Option<usize> {
	fn as_foreign(self) -> usize {
		match self {
			None => ffi::RDKIT_DEFAULT_SIZE,
			Some(x) => x,
		}
	}
}

pub trait AsForeignOptionBool {
	fn as_foreign(self) -> ffi::rdkit_tribool;
}

impl AsForeignOptionBool for Option<bool> {
	fn as_foreign(self) -> ffi::rdkit_tribool {
		match self {
			None => ffi::RDKIT_DEFAULT_TRIBOOL,
			Some(false) => ffi::RDKIT_FALSE,
			Some(true) => ffi::RDKIT_TRUE,
		}
	}
}
