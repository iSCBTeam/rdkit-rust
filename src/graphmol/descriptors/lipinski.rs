use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::LipinskiImpl;
}

pub trait LipinskiImpl {
	fn calc_fraction_csp3(&self) -> f64;
	fn calc_num_hba(&self) -> u32;
	fn calc_num_hbd(&self) -> u32;
	fn calc_num_rings(&self) -> u32;
	fn calc_num_rotatable_bonds(&self) -> u32;
	fn calc_num_atom_stereo_centers(&self) -> u32;
	fn calc_num_unspecified_atom_stereo_centers(&self) -> u32;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> LipinskiImpl for T {
	fn calc_fraction_csp3(&self) -> f64 {
		unsafe { ffi::rdkit_descriptors_calc_fraction_csp3(self.as_foreign()) }
	}
	fn calc_num_hba(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_hba(self.as_foreign()) }
	}
	fn calc_num_hbd(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_hbd(self.as_foreign()) }
	}
	fn calc_num_rings(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_rings(self.as_foreign()) }
	}
	fn calc_num_rotatable_bonds(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_rotatable_bonds(self.as_foreign()) }
	}
	fn calc_num_atom_stereo_centers(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_atom_stereo_centers(self.as_foreign()) }
	}
	fn calc_num_unspecified_atom_stereo_centers(&self) -> u32 {
		unsafe { ffi::rdkit_descriptors_calc_num_unspecified_atom_stereo_centers(self.as_foreign()) }
	}
}
