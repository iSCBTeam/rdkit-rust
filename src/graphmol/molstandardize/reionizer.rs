use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::Reionizer;
	pub use super::ReionizerImplRef;
	pub use super::ReionizerImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Reionizer {
	foreign: ffi::rdkit_mol_standardize_Reionizer,
}

impl InitParams<Reionizer> for () {}
impl Initializable<()> for Reionizer {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_reionizer_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for Reionizer {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_standardize_reionizer_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Reionizer {
	type ForeignSelf = ffi::rdkit_mol_standardize_Reionizer;
}
impl FFIBindingSelfNative for ffi::rdkit_mol_standardize_Reionizer {
	type NativeSelf = Reionizer;
}

#[derive(Debug)]
pub enum ReionizerError {
	Error,
}

pub trait ReionizerImplRef<Native> {
	fn reionize<
		MolNative: AsForeign<ffi::rdkit_RWMol>,
		Mol: InitializedMut<MolNative, ffi::rdkit_RWMol>
	>(&self, mol: &mut Mol) -> Result<(), ReionizerError>;
}
pub trait ReionizerImplMut<Native> {}

impl<'s, Wrapper, Native: 's> ReionizerImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_mol_standardize_Reionizer>,
	Native: AsForeign<ffi::rdkit_mol_standardize_Reionizer>,
{
	fn reionize<
		MolNative: AsForeign<ffi::rdkit_RWMol>,
		Mol: InitializedMut<MolNative, ffi::rdkit_RWMol>
	>(&self, mol: &mut Mol) -> Result<(), ReionizerError> {
		unsafe { ffi::rdkit_mol_standardize_reionizer_reionize(self.as_ref().as_foreign(), mol.get_unchecked_mut().as_foreign_mut()) }
			.then_some(())
			.ok_or(ReionizerError::Error)
	}
}

impl<'s, Wrapper, Native: 's> ReionizerImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_mol_standardize_Reionizer>,
	Native: AsForeign<ffi::rdkit_mol_standardize_Reionizer>,
{
}
