use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::DepictorMutImpl;
}

pub trait DepictorMutImpl {
	fn compute_2d_coords(&mut self) -> u32;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> DepictorMutImpl for T {
	fn compute_2d_coords(&mut self) -> u32 {
		unsafe { ffi::rdkit_depictor_compute_2d_coords(self.as_foreign_mut()) }
	}
}
