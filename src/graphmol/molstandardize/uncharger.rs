use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::Uncharger;
	pub use super::UnchargerImplRef;
	pub use super::UnchargerImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Uncharger {
	foreign: ffi::rdkit_mol_standardize_Uncharger,
}

impl InitParams<Uncharger> for () {}
impl Initializable<()> for Uncharger {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_uncharger_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for Uncharger {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_standardize_uncharger_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Uncharger {
	type ForeignSelf = ffi::rdkit_mol_standardize_Uncharger;
}
impl FFIBindingSelfNative for ffi::rdkit_mol_standardize_Uncharger {
	type NativeSelf = Uncharger;
}

#[derive(Debug)]
pub enum UnchargerError {
	Error,
}

pub trait UnchargerImplRef<Native> {
	fn uncharge<
		MolNative: AsForeign<ffi::rdkit_RWMol>,
		Mol: InitializedMut<MolNative, ffi::rdkit_RWMol>
	>(&self, mol: &mut Mol) -> Result<(), UnchargerError>;
}
pub trait UnchargerImplMut<Native> {}

impl<'s, Wrapper, Native: 's> UnchargerImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_mol_standardize_Uncharger>,
	Native: AsForeign<ffi::rdkit_mol_standardize_Uncharger>,
{
	fn uncharge<
		MolNative: AsForeign<ffi::rdkit_RWMol>,
		Mol: InitializedMut<MolNative, ffi::rdkit_RWMol>
	>(&self, mol: &mut Mol) -> Result<(), UnchargerError> {
		unsafe { ffi::rdkit_mol_standardize_uncharger_uncharge(self.as_ref().as_foreign(), mol.get_unchecked_mut().as_foreign_mut()) }
			.then_some(())
			.ok_or(UnchargerError::Error)
	}
}

impl<'s, Wrapper, Native: 's> UnchargerImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_mol_standardize_Uncharger>,
	Native: AsForeign<ffi::rdkit_mol_standardize_Uncharger>,
{
}
