use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;

use crate::ffi;
use crate::general::cxxvec::*;
use crate::general::wrap::*;
use crate::general::string::prelude::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct Atom {
	foreign: ffi::rdkit_Atom,
}

impl InitParams<Atom> for () {}
impl Initializable<()> for Atom {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_atom_ctor(std::mem::transmute(value));
		Ok(())
	}
}

pub struct AtomInitParamsAtomicNum {
	pub atomic_num: u32,
}
impl InitParams<Atom> for &AtomInitParamsAtomicNum {}
impl Initializable<&AtomInitParamsAtomicNum> for Atom {
	unsafe fn init(value: *mut Self, params: &AtomInitParamsAtomicNum) -> Result<(), ()> {
		ffi::rdkit_atom_ctor_atomic_num(std::mem::transmute(value), params.atomic_num);
		Ok(())
	}
}

pub struct AtomInitParamsSymbol<'s> {
	pub symbol: &'s str,
}
impl InitParams<Atom> for &AtomInitParamsSymbol<'_> {}
impl Initializable<&AtomInitParamsSymbol<'_>> for Atom {
	unsafe fn init(value: *mut Self, params: &AtomInitParamsSymbol) -> Result<(), ()> {
		ffi::rdkit_atom_ctor_symbol(std::mem::transmute(value), RDstr::from(params.symbol).as_foreign());
		Ok(())
	}
}

impl Drop for Atom {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_atom_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Atom {
	type ForeignSelf = ffi::rdkit_Atom;
}
impl FFIBindingSelfNative for ffi::rdkit_Atom {
	type NativeSelf = Atom;
}

impl AsForeign<ffi::rdkit_RDProps> for Atom {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let ptr: *const ffi::rdkit_Atom = self.as_foreign();
		unsafe { &*ffi::rdkit_atom_to_rdprops(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		unsafe { &mut *ffi::rdkit_atom_to_rdprops(self.as_foreign_mut()) }
	}
}

pub trait AtomImplRef<Native> {
	fn get_atomic_num(&self) -> i32;
	fn get_isotope(&self) -> u32;
	fn get_symbol(&self) -> Option<String>;
	fn match_atom<
		Other: InitializedRef<OtherNative, ffi::rdkit_Atom>,
		OtherNative: AsForeign<ffi::rdkit_Atom>,
	>(&self, other: &Other) -> bool;
}

impl<'s, Wrapper, Native: 's> AtomImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_Atom>,
	Native: AsForeign<ffi::rdkit_Atom>,
{
	fn get_atomic_num(&self) -> i32 {
		unsafe {
			ffi::rdkit_atom_get_atomic_num(self.as_ref().as_foreign())
		}
	}
	fn get_isotope(&self) -> u32 {
		unsafe {
			ffi::rdkit_atom_get_isotope(self.as_ref().as_foreign())
		}
	}
	fn get_symbol(&self) -> Option<String> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_atom_get_symbol(
					self.as_ref().as_foreign(),
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()))
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
		}
	}
	fn match_atom<
			Other: InitializedRef<OtherNative, ffi::rdkit_Atom>,
			OtherNative: AsForeign<ffi::rdkit_Atom>,
		>(&self, other: &Other) -> bool {
		unsafe {
			ffi::rdkit_atom_match(
				self.as_ref().as_foreign(),
				other.as_ref().as_foreign())
		}
	}
}

pub trait AtomImplMut<Native> {
	fn set_atomic_num(&mut self, atomic_num: i32);
	fn set_isotope(&mut self, mass_number: u32);
}

