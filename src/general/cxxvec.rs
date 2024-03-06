use crate::general::wrap::*;

pub mod prelude {
	pub use super::CxxVecImplRef;
	pub use super::CxxVecImplMut;
}

pub trait CxxVecImplRef<'s, Native, Foreign, ElemNative: 's, ElemForeign>
where
	Self: InitializedRef<Native, Foreign>,
	Native: AsForeign<Foreign>,
{
	fn get(&self, i: usize) -> Option<InitializedForeignRef<&ElemNative>>;
	fn size(&self) -> usize;
}
pub trait CxxVecImplMut<'s, Native, Foreign, ElemNative: 's, ElemForeign>
where
	Self: InitializedRef<Native, Foreign>,
	Native: AsForeign<Foreign>,
{
	fn get_mut(&mut self, i: usize) -> Option<InitializedForeignRef<&mut ElemNative>>;
	fn append<T: 's + InitializedOwned<'s, ElemNative, ElemForeign>>(&mut self, elem: T);
	fn append_mut<T: 's + InitializedMut<ElemNative, ElemForeign>>(&mut self, elem: &mut T);
	fn set<T: 's + InitializedOwned<'s, ElemNative, ElemForeign>>(&mut self, i: usize, elem: T);
	fn set_mut<T: 's + InitializedMut<ElemNative, ElemForeign>>(&mut self, i: usize, elem: &mut T);
	fn clear(&mut self);
	fn delete(&mut self, i: usize);
}