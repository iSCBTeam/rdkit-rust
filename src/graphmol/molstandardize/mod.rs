use crate::ffi;
use crate::general::wrap::*;
use crate::general::foreign::*;

pub mod allowedatomsvalidation;
pub mod largestfragmentchooser;
pub mod reionizer;
pub mod uncharger;

pub mod prelude {
	pub use super::MolStandardizeImpl;
	pub use super::CleanupParameters;
	pub use super::CleanupParametersInitParams;
	pub use super::allowedatomsvalidation::prelude::*;
	pub use super::largestfragmentchooser::prelude::*;
	pub use super::reionizer::prelude::*;
	pub use super::uncharger::prelude::*;
}

#[derive(Debug)]
pub enum MolStandardizeError {
	Error,
}

pub trait MolStandardizeImpl {
	fn standardize_cleanup(&mut self) -> Result<(), MolStandardizeError>;
}

impl<'s, T: AsForeign<ffi::rdkit_RWMol>> MolStandardizeImpl for T {
	fn standardize_cleanup(&mut self) -> Result<(), MolStandardizeError> {
		unsafe { ffi::rdkit_mol_standardize_cleanup(self.as_foreign_mut()) }
			.then_some(())
			.ok_or(MolStandardizeError::Error)
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct CleanupParameters {
	foreign: ffi::rdkit_mol_standardize_CleanupParameters,
}

impl InitParams<CleanupParameters> for () {}
impl Initializable<()> for CleanupParameters {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_cleanup_parameters_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}

#[derive(Debug, Default)]
pub struct CleanupParametersInitParams {
	pub max_restarts: Option<i32>,
    pub prefer_organic: Option<bool>,
    pub do_canonical: Option<bool>,
    pub max_tautomers: Option<i32>,
    pub max_transforms: Option<i32>,
    pub tautomer_remove_sp3_stereo: Option<bool>,
    pub tautomer_remove_bond_stereo: Option<bool>,
    pub tautomer_remove_isotopic_hs: Option<bool>,
    pub tautomer_reassign_stereo: Option<bool>,
    pub largest_fragment_chooser_use_atom_count: Option<bool>,
    pub largest_fragment_chooser_count_heavy_atoms_only: Option<bool>,
}

impl CleanupParametersInitParams {
	pub fn as_foreign(&self) -> ffi::rdkit_mol_standardize_cleanup_parameters {
		ffi::rdkit_mol_standardize_cleanup_parameters {
			max_restarts: self.max_restarts.as_foreign(),
			prefer_organic: self.prefer_organic.as_foreign(),
			do_canonical: self.do_canonical.as_foreign(),
			max_tautomers: self.max_tautomers.as_foreign(),
			max_transforms: self.max_transforms.as_foreign(),
			tautomer_remove_sp3_stereo: self.tautomer_remove_sp3_stereo.as_foreign(),
			tautomer_remove_bond_stereo: self.tautomer_remove_bond_stereo.as_foreign(),
			tautomer_remove_isotopic_hs: self.tautomer_remove_isotopic_hs.as_foreign(),
			tautomer_reassign_stereo: self.tautomer_reassign_stereo.as_foreign(),
			largest_fragment_chooser_use_atom_count: self.largest_fragment_chooser_use_atom_count.as_foreign(),
			largest_fragment_chooser_count_heavy_atoms_only: self.largest_fragment_chooser_count_heavy_atoms_only.as_foreign(),
		}
	}
}

impl InitParams<CleanupParameters> for &CleanupParametersInitParams {}
impl Initializable<&CleanupParametersInitParams> for CleanupParameters {
	unsafe fn init(value: *mut Self, params: &CleanupParametersInitParams) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_cleanup_parameters_ctor_params(std::mem::transmute(value), &params.as_foreign());
		Ok(())
	}
}

impl Drop for CleanupParameters {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_standardize_cleanup_parameters_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for CleanupParameters {
	type ForeignSelf = ffi::rdkit_mol_standardize_CleanupParameters;
}
impl FFIBindingSelfNative for ffi::rdkit_mol_standardize_CleanupParameters {
	type NativeSelf = CleanupParameters;
}