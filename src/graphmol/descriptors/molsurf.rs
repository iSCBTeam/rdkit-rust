use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::MolSurfImpl;
}

pub trait MolSurfImpl {
	fn calc_tpsa(&self) -> f64;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> MolSurfImpl for T {
	fn calc_tpsa(&self) -> f64 {
		unsafe { ffi::rdkit_descriptors_calc_tpsa(self.as_foreign()) }
	}
}
