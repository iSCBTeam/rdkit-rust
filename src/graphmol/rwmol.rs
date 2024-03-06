use crate::ffi;
use crate::general::wrap::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct RWMol {
	pub(crate) foreign: ffi::rdkit_RWMol,
}

impl InitParams<RWMol> for () {}
impl Initializable<()> for RWMol {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_rwmol_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}
impl Uninitializable for RWMol {
	unsafe fn uninit(value: *mut Self) {
		ffi::rdkit_rwmol_dtor(std::ptr::addr_of_mut!((*value).foreign));
	}
}

pub struct RWMolInitParamsROMol<'s, Native: AsForeign<ffi::rdkit_ROMol>> {
	pub romol: &'s Native,
}

impl<Native: AsForeign<ffi::rdkit_ROMol>> InitParams<RWMol> for RWMolInitParamsROMol<'_, Native> {}
impl<Native: AsForeign<ffi::rdkit_ROMol>> Initializable<RWMolInitParamsROMol<'_, Native>> for RWMol {
	unsafe fn init(value: *mut Self, params: RWMolInitParamsROMol<Native>) -> Result<(), ()> {
		ffi::rdkit_rwmol_ctor_from_romol(std::ptr::addr_of_mut!((*value).foreign), params.romol.as_foreign());
		Ok(())
	}
}

impl Drop for RWMol {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_rwmol_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for RWMol {
	type ForeignSelf = ffi::rdkit_RWMol;
}
impl FFIBindingSelfNative for ffi::rdkit_RWMol {
	type NativeSelf = RWMol;
}

impl AsForeign<ffi::rdkit_ROMol> for RWMol {
	fn as_foreign(&self) -> &ffi::rdkit_ROMol {
		let ptr: *const ffi::rdkit_RWMol = self.as_foreign();
		unsafe { &*ffi::rdkit_rwmol_to_romol(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_ROMol {
		unsafe { &mut *ffi::rdkit_rwmol_to_romol(self.as_foreign_mut()) }
	}
}

impl AsForeign<ffi::rdkit_RDProps> for RWMol {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let s: &ffi::rdkit_ROMol = self.as_foreign();
		s.as_native().as_foreign()
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		let s: &mut ffi::rdkit_ROMol = self.as_foreign_mut();
		s.as_native_mut().as_foreign_mut()
	}
}

pub trait RWMolImplRef<Native> {
}
pub trait RWMolImplMut<Native>
where
	Self: InitializedMut<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
}

impl<'s, Wrapper, Native: 's> RWMolImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
}

impl<'s, Wrapper, Native: 's> RWMolImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_RWMol>,
	Native: AsForeign<ffi::rdkit_RWMol>,
{
}
