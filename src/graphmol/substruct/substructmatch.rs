use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::SubstructMatchROMolImpl;
}

pub trait SubstructMatchROMolImpl {
	fn has_substruct_match<
		QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
		Query: AsForeign<ffi::rdkit_ROMol>>(&self, query: &QueryWrapper) -> bool;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> SubstructMatchROMolImpl for T {
    fn has_substruct_match<
		QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
		Query: AsForeign<ffi::rdkit_ROMol>>(&self, query: &QueryWrapper) -> bool {
		unsafe { ffi::rdkit_has_substruct_match(self.as_foreign(), query.as_ref().as_foreign()) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MatchVectTypeVec {
	pub(crate) foreign: ffi::rdkit_MatchVectType_vec,
}

pub struct MatchVectTypeVecInitParamsFromSubstructMatch<
	's,
	MolWrapper: InitializedRef<Mol, ffi::rdkit_ROMol>,
	Mol: AsForeign<ffi::rdkit_ROMol>,
	QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
	Query: AsForeign<ffi::rdkit_ROMol>,
> {
	pub mol: &'s MolWrapper,
	pub query: &'s QueryWrapper,
	pub _mol_phantom: PhantomData<Mol>,
	pub _query_phantom: PhantomData<Query>,
}

impl<
	's,
	MolWrapper: InitializedRef<Mol, ffi::rdkit_ROMol>,
	Mol: AsForeign<ffi::rdkit_ROMol>,
	QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
	Query: AsForeign<ffi::rdkit_ROMol>,
> MatchVectTypeVecInitParamsFromSubstructMatch<'s, MolWrapper, Mol, QueryWrapper, Query> {
	pub fn new(mol: &'s MolWrapper, query: &'s QueryWrapper) -> Self {
		Self {
			mol,
			query,
			_mol_phantom: PhantomData,
			_query_phantom: PhantomData,
		}
	}
}

impl<
	MolWrapper: InitializedRef<Mol, ffi::rdkit_ROMol>,
	Mol: AsForeign<ffi::rdkit_ROMol>,
	QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
	Query: AsForeign<ffi::rdkit_ROMol>,
> InitParams<MatchVectTypeVec> for &MatchVectTypeVecInitParamsFromSubstructMatch<'_, MolWrapper, Mol, QueryWrapper, Query> {}
impl<
	MolWrapper: InitializedRef<Mol, ffi::rdkit_ROMol>,
	Mol: AsForeign<ffi::rdkit_ROMol>,
	QueryWrapper: InitializedRef<Query, ffi::rdkit_ROMol>,
	Query: AsForeign<ffi::rdkit_ROMol>,
> Initializable<&MatchVectTypeVecInitParamsFromSubstructMatch<'_, MolWrapper, Mol, QueryWrapper, Query>> for MatchVectTypeVec {
	unsafe fn init(value: *mut Self, params: &MatchVectTypeVecInitParamsFromSubstructMatch<MolWrapper, Mol, QueryWrapper, Query>) -> Result<(), ()> {
		ffi::rdkit_substruct_match(std::ptr::addr_of_mut!((*value).foreign),params.mol.as_ref().as_foreign(), params.query.as_ref().as_foreign())
			.then_some(())
			.ok_or(())
	}
}

impl Drop for MatchVectTypeVec {
	fn drop(&mut self) {
		//println!("Dropping MatchVectTypeVec");
		unsafe { ffi::rdkit_match_vect_type_vec_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MatchVectTypeVec {
	type ForeignSelf = ffi::rdkit_MatchVectType_vec;
}
impl FFIBindingSelfNative for ffi::rdkit_MatchVectType_vec {
	type NativeSelf = MatchVectTypeVec;
}

pub trait MatchVectTypeVecImplRef<Native> {
	fn len(&self)-> usize;
	fn entry_len(&self, idx: usize)-> usize;
	fn entry_get_atom_pair(&self, idx_entry: usize, idx_pair: usize)-> Option<(i32, i32)>;
}
pub trait MatchVectTypeVecImplMut<Native> {
}

impl<'s, Wrapper, Native: 's> MatchVectTypeVecImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_MatchVectType_vec>,
	Native: AsForeign<ffi::rdkit_MatchVectType_vec>,
{
	fn len(&self)-> usize {
		unsafe { ffi::rdkit_match_vect_type_vec_size(self.as_ref().as_foreign()) }
	}
	fn entry_len(&self, idx: usize)-> usize {
		unsafe { ffi::rdkit_match_vect_type_vec_entry_size(self.as_ref().as_foreign(), idx) }
	}
	fn entry_get_atom_pair(&self, idx_entry: usize, idx_pair: usize)-> Option<(i32, i32)> {
		let mut idx_query_atom = MaybeUninit::uninit();
		let mut idx_mol_atom = MaybeUninit::uninit();

		unsafe {
			ffi::rdkit_match_vect_type_vec_get_entry_atom_pair(self.as_ref().as_foreign(), idx_entry, idx_pair, idx_query_atom.as_mut_ptr(), idx_mol_atom.as_mut_ptr())
				.then(|| (idx_query_atom.assume_init(), idx_mol_atom.assume_init()))
		}
	}
}

impl<'s, Wrapper, Native: 's> MatchVectTypeVecImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_MatchVectType_vec>,
	Native: AsForeign<ffi::rdkit_MatchVectType_vec>,
{
}
