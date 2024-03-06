use std::mem::MaybeUninit;

use crate::ffi;
use crate::general::string::*;
use crate::general::foreign::*;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::RDProps;
	pub use super::RDPropsImplRef;
	pub use super::RDPropsImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RDProps {
	foreign: ffi::rdkit_RDProps,
}

impl InitParams<RDProps> for () {}
impl Initializable<()> for RDProps {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_rdprops_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for RDProps {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_rdprops_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for RDProps {
	type ForeignSelf = ffi::rdkit_RDProps;
}
impl FFIBindingSelfNative for ffi::rdkit_RDProps {
	type NativeSelf = RDProps;
}

pub trait RDPropsImplRef<Native>
{
	fn get_prop_f32(&self, key: &str) -> Option<f32>;
	fn get_prop_f64(&self, key: &str) -> Option<f64>;
	fn get_prop_i32(&self, key: &str) -> Option<i32>;
	fn get_prop_str(&self, key: &str) -> Option<String>;
	fn get_prop_u32(&self, key: &str) -> Option<u32>;
	fn has_prop(&self, key: &str) -> bool;
}
pub trait RDPropsImplMut<Native>
{
	fn clear(&mut self);
	fn clear_computed_props(&mut self);
	fn clear_prop(&mut self, key: &str);
	fn set_prop_f32(&mut self, key: &str, val: f32);
	fn set_prop_f64(&mut self, key: &str, val: f64);
	fn set_prop_i32(&mut self, key: &str, val: i32);
	fn set_prop_str(&mut self, key: &str, val: &str);
	fn set_prop_u32(&mut self, key: &str, val: u32);
	fn update_props(&mut self, other: &RDProps, preserve_existing: Option<bool>);
}

impl<'s, Wrapper, Native: 's> RDPropsImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_RDProps>,
	Native: AsForeign<ffi::rdkit_RDProps>,
{
	fn get_prop_f32(&self, key: &str) -> Option<f32> {
		let mut uninit = MaybeUninit::<f32>::uninit();
		unsafe {
			ffi::rdkit_rdprops_get_prop_f32(
				self.as_ref().as_foreign(),
				RDstr::from(key).as_foreign(),
				uninit.as_mut_ptr()
			).then(|| uninit.assume_init())
		}
	}
	fn get_prop_f64(&self, key: &str) -> Option<f64> {
		let mut uninit = MaybeUninit::<f64>::uninit();
		unsafe {
			ffi::rdkit_rdprops_get_prop_f64(
				self.as_ref().as_foreign(),
				RDstr::from(key).as_foreign(),
				uninit.as_mut_ptr()
			).then(|| uninit.assume_init())
		}
	}
	fn get_prop_i32(&self, key: &str) -> Option<i32> {
		let mut uninit = MaybeUninit::<i32>::uninit();
		unsafe {
			ffi::rdkit_rdprops_get_prop_i32(
				self.as_ref().as_foreign(),
				RDstr::from(key).as_foreign(),
				uninit.as_mut_ptr()
			).then(|| uninit.assume_init())
		}
	}
	fn get_prop_str(&self, key: &str) -> Option<String> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_rdprops_get_prop_str(
					self.as_ref().as_foreign(),
					RDstr::from(key).as_foreign(),
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()))
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
		}
	}
	fn get_prop_u32(&self, key: &str) -> Option<u32> {
		let mut uninit = MaybeUninit::<u32>::uninit();
		let key = *RDstr::from(key).as_foreign();
		unsafe {
			ffi::rdkit_rdprops_get_prop_u32(
				self.as_ref().as_foreign(),
				&key,
				uninit.as_mut_ptr()
			).then(|| uninit.assume_init())
		}
	}
	fn has_prop(&self, key: &str) -> bool {
		unsafe {
			ffi::rdkit_rdprops_has_prop(
				self.as_ref().as_foreign(),
				RDstr::from(key).as_foreign())
		}
	}
}

impl<'s, Wrapper, Native: 's> RDPropsImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_RDProps>,
	Native: AsForeign<ffi::rdkit_RDProps>,
{
	fn clear(&mut self) {
		unsafe { ffi::rdkit_rdprops_clear(self.get_unchecked_mut().as_foreign_mut()); }
	}
	fn clear_computed_props(&mut self) {
		unsafe { ffi::rdkit_rdprops_clear_computed_props(self.get_unchecked_mut().as_foreign_mut()); }
	}
	fn clear_prop(&mut self, key: &str) {
		unsafe {
			ffi::rdkit_rdprops_clear_prop(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign());
		}
	}
	fn set_prop_f32(&mut self, key: &str, val: f32) {
		unsafe {
			ffi::rdkit_rdprops_set_prop_f32(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign(),
				val)
		}
	}
	fn set_prop_f64(&mut self, key: &str, val: f64) {
		unsafe {
			ffi::rdkit_rdprops_set_prop_f64(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign(),
				val)
		}
	}
	fn set_prop_i32(&mut self, key: &str, val: i32) {
		unsafe {
			ffi::rdkit_rdprops_set_prop_i32(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign(),
				val)
		}
	}
	fn set_prop_str(&mut self, key: &str, val: &str) {
		unsafe {
			ffi::rdkit_rdprops_set_prop_str(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign(),
				RDstr::from(val).as_foreign())
		}
	}
	fn set_prop_u32(&mut self, key: &str, val: u32) {
		unsafe {
			ffi::rdkit_rdprops_set_prop_u32(
				self.get_unchecked_mut().as_foreign_mut(),
				RDstr::from(key).as_foreign(),
				val)
		}
	}
	fn update_props(&mut self, source: &RDProps, preserve_existing: Option<bool>) {
		unsafe {
			ffi::rdkit_rdprops_update_props_ex(
				self.get_unchecked_mut().as_foreign_mut(),
				source.as_foreign(),
				preserve_existing.as_foreign())
		}
	}
}
