use std::mem::MaybeUninit;

use crate::ffi;
use crate::general::string::*;
use crate::general::wrap::*;
use crate::prelude::AsForeignOptionI32;

pub mod prelude {
	pub use super::MolDraw2DSVG;
	pub use super::MolDraw2DSVGImplRef;
	pub use super::MolDraw2DSVGImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MolDraw2DSVG {
	foreign: ffi::rdkit_MolDraw2DSVG,
}

pub struct MolDraw2DSVGInitParams {
	pub width: i32,
	pub height: i32,
	pub panel_width: Option<i32>,
	pub panel_height: Option<i32>,
}
impl InitParams<MolDraw2DSVG> for &MolDraw2DSVGInitParams {}
impl Initializable<&MolDraw2DSVGInitParams> for MolDraw2DSVG {
	unsafe fn init(value: *mut Self, params: &MolDraw2DSVGInitParams) -> Result<(), ()> {
		ffi::rdkit_mol_draw_2d_svg_ctor_ex(
			std::mem::transmute(value),
			params.width,
			params.height,
			params.panel_width.as_foreign(),
			params.panel_height.as_foreign());
		Ok(())
	}
}

impl Drop for MolDraw2DSVG {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_draw_2d_svg_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MolDraw2DSVG {
	type ForeignSelf = ffi::rdkit_MolDraw2DSVG;
}
impl FFIBindingSelfNative for ffi::rdkit_MolDraw2DSVG {
	type NativeSelf = MolDraw2DSVG;
}

impl AsForeign<ffi::rdkit_MolDraw2D> for MolDraw2DSVG {
	fn as_foreign(&self) -> &ffi::rdkit_MolDraw2D {
		let ptr: *const ffi::rdkit_MolDraw2DSVG = self.as_foreign();
		unsafe { &*ffi::rdkit_mol_draw_2d_svg_to_mol_draw_2d(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MolDraw2D {
		unsafe { &mut *ffi::rdkit_mol_draw_2d_svg_to_mol_draw_2d(self.as_foreign_mut()) }
	}
}

pub trait MolDraw2DSVGImplRef<Native> {
	fn get_drawing_text(&self) -> Option<String>;
}

impl<'s, Wrapper, Native: 's> MolDraw2DSVGImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_MolDraw2DSVG>,
	Native: AsForeign<ffi::rdkit_MolDraw2DSVG>,
{
	fn get_drawing_text(&self) -> Option<String> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_mol_draw_2d_svg_get_drawing_text(
					self.as_ref().as_foreign(),
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()))
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
		}
	}
}

pub trait MolDraw2DSVGImplMut<Native> {
	fn finish_drawing(&mut self);
}

impl<'s, Wrapper, Native: 's> MolDraw2DSVGImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_MolDraw2DSVG>,
	Native: AsForeign<ffi::rdkit_MolDraw2DSVG>,
{
	fn finish_drawing(&mut self) {
		unsafe { ffi::rdkit_mol_draw_2d_svg_finish_drawing(self.get_unchecked_mut().as_foreign_mut()); }
	}
}
