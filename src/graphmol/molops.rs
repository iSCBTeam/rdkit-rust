use crate::ffi;
use crate::general::prelude::*;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::RemoveHsParameters;
	pub use super::SanitizeError;
	pub use super::RemoveHsError;
	pub use super::MolOpsRWMolImplRef;
	pub use super::MolOpsRWMolImplMut;
	pub use super::MolOpsROMolImplRef;
	pub use super::MolOpsROMolImplMut;
}

pub use ffi::rdkit_mol_ops_RemoveHsParameters as RemoveHsParameters;

pub enum AssignStereochemistryError {
	Error,
}
pub enum SanitizeError {
	Error,
}
pub enum AddHsError {
	Error,
}
pub enum RemoveHsError {
	Error,
}

pub trait MolOpsROMolImplRef<Native>
where
	Self: InitializedRef<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>, {
}
pub trait MolOpsROMolImplMut<Native>
where
	Self: InitializedMut<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>,
{
	fn assign_stereochemistry(&mut self) -> Result<(), AssignStereochemistryError>;
}

impl<'s, Wrapper, Native: 's> MolOpsROMolImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>,
{
}

impl<'s, Wrapper, Native: 's> MolOpsROMolImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>,
{
	fn assign_stereochemistry(&mut self) -> Result<(), AssignStereochemistryError> {
		unsafe { ffi::rdkit_mol_ops_assign_stereochemistry(self.get_unchecked_mut().as_foreign_mut()) }
			.then_some(())
			.ok_or(AssignStereochemistryError::Error)
	}
}

pub trait MolOpsRWMolImplRef<Native>
where
	Self: InitializedRef<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>, {
}
pub trait MolOpsRWMolImplMut<Native>
where
	Self: InitializedMut<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
	fn sanitize(&mut self) -> Result<(), SanitizeError>;
	fn add_hs(&mut self, explicit_only: Option<bool>, add_coords: Option<bool>, add_residue_info: Option<bool>) -> Result<(), AddHsError>;
	fn remove_hs(&mut self, params: &RemoveHsParameters, sanitize: Option<bool>) -> Result<(), RemoveHsError>;
}

impl<'s, Wrapper, Native: 's> MolOpsRWMolImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
}

impl<'s, Wrapper, Native: 's> MolOpsRWMolImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
	fn sanitize(&mut self) -> Result<(), SanitizeError> {
		unsafe { ffi::rdkit_mol_ops_sanitize_mol(self.get_unchecked_mut().as_foreign_mut()) }
			.then_some(())
			.ok_or(SanitizeError::Error)
	}
	fn add_hs(&mut self, explicit_only: Option<bool>, add_coords: Option<bool>, add_residue_info: Option<bool>) -> Result<(), AddHsError> {
		unsafe { ffi::rdkit_mol_ops_add_hs_ex(self.get_unchecked_mut().as_foreign_mut(), explicit_only.as_foreign(), add_coords.as_foreign(), add_residue_info.as_foreign()) }
			.then_some(())
			.ok_or(AddHsError::Error)
	}
	fn remove_hs(&mut self, params: &RemoveHsParameters, sanitize: Option<bool>) -> Result<(), RemoveHsError> {
		unsafe { ffi::rdkit_mol_ops_remove_hs_ex(self.get_unchecked_mut().as_foreign_mut(), params, sanitize.as_foreign()) }
			.then_some(())
			.ok_or(RemoveHsError::Error)
	}
}

