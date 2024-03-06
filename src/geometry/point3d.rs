use std::mem::MaybeUninit;

use crate::ffi;
use crate::general::wrap::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct Point3D {
	foreign: ffi::rdkit_geom_Point3D,
}

impl InitParams<Point3D> for () {}
impl Initializable<()> for Point3D {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_geom_point3d_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for Point3D {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_geom_point3d_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Point3D {
	type ForeignSelf = ffi::rdkit_geom_Point3D;
}
impl FFIBindingSelfNative for ffi::rdkit_geom_Point3D {
	type NativeSelf = Point3D;
}

pub trait Point3DImplRef<Native> {
	fn get_coords(&self) -> (f64, f64, f64);
}

impl<'s, Wrapper, Native: 's> Point3DImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_geom_Point3D>,
	Native: AsForeign<ffi::rdkit_geom_Point3D>,
{
	fn get_coords(&self) -> (f64, f64, f64) {
        let mut x = MaybeUninit::<f64>::uninit();
        let mut y = MaybeUninit::<f64>::uninit();
        let mut z = MaybeUninit::<f64>::uninit();

		unsafe {
			ffi::rdkit_geom_point3d_get_coords(self.as_ref().as_foreign(), x.as_mut_ptr(), y.as_mut_ptr(), z.as_mut_ptr());
            (x.assume_init(), y.assume_init(), z.assume_init())
		}
	}
}

pub trait Point3DImplMut<Native> {}

impl<'s, Wrapper, Native: 's> Point3DImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_geom_Point3D>,
	Native: AsForeign<ffi::rdkit_geom_Point3D>,
{}