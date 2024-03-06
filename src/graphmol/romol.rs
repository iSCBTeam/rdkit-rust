use std::mem::ManuallyDrop;

use crate::ffi;
use crate::general::cxxvec::*;
use crate::general::wrap::*;
use crate::graphmol::atom::Atom;
use crate::graphmol::bond::Bond;
use crate::graphmol::conformer::Conformer;
use crate::graphmol::rwmol::RWMol;

#[derive(Debug)]
#[repr(transparent)]
pub struct ROMol {
	pub(crate) foreign: ffi::rdkit_ROMol,
}

impl InitParams<ROMol> for () {}
impl Initializable<()> for ROMol {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_romol_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}
impl Uninitializable for ROMol {
	unsafe fn uninit(value: *mut Self) {
		ffi::rdkit_romol_dtor(std::ptr::addr_of_mut!((*value).foreign));
	}
}

pub struct ROMolInitParamsROMol<'s, Native: AsForeign<ffi::rdkit_ROMol>> {
	pub romol: &'s Native,
}

impl<Native: AsForeign<ffi::rdkit_ROMol>> InitParams<ROMol> for ROMolInitParamsROMol<'_, Native> {}
impl<Native: AsForeign<ffi::rdkit_ROMol>> Initializable<ROMolInitParamsROMol<'_, Native>> for ROMol {
	unsafe fn init(value: *mut Self, params: ROMolInitParamsROMol<Native>) -> Result<(), ()> {
		ffi::rdkit_romol_ctor_clone(std::ptr::addr_of_mut!((*value).foreign), params.romol.as_foreign());
		Ok(())
	}
}

impl<'s, T: InitializedOwned<'s, RWMol, ffi::rdkit_RWMol>> InitParams<ROMol> for T {}
impl<'s, T: InitializedOwned<'s, RWMol, ffi::rdkit_RWMol>> Initializable<T> for ROMol {
	unsafe fn init(value: *mut Self, params: T) -> Result<(), ()> {
		let mut container = params.into_inner().0;
		let rwmol = match container {
			InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
			InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
		};

		ffi::rdkit_romol_ctor_from_rwmol_move(std::ptr::addr_of_mut!((*value).foreign), rwmol);

		match container {
			InitializedInnerImpl::Local(inner) => {
				ManuallyDrop::drop(inner);
			},
			InitializedInnerImpl::Heap(mut inner) => {
				ManuallyDrop::drop(&mut *inner);
			},
		};

		Ok(())
	}
}