impl<'s, Wrapper, Native: 's> AtomImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_Atom>,
	Native: AsForeign<ffi::rdkit_Atom>,
{
	fn set_atomic_num(&mut self, atomic_num: i32) {
		unsafe {
			ffi::rdkit_atom_set_atomic_num(
				self.get_unchecked_mut().as_foreign_mut(),
				atomic_num)
		}
	}
	fn set_isotope(&mut self, mass_number: u32) {
		unsafe {
			ffi::rdkit_atom_set_isotope(
				self.get_unchecked_mut().as_foreign_mut(),
				mass_number)
		}
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct AtomSptrVec {
	foreign: ffi::rdkit_Atom_sptr_vec,
}

impl InitParams<AtomSptrVec> for () {}
impl Initializable<()> for AtomSptrVec {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_atom_sptr_vec_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}

impl Drop for AtomSptrVec {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_atom_sptr_vec_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for AtomSptrVec {
	type ForeignSelf = ffi::rdkit_Atom_sptr_vec;
}
impl FFIBindingSelfNative for ffi::rdkit_Atom_sptr_vec {
	type NativeSelf = AtomSptrVec;
}

impl<'s, Wrapper, Native: 's> CxxVecImplRef<'s, Native, ffi::rdkit_Atom_sptr_vec, Atom, ffi::rdkit_Atom> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_Atom_sptr_vec>,
	Native: AsForeign<ffi::rdkit_Atom_sptr_vec>,
{
	fn get(&self, i: usize) -> Option<InitializedForeignRef<&Atom>> {
		unsafe {
			let ptr = ffi::rdkit_atom_sptr_vec_get(self.as_ref().as_foreign(), i);
			ptr.as_ref().and_then(|r| Some(InitializedForeignRef::from_raw(r.as_native())))
		}
	}
	fn size(&self) -> usize {
		unsafe { ffi::rdkit_atom_sptr_vec_size(self.as_ref().as_foreign()) }
	}
}
impl<'s, Wrapper, Native: 's> CxxVecImplMut<'s, Native, ffi::rdkit_Atom_sptr_vec, Atom, ffi::rdkit_Atom> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_Atom_sptr_vec>,
	Native: AsForeign<ffi::rdkit_Atom_sptr_vec>,
{
	fn get_mut(&mut self, i: usize) -> Option<InitializedForeignRef<&mut Atom>> {
		unsafe {
			let ptr = ffi::rdkit_atom_sptr_vec_get_mut(self.get_unchecked_mut().as_foreign_mut(), i);
			ptr.as_mut().and_then(|r| Some(InitializedForeignRef::from_raw_mut(r.as_native_mut())))
		}
	}
	fn append<T: 's + InitializedOwned<'s, Atom, ffi::rdkit_Atom>>(&mut self, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let elem = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_atom_sptr_vec_append_move(self.get_unchecked_mut().as_foreign_mut(), elem);

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
	fn append_mut<T: 's + InitializedMut<Atom, ffi::rdkit_Atom>>(&mut self, elem: &mut T) {
		unsafe { ffi::rdkit_atom_sptr_vec_append(self.get_unchecked_mut().as_foreign_mut(), elem.get_unchecked_mut().as_foreign_mut()); }
	}
	fn set<T: 's + InitializedOwned<'s, Atom, ffi::rdkit_Atom>>(&mut self, i: usize, elem: T) {
		unsafe {
			let mut container = elem.into_inner().0;
			let elem = match container {
				InitializedInnerImpl::Local(ref mut inner) => inner.as_foreign_mut(),
				InitializedInnerImpl::Heap(ref mut inner) => inner.as_foreign_mut(),
			};

			ffi::rdkit_atom_sptr_vec_set_move(self.get_unchecked_mut().as_foreign_mut(), i, elem);

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
	fn set_mut<T: 's + InitializedMut<Atom, ffi::rdkit_Atom>>(&mut self, i: usize, elem: &mut T) {
		unsafe { ffi::rdkit_atom_sptr_vec_set(self.get_unchecked_mut().as_foreign_mut(), i, elem.get_unchecked_mut().as_foreign_mut()); }
	}
	fn clear(&mut self) {
		unsafe { ffi::rdkit_atom_sptr_vec_clear(self.get_unchecked_mut().as_foreign_mut()); }
	}
	fn delete(&mut self, i: usize) {
		unsafe { ffi::rdkit_atom_sptr_vec_delete(self.get_unchecked_mut().as_foreign_mut(), i); }
	}
}
