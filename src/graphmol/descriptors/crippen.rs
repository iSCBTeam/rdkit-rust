use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::CrippenImpl;
}

pub trait CrippenImpl {
	fn calc_clogp(&self) -> f64;
	fn calc_mr(&self) -> f64;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> CrippenImpl for T {
	fn calc_clogp(&self) -> f64 {
		unsafe { ffi::rdkit_descriptors_calc_clogp(self.as_foreign()) }
	}
	fn calc_mr(&self) -> f64 {
		unsafe { ffi::rdkit_descriptors_calc_mr(self.as_foreign()) }
	}
}