impl Drop for ROMol {
	fn drop(&mut self) {
		//println!("Dropping ROMol");
		unsafe { ffi::rdkit_romol_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for ROMol {
	type ForeignSelf = ffi::rdkit_ROMol;
}
impl FFIBindingSelfNative for ffi::rdkit_ROMol {
	type NativeSelf = ROMol;
}

impl AsForeign<ffi::rdkit_RDProps> for ROMol {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let ptr: *const ffi::rdkit_ROMol = self.as_foreign();
		unsafe { &*ffi::rdkit_romol_to_rdprops(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		unsafe { &mut *ffi::rdkit_romol_to_rdprops(self.as_foreign_mut()) }
	}
}

pub trait ROMolImplRef<Native> {
	fn get_num_atoms(&self)-> u32;
	fn get_num_explicit_atoms(&self) -> u32;
	fn get_num_heavy_atoms(&self) -> u32;
	fn get_atom(&self, idx: u32) -> Option<InitializedForeignRef<&Atom>>;
	fn get_num_bonds(&self)-> u32;
	fn get_bond(&self, idx: u32) -> Option<InitializedForeignRef<&Bond>>;
	fn get_num_conformers(&self)-> u32;
	fn get_conformer(&self, idx: i32) -> Option<InitializedForeignRef<&Conformer>>;
}
pub trait ROMolImplMut<Native> {
	fn get_atom_mut(&mut self, idx: u32) -> Option<InitializedForeignRef<&mut Atom>>;
	fn get_bond_mut(&mut self, idx: u32) -> Option<InitializedForeignRef<&mut Bond>>;
	fn get_conformer_mut(&mut self, idx: i32) -> Option<InitializedForeignRef<&mut Conformer>>;
}

impl<'s, Wrapper, Native: 's> ROMolImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>,
{
	fn get_num_atoms(&self)-> u32 {
		unsafe { ffi::rdkit_romol_get_num_atoms(self.as_ref().as_foreign()) }
	}
	fn get_num_explicit_atoms(&self) -> u32 {
		unsafe { ffi::rdkit_romol_get_num_explicit_atoms(self.as_ref().as_foreign()) }
	}
	fn get_num_heavy_atoms(&self) -> u32 {
		unsafe { ffi::rdkit_romol_get_num_heavy_atoms(self.as_ref().as_foreign()) }
	}
	fn get_atom(&self, idx: u32) -> Option<InitializedForeignRef<&Atom>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_atom(self.as_ref().as_foreign(), idx);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
	fn get_num_bonds(&self)-> u32 {
		unsafe { ffi::rdkit_romol_get_num_bonds(self.as_ref().as_foreign()) }
	}
	fn get_bond(&self, idx: u32) -> Option<InitializedForeignRef<&Bond>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_bond_with_idx(self.as_ref().as_foreign(), idx);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
	fn get_num_conformers(&self)-> u32 {
		unsafe { ffi::rdkit_romol_get_num_conformers(self.as_ref().as_foreign()) }
	}
	fn get_conformer(&self, idx: i32) -> Option<InitializedForeignRef<&Conformer>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_conformer(self.as_ref().as_foreign(), idx);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
}

impl<'s, Wrapper, Native: 's> ROMolImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_ROMol>,
	Native: AsForeign<ffi::rdkit_ROMol>,
{
	fn get_atom_mut(&mut self, idx: u32) -> Option<InitializedForeignRef<&mut Atom>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_atom_mut(self.get_unchecked_mut().as_foreign_mut(), idx);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
	fn get_bond_mut(&mut self, idx: u32) -> Option<InitializedForeignRef<&mut Bond>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_bond_with_idx_mut(self.get_unchecked_mut().as_foreign_mut(), idx);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
	fn get_conformer_mut(&mut self, idx: i32) -> Option<InitializedForeignRef<&mut Conformer>> {
		unsafe {
			let ptr = ffi::rdkit_romol_get_conformer_mut(self.get_unchecked_mut().as_foreign_mut(), idx);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ROMolSptrVec {
	foreign: ffi::rdkit_ROMol_sptr_vec,
}

impl InitParams<ROMolSptrVec> for () {}
impl Initializable<()> for ROMolSptrVec {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_romol_sptr_vec_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}

impl Drop for ROMolSptrVec {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_romol_sptr_vec_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for ROMolSptrVec {
	type ForeignSelf = ffi::rdkit_ROMol_sptr_vec;
}
impl FFIBindingSelfNative for ffi::rdkit_ROMol_sptr_vec {
	type NativeSelf = ROMolSptrVec;
}

impl<'s, Wrapper, Native: 's> CxxVecImplRef<'s, Native, ffi::rdkit_ROMol_sptr_vec, ROMol, ffi::rdkit_ROMol> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_ROMol_sptr_vec>,
	Native: AsForeign<ffi::rdkit_ROMol_sptr_vec>,
{
	fn get(&self, i: usize) -> Option<InitializedForeignRef<&ROMol>> {
		unsafe {
			let ptr = ffi::rdkit_romol_sptr_vec_get(self.as_ref().as_foreign(), i);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
	fn size(&self) -> usize {
		unsafe { ffi::rdkit_romol_sptr_vec_size(self.as_ref().as_foreign()) }
	}
}
impl<'s, Wrapper, Native: 's> CxxVecImplMut<'s, Native, ffi::rdkit_ROMol_sptr_vec, ROMol, ffi::rdkit_ROMol> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_ROMol_sptr_vec>,
	Native: AsForeign<ffi::rdkit_ROMol_sptr_vec>,
{
	fn get_mut(&mut self, i: usize) -> Option<InitializedForeignRef<&mut ROMol>> {
		unsafe {
			let ptr = ffi::rdkit_romol_sptr_vec_get_mut(self.get_unchecked_mut().as_foreign_mut(), i);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
	fn append<T: 's + InitializedOwned<'s, ROMol, ffi::rdkit_ROMol>>(&mut self, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let elem = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_romol_sptr_vec_append_move(self.get_unchecked_mut().as_foreign_mut(), elem);

			match container {
				InitializedInnerImpl::Local(inner) => {
					ManuallyDrop::drop(inner);
				},
				InitializedInnerImpl::Heap(mut inner) => {
					ManuallyDrop::drop(&mut *inner);
				},
			};
		}
	}
	fn append_mut<T: 's + InitializedMut<ROMol, ffi::rdkit_ROMol>>(&mut self, elem: &mut T) {
		unsafe { ffi::rdkit_romol_sptr_vec_append(self.get_unchecked_mut().as_foreign_mut(), elem.get_unchecked_mut().as_foreign_mut()); }
	}
	fn set<T: 's + InitializedOwned<'s, ROMol, ffi::rdkit_ROMol>>(&mut self, i: usize, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let elem = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_romol_sptr_vec_set_move(self.get_unchecked_mut().as_foreign_mut(), i, elem);

			match container {
				InitializedInnerImpl::Local(inner) => {
					ManuallyDrop::drop(inner);
				},
				InitializedInnerImpl::Heap(mut inner) => {
					ManuallyDrop::drop(&mut *inner);
				},
			};
		}
	}
	fn set_mut<T: 's + InitializedMut<ROMol, ffi::rdkit_ROMol>>(&mut self, i: usize, elem: &mut T) {
		unsafe { ffi::rdkit_romol_sptr_vec_set(self.get_unchecked_mut().as_foreign_mut(), i, elem.get_unchecked_mut().as_foreign_mut()); }
	}
	fn clear(&mut self) {
		unsafe { ffi::rdkit_romol_sptr_vec_clear(self.get_unchecked_mut().as_foreign_mut()); }
	}
	fn delete(&mut self, i: usize) {
		unsafe { ffi::rdkit_romol_sptr_vec_delete(self.get_unchecked_mut().as_foreign_mut(), i); }
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ROMolSptrVecVec {
	foreign: ffi::rdkit_ROMol_sptr_vec_vec,
}

impl InitParams<ROMolSptrVecVec> for () {}
impl Initializable<()> for ROMolSptrVecVec {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_romol_sptr_vec_vec_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}

impl Drop for ROMolSptrVecVec {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_romol_sptr_vec_vec_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for ROMolSptrVecVec {
	type ForeignSelf = ffi::rdkit_ROMol_sptr_vec_vec;
}
impl FFIBindingSelfNative for ffi::rdkit_ROMol_sptr_vec_vec {
	type NativeSelf = ROMolSptrVecVec;
}

impl<'s, Wrapper, Native: 's> CxxVecImplRef<'s, Native, ffi::rdkit_ROMol_sptr_vec_vec, ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_ROMol_sptr_vec_vec>,
	Native: AsForeign<ffi::rdkit_ROMol_sptr_vec_vec>,
{
	fn get(&self, i: usize) -> Option<InitializedForeignRef<&ROMolSptrVec>> {
		unsafe {
			let ptr = ffi::rdkit_romol_sptr_vec_vec_get(self.get_ref().as_foreign(), i);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
	fn size(&self) -> usize {
		unsafe { ffi::rdkit_romol_sptr_vec_vec_size(self.as_ref().as_foreign()) }
	}
}
impl<'s, Wrapper, Native: 's> CxxVecImplMut<'s, Native, ffi::rdkit_ROMol_sptr_vec_vec, ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_ROMol_sptr_vec_vec>,
	Native: AsForeign<ffi::rdkit_ROMol_sptr_vec_vec>,
{
	fn get_mut(&mut self, i: usize) -> Option<InitializedForeignRef<&mut ROMolSptrVec>> {
		unsafe {
			let ptr = ffi::rdkit_romol_sptr_vec_vec_get_mut(self.get_unchecked_mut().as_foreign_mut(), i);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
	fn append<T: 's + InitializedOwned<'s, ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec>>(&mut self, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let svec = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_romol_sptr_vec_vec_append_move(self.get_unchecked_mut().as_foreign_mut(), svec);

			match container {
				InitializedInnerImpl::Local(inner) => {
					ManuallyDrop::drop(inner);
				},
				InitializedInnerImpl::Heap(mut inner) => {
					ManuallyDrop::drop(&mut *inner);
				},
			};
		}
	}
	fn append_mut<T: 's + InitializedMut<ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec>>(&mut self, elem: &mut T) {
		unsafe {
			ffi::rdkit_romol_sptr_vec_vec_append(self.get_unchecked_mut().as_foreign_mut(), elem.get_unchecked_mut().as_foreign_mut());
		}
	}
	fn set<T: 's + InitializedOwned<'s, ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec>>(&mut self, i: usize, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let svec = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_romol_sptr_vec_vec_set_move(self.get_unchecked_mut().as_foreign_mut(), i, svec);

			match container {
				InitializedInnerImpl::Local(inner) => {
					ManuallyDrop::drop(inner);
				},
				InitializedInnerImpl::Heap(mut inner) => {
					ManuallyDrop::drop(&mut *inner);
				},
			};
		}
	}
	fn set_mut<T: 's + InitializedMut<ROMolSptrVec, ffi::rdkit_ROMol_sptr_vec>>(&mut self, i: usize, elem: &mut T) {
		unsafe { ffi::rdkit_romol_sptr_vec_vec_set(self.get_unchecked_mut().as_foreign_mut(), i, elem.get_unchecked_mut().as_foreign_mut()); }
	}
	fn clear(&mut self) {
		unsafe { ffi::rdkit_romol_sptr_vec_vec_clear(self.get_unchecked_mut().as_foreign_mut()); }
	}
	fn delete(&mut self, i: usize) {
		unsafe { ffi::rdkit_romol_sptr_vec_vec_delete(self.get_unchecked_mut().as_foreign_mut(), i); }
	}
}
