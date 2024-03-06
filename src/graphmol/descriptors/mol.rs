use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::MolDescriptorsImpl;
}

pub trait MolDescriptorsImpl {
	fn calc_exact_mw(&self) -> f64;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> MolDescriptorsImpl for T {
	fn calc_exact_mw(&self) -> f64 {
		unsafe { ffi::rdkit_descriptors_calc_exact_mw(self.as_foreign()) }
	}
}
