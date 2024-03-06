use crate::ffi;
use crate::general::wrap::*;
use crate::graphmol::romol::*;

pub mod prelude {
	pub use super::LargestFragmentChooser;
	pub use super::LargestFragmentChooserInitParams;
	pub use super::LargestFragmentChooserImplRef;
	pub use super::LargestFragmentChooserImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct LargestFragmentChooser {
	foreign: ffi::rdkit_mol_standardize_LargestFragmentChooser,
}

impl InitParams<LargestFragmentChooser> for () {}
impl Initializable<()> for LargestFragmentChooser {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_largest_fragment_chooser_ctor(std::mem::transmute(value));
		Ok(())
	}
}

pub struct LargestFragmentChooserInitParams<'s, Native: AsForeign<ffi::rdkit_mol_standardize_CleanupParameters>> {
	pub cleanup_params: &'s Native,
}

impl<Native: AsForeign<ffi::rdkit_mol_standardize_CleanupParameters>> InitParams<LargestFragmentChooser> for &LargestFragmentChooserInitParams<'_, Native> {}
impl<Native: AsForeign<ffi::rdkit_mol_standardize_CleanupParameters>> Initializable<&LargestFragmentChooserInitParams<'_, Native>> for LargestFragmentChooser {
	unsafe fn init(value: *mut Self, params: &LargestFragmentChooserInitParams<Native>) -> Result<(), ()> {
		ffi::rdkit_mol_standardize_largest_fragment_chooser_ctor_params(std::mem::transmute(value), params.cleanup_params.as_foreign());
		Ok(())
	}
}

impl Drop for LargestFragmentChooser {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_standardize_largest_fragment_chooser_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for LargestFragmentChooser {
	type ForeignSelf = ffi::rdkit_mol_standardize_LargestFragmentChooser;
}
impl FFIBindingSelfNative for ffi::rdkit_mol_standardize_LargestFragmentChooser {
	type NativeSelf = LargestFragmentChooser;
}

#[derive(Debug)]
pub enum LargestFragmentChooserError {
	Error,
}

pub trait LargestFragmentChooserImplRef<Native> {
	fn choose<MolNative: AsForeign<ffi::rdkit_ROMol>, Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>>(&self, mol: &Mol) -> Result<InitializedHeap<'_, ROMol>, LargestFragmentChooserError>;
	fn choose_in_place<Mol: InitializedMut<ROMol, ffi::rdkit_ROMol>>(&self, mol: &mut Mol) -> Result<(), LargestFragmentChooserError>;
}
pub trait LargestFragmentChooserImplMut<Native> {
}

impl<'s, Wrapper, Native: 's> LargestFragmentChooserImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_mol_standardize_LargestFragmentChooser>,
	Native: AsForeign<ffi::rdkit_mol_standardize_LargestFragmentChooser>,
{
	fn choose<
		MolNative: AsForeign<ffi::rdkit_ROMol>,
		Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>,
	>(&self, mol: &Mol) -> Result<InitializedHeap<'_, ROMol>, LargestFragmentChooserError> {
		let mut frag = ROMol::new_uninit();
		unsafe { ffi::rdkit_mol_standardize_largest_fragment_chooser_choose(self.as_ref().as_foreign(), mol.as_ref().as_foreign(), frag.as_mut().as_foreign_mut()) }
			.then(|| unsafe { frag.assume_init() })
			.ok_or(LargestFragmentChooserError::Error)
	}
	fn choose_in_place<Mol: InitializedMut<ROMol, ffi::rdkit_ROMol>,
	>(&self, mol: &mut Mol) -> Result<(), LargestFragmentChooserError> {
		unsafe { ffi::rdkit_mol_standardize_largest_fragment_chooser_choose_in_place(self.as_ref().as_foreign(), mol.get_unchecked_mut().as_foreign_mut()) }
			.then_some(())
			.ok_or(LargestFragmentChooserError::Error)
	}
}

impl<'s, Wrapper, Native: 's> LargestFragmentChooserImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_mol_standardize_LargestFragmentChooser>,
	Native: AsForeign<ffi::rdkit_mol_standardize_LargestFragmentChooser>,
{
}
