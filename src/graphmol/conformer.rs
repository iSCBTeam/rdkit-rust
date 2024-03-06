use crate::ffi;
use crate::general::wrap::*;
use crate::geometry::point3d::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct Conformer {
	foreign: ffi::rdkit_Conformer,
}

impl InitParams<Conformer> for () {}
impl Initializable<()> for Conformer {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_conformer_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for Conformer {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_conformer_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Conformer {
	type ForeignSelf = ffi::rdkit_Conformer;
}
impl FFIBindingSelfNative for ffi::rdkit_Conformer {
	type NativeSelf = Conformer;
}

impl AsForeign<ffi::rdkit_RDProps> for Conformer {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let ptr: *const ffi::rdkit_Conformer = self.as_foreign();
		unsafe { &*ffi::rdkit_conformer_to_rdprops(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		unsafe { &mut *ffi::rdkit_conformer_to_rdprops(self.as_foreign_mut()) }
	}
}

pub trait ConformerImplRef<Native> {
	fn get_num_atoms(&self) -> u32;
	fn get_atom_pos(&self, idx: u32) -> Option<(f64, f64, f64)>;
}

impl<'s, Wrapper, Native: 's> ConformerImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_Conformer>,
	Native: AsForeign<ffi::rdkit_Conformer>,
{
	fn get_num_atoms(&self) -> u32 {
		unsafe {
			ffi::rdkit_conformer_get_num_atoms(self.as_ref().as_foreign())
		}
	}
	fn get_atom_pos(&self, idx: u32) -> Option<(f64, f64, f64)> {
		unsafe {
			let ptr = ffi::rdkit_conformer_get_atom_pos(self.as_ref().as_foreign(), idx);
			ptr
				.as_ref()
				.and_then(|r|
					Some(InitializedForeignRef::from_raw(r.as_native())
						.get_coords()))
		}
	}
}

pub trait ConformerImplMut<Native> {}

impl<'s, Wrapper, Native: 's> ConformerImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_Conformer>,
	Native: AsForeign<ffi::rdkit_Conformer>,
{}
