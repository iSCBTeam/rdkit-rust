use std::marker::PhantomData;

use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::AllowedAtomsValidation;
	pub use super::AllowedAtomsValidationInitParams;
	pub use super::AllowedAtomsValidationImplRef;
	pub use super::AllowedAtomsValidationImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AllowedAtomsValidation {
	foreign: ffi::rdkit_mol_standardize_AllowedAtomsValidation,
}

pub struct AllowedAtomsValidationInitParams<'s, AtomsNative: AsForeign<ffi::rdkit_Atom_sptr_vec>, Atoms: InitializedRef<AtomsNative, ffi::rdkit_Atom_sptr_vec>> {
	pub atoms: &'s Atoms,
    pub _phantom_atoms_native: PhantomData<AtomsNative>,
}

impl<AtomsNative: AsForeign<ffi::rdkit_Atom_sptr_vec>, Atoms: InitializedRef<AtomsNative, ffi::rdkit_Atom_sptr_vec>> InitParams<AllowedAtomsValidation> for &AllowedAtomsValidationInitParams<'_, AtomsNative, Atoms> {}
impl<AtomsNative: AsForeign<ffi::rdkit_Atom_sptr_vec>, Atoms: InitializedRef<AtomsNative, ffi::rdkit_Atom_sptr_vec>> Initializable<&AllowedAtomsValidationInitParams<'_, AtomsNative, Atoms>> for AllowedAtomsValidation {
	unsafe fn init(value: *mut Self, params: &AllowedAtomsValidationInitParams<AtomsNative, Atoms>) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_allowed_atoms_validation_ctor(std::mem::transmute(value), params.atoms.as_ref().as_foreign());
		Ok(())
	}
}

impl Drop for AllowedAtomsValidation {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_standardize_allowed_atoms_validation_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for AllowedAtomsValidation {
	type ForeignSelf = ffi::rdkit_mol_standardize_AllowedAtomsValidation;
}
impl FFIBindingSelfNative for ffi::rdkit_mol_standardize_AllowedAtomsValidation {
	type NativeSelf = AllowedAtomsValidation;
}

#[derive(Debug)]
pub enum AllowedAtomsValidationError {
	Error,
}

pub trait AllowedAtomsValidationImplRef<Native> {
	fn validate<
		MolNative: AsForeign<ffi::rdkit_ROMol>,
		Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>,
	>(&self, mol: &Mol) -> Result<(), AllowedAtomsValidationError>;
}
pub trait AllowedAtomsValidationImplMut<Native> {
}

impl<'s, Wrapper, Native: 's> AllowedAtomsValidationImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_mol_standardize_AllowedAtomsValidation>,
	Native: AsForeign<ffi::rdkit_mol_standardize_AllowedAtomsValidation>,
{
	fn validate<
		MolNative: AsForeign<ffi::rdkit_ROMol>,
		Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>,
	>(&self, mol: &Mol) -> Result<(), AllowedAtomsValidationError> {
		unsafe { ffi::rdkit_mol_standardize_allowed_atoms_validation_validate(self.as_ref().as_foreign(), mol.as_ref().as_foreign()) }
			.then_some(())
			.ok_or(AllowedAtomsValidationError::Error)
	}
}

impl<'s, Wrapper, Native: 's> AllowedAtomsValidationImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_mol_standardize_AllowedAtomsValidation>,
	Native: AsForeign<ffi::rdkit_mol_standardize_AllowedAtomsValidation>,
{
}
