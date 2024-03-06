use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;

pub mod prelude {
	pub use super::InitializedHeap;
	pub use super::InitializeHeapWithParams;
	pub use super::InitializedLocal;
	pub use super::InitializeLocalWithParams;
	pub use super::NewInitialized;
	pub use super::NewUninitialized;
	pub use super::UninitializeHeap;
	pub use super::UninitializeLocal;
}

pub trait AsForeign<Foreign> {
	fn as_foreign(&self) -> &Foreign;
	fn as_foreign_mut(&mut self) -> &mut Foreign;
}

pub trait AsNative<Native> {
	fn as_native(&self) -> &Native;
	fn as_native_mut(&mut self) -> &mut Native;
}

pub trait FFIBindingSelfForeign {
	type ForeignSelf;
}

pub trait FFIBindingSelfNative {
	type NativeSelf;
}

impl<Foreign, Native: FFIBindingSelfForeign<ForeignSelf = Foreign>> AsForeign<Foreign> for Native {
	fn as_foreign(&self) -> &Foreign {
		unsafe { std::mem::transmute(self) }
	}
	fn as_foreign_mut(&mut self) -> &mut Foreign {
		unsafe { std::mem::transmute(self) }
	}
}

impl<Native, Foreign: FFIBindingSelfNative<NativeSelf = Native>> AsNative<Native> for Foreign {
	fn as_native(&self) -> &Native {
		unsafe { std::mem::transmute(self) }
	}
	fn as_native_mut(&mut self) -> &mut Native {
		unsafe { std::mem::transmute(self) }
	}
}

pub trait Initializable<P>
where
	Self: Sized,
	P: InitParams<Self>,
{
	unsafe fn init(value: *mut Self, params: P) -> Result<(), ()>;
}

pub trait InitParams<T> {}

pub trait Uninitializable
where
	Self: Sized,
{
	unsafe fn uninit(value: *mut Self);
}

#[derive(Debug)]
#[repr(transparent)]
pub struct LocalWrapper<T>(pub ManuallyDrop<T>);

#[derive(Debug)]
#[repr(transparent)]
pub struct HeapWrapper<T>(ManuallyDrop<Box<ManuallyDrop<T>>>);

#[derive(Debug)]
#[repr(transparent)]
pub struct ForeignRefWrapper<T>(T);

impl<'s, T> LocalWrapper<T> {
	pub unsafe fn from_raw(raw: *mut ManuallyDrop<T>) -> &'s mut Self {
		std::mem::transmute(raw)
	}
	pub fn to_raw(val: &mut LocalWrapper<T>) -> *mut ManuallyDrop<T> {
		&mut val.0
	}
}

impl<T> HeapWrapper<T> {
	pub unsafe fn from_raw(raw: *mut ManuallyDrop<T>) -> Self {
		HeapWrapper(ManuallyDrop::new(Box::from_raw(&mut *raw)))
	}
	pub fn into_raw(val: HeapWrapper<T>) -> *mut ManuallyDrop<T> {
		Box::into_raw(ManuallyDrop::into_inner(val.0))
	}
}

impl<T> ForeignRefWrapper<T> {
	pub unsafe fn from_raw<'s>(raw: *const T) -> &'s Self {
		std::mem::transmute(raw)
	}
	pub unsafe fn from_raw_mut<'s>(raw: *mut T) -> &'s mut Self {
		std::mem::transmute(raw)
	}
	pub fn into_raw(val: &Self) -> *const T {
		&val.0
	}
	pub fn into_raw_mut(val: &mut Self) -> *mut T {
		&mut val.0
	}
}

impl<'s, T> Deref for LocalWrapper<T> {
	type Target = ManuallyDrop<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<'s, T> DerefMut for LocalWrapper<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T> Deref for HeapWrapper<T> {
	type Target = ManuallyDrop<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> DerefMut for HeapWrapper<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T> Deref for ForeignRefWrapper<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> DerefMut for ForeignRefWrapper<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct UninitializedLocal<'s, T: 's> {
	#[doc(hidden)]
	pub value: Pin<&'s mut LocalWrapper<T>>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct UninitializedHeap<'s, T> {
	value: Pin<HeapWrapper<T>>,
	_value_lifetime: PhantomData<&'s mut T>,
}

impl<'s, T> UninitializedLocal<'s, T> {
	pub unsafe fn as_ref(&self) -> Pin<&LocalWrapper<T>> {
		self.value.as_ref()
	}
	pub unsafe fn as_mut(&mut self) -> Pin<&mut LocalWrapper<T>> {
		self.value.as_mut()
	}
	pub unsafe fn assume_init(self) -> InitializedHeap<'s, T> {
		let mut this = ManuallyDrop::new(self);
		let ptr = &mut **this.value.as_mut().get_unchecked_mut();

		InitializedHeap {
			value: Pin::new_unchecked(HeapWrapper::from_raw(ptr)),
			_value_lifetime: PhantomData,
		}
	}
}
impl<'s, T> UninitializedHeap<'s, T> {
	pub unsafe fn as_ref(&self) -> Pin<&ManuallyDrop<T>> {
		self.value.as_ref()
	}
	pub unsafe fn as_mut(&mut self) -> Pin<&mut ManuallyDrop<T>> {
		self.value.as_mut()
	}
	pub unsafe fn assume_init(self) -> InitializedHeap<'s, T> {
		let mut this = ManuallyDrop::new(self);
		let ptr = this.value.as_mut().get_unchecked_mut();

		InitializedHeap {
			value: Pin::new_unchecked(HeapWrapper::from_raw(ptr)),
			_value_lifetime: PhantomData,
		}
	}
}
/*
impl<'s, T> Drop for UninitializedLocal<'s, T> {
	fn drop(&mut self) {
		//println!("Dropping UninitializedLocal<{}>", std::any::type_name::<T>());
		// Nothing to drop
	}
}
*/
impl<'s, T> Drop for UninitializedHeap<'s, T> {
	fn drop(&mut self) {
		//println!("Dropping UninitializedHeap<{}>", std::any::type_name::<T>());
		unsafe {
			let ref_content = self.value.as_mut().get_unchecked_mut();
			let mut storage = ManuallyDrop::new(Box::from_raw(ref_content));
			ManuallyDrop::drop(&mut storage);
		}
	}
}

pub trait InitializeLocalWithParams<'s, T, P>
where
	Self: Sized,
	T: Initializable<P>,
	P: InitParams<T>,
{
	fn init(self, params: P) -> Result<InitializedLocal<'s, T>, Self>;
}
pub trait InitializeHeapWithParams<'s, T, P>
where
	Self: Sized,
	T: Initializable<P>,
	P: InitParams<T>,
{
	fn init(self, params: P) -> Result<InitializedHeap<'s, T>, Self>;
}

impl<'s, T, P> InitializeLocalWithParams<'s, T, P> for UninitializedLocal<'s, T>
where
	T: Initializable<P>,
	P: InitParams<T>,
{
	fn init(self, params: P) -> Result<InitializedLocal<'s, T>, Self> {
		unsafe {
			let mut this = ManuallyDrop::new(self);
			let ptr = &mut **this.value.as_mut().get_unchecked_mut();

			T::init(&mut **ptr, params)
				.map_err(|_| UninitializedLocal {
					value: Pin::new_unchecked(LocalWrapper::from_raw(ptr))
				})
				.map(|_| InitializedLocal {
					value: Pin::new_unchecked(LocalWrapper::from_raw(ptr))
				})
		}
	}
}
impl<'s, T, P> InitializeHeapWithParams<'s, T, P> for UninitializedHeap<'s, T>
where
	T: Initializable<P>,
	P: InitParams<T>,
{
	fn init(self, params: P) -> Result<InitializedHeap<'s, T>, Self> {
		unsafe {
			let mut this = ManuallyDrop::new(self);
			let ptr = this.value.as_mut().get_unchecked_mut();

			T::init(&mut **ptr, params)
			.map_err(|_| UninitializedHeap {
				value: Pin::new_unchecked(HeapWrapper::from_raw(ptr)),
				_value_lifetime: PhantomData,
			})
			.map(|_| InitializedHeap {
				value: Pin::new_unchecked(HeapWrapper::from_raw(ptr)),
				_value_lifetime: PhantomData,
			})
		}
	}
}

pub trait UninitializeLocal<'s, T>
where
	T: Uninitializable,
{
	fn uninit(self) -> UninitializedLocal<'s, T>;
}
pub trait UninitializeHeap<'s, T>
where
	T: Uninitializable,
{
	fn uninit(self) -> UninitializedHeap<'s, T>;
}

impl<'s, T> UninitializeLocal<'s, T> for InitializedLocal<'s, T>
where
	T: Uninitializable,
{
	fn uninit(self) -> UninitializedLocal<'s, T> {
		unsafe {
			let mut this = ManuallyDrop::new(self);
			let ptr = &mut **this.value.as_mut().get_unchecked_mut();

			T::uninit(&mut **ptr);

			UninitializedLocal {
				value: Pin::new_unchecked(LocalWrapper::from_raw(ptr))
			}
		}
	}
}
impl<'s, T> UninitializeHeap<'s, T> for InitializedHeap<'s, T>
where
	T: Uninitializable,
{
	fn uninit(self) -> UninitializedHeap<'s, T> {
		unsafe {
			let mut this = ManuallyDrop::new(self);
			let ptr = this.value.as_mut().get_unchecked_mut();

			T::uninit(&mut **ptr);

			UninitializedHeap {
				value: Pin::new_unchecked(HeapWrapper::from_raw(ptr)),
				_value_lifetime: PhantomData,
			}
		}
	}
}

pub trait NewUninitialized<'s, T>
{
	fn new_uninit() -> UninitializedHeap<'s, T>;
}

impl<'s, T> NewUninitialized<'s, T> for T
where
	T: FFIBindingSelfForeign,
{
	fn new_uninit() -> UninitializedHeap<'s, T> {
		unsafe {
			UninitializedHeap {
				// Use ::zeroed() to avoid leaving padding bytes uninitialized
				value: Pin::new_unchecked(HeapWrapper(ManuallyDrop::new(Box::new(ManuallyDrop::new(std::mem::zeroed()))))),
				_value_lifetime: PhantomData,
			}
		}
	}
}

pub trait NewInitialized<'s, P, T>
{
	fn new(params: P) -> Result<InitializedHeap<'s, T>, ()>;
}

impl<'s, T, P> NewInitialized<'s, P, T> for T
where
	T: Initializable<P> + NewUninitialized<'s, T>,
	P: InitParams<T>,
{
	fn new(params: P) -> Result<InitializedHeap<'s, T>, ()> {
		Self::new_uninit()
			.init(params)
			.map_err(|_| ())
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! new_local {
	($t: ty) => {
		crate::general::wrap::UninitializedLocal::<$t> {
			// Use ::zeroed() to avoid leaving padding bytes uninitialized
			value: unsafe { std::pin::pin!(crate::general::wrap::LocalWrapper(std::mem::zeroed())) },
		}
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InitializedLocal<'s, T> {
	pub value: Pin<&'s mut LocalWrapper<T>>, // FIXME: private
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InitializedHeap<'s, T> {
	value: Pin<HeapWrapper<T>>, // FIXME: private
	_value_lifetime: PhantomData<&'s mut T>, // FIXME: private
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InitializedForeignRef<T>(Pin<T>);

#[derive(Debug)]
pub enum InitializedInnerImpl<'s, T> {
	Local(&'s mut ManuallyDrop<T>),
	Heap(Box<ManuallyDrop<T>>),
}

mod initialized_impl {
	use std::pin::Pin;

	pub trait Ref<Native, Foreign> {
		fn as_ref(&self) -> Pin<&Native>;
		fn get_ref(&self) -> &Native;
	}
	pub trait Mut<Native, Foreign>: Ref<Native, Foreign> {
		fn as_mut(&mut self) -> Pin<&mut Native>;
		unsafe fn get_unchecked_mut(&mut self) -> &mut Native;
	}

	pub trait Owned<'s, Native: 's, Foreign> {
		unsafe fn into_inner(self) -> super::InitializedInner<'s, Native>;
	}
}

pub use initialized_impl::Ref as InitializedRef;
pub use initialized_impl::Mut as InitializedMut;
pub use initialized_impl::Owned as InitializedOwned;

#[derive(Debug)]
#[repr(transparent)]
pub struct InitializedInner<'s, Native>(pub InitializedInnerImpl<'s, Native>);

impl<'s, Native: AsForeign<Foreign>, Foreign> InitializedRef<Native, Foreign> for InitializedLocal<'s, Native> {
	fn as_ref(&self) -> Pin<&Native> {
		unsafe { self.value.as_ref().map_unchecked(|val| &***val) }
	}
	fn get_ref(&self) -> &Native {
		self.as_ref().get_ref()
	}
}
impl<'s, Native: AsForeign<Foreign>, Foreign> InitializedMut<Native, Foreign> for InitializedLocal<'s, Native> {
	fn as_mut(&mut self) -> Pin<&mut Native> {
		unsafe { self.value.as_mut().map_unchecked_mut(|val| &mut ***val) }
	}
	unsafe fn get_unchecked_mut(&mut self) -> &mut Native {
		self.as_mut().get_unchecked_mut()
	}
}
impl<'s, Native: AsForeign<Foreign>, Foreign> InitializedOwned<'s, Native, Foreign> for InitializedLocal<'s, Native> {
	unsafe fn into_inner(self) -> InitializedInner<'s, Native> {
		let mut this = ManuallyDrop::new(self);
		let ptr = this.value.as_mut().get_unchecked_mut();
		InitializedInner(InitializedInnerImpl::Local(&mut *(ptr as *mut LocalWrapper<Native>)))
	}
}
impl<'s, Native: AsForeign<Foreign>, Foreign> AsForeign<Foreign> for InitializedLocal<'s, Native> {
	fn as_foreign(&self) -> &Foreign {
		self.get_ref().as_foreign()
	}
	fn as_foreign_mut(&mut self) -> &mut Foreign {
		unsafe { self.get_unchecked_mut() }.as_foreign_mut()
	}
}

impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedRef<Native, Foreign> for InitializedHeap<'s, Native>
{
	fn as_ref(&self) -> Pin<&Native> {
		unsafe { self.value.as_ref().map_unchecked(|val| &**val) }
	}
	fn get_ref(&self) -> &Native {
		self.as_ref().get_ref()
	}
}
impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedMut<Native, Foreign> for InitializedHeap<'s, Native>
{
	fn as_mut(&mut self) -> Pin<&mut Native> {
		unsafe { self.value.as_mut().map_unchecked_mut(|val| &mut **val) }
	}
	unsafe fn get_unchecked_mut(&mut self) -> &mut Native {
		self.as_mut().get_unchecked_mut()
	}
}
impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedOwned<'s, Native, Foreign> for InitializedHeap<'s, Native>
{
	unsafe fn into_inner(self) -> InitializedInner<'s, Native> {
		let mut this = ManuallyDrop::new(self);
		let ptr = this.value.as_mut().get_unchecked_mut();
		InitializedInner(InitializedInnerImpl::Heap(Box::from_raw(ptr)))
	}
}
impl<'s, Native: AsForeign<Foreign>, Foreign> AsForeign<Foreign> for InitializedHeap<'s, Native> {
	fn as_foreign(&self) -> &Foreign {
		self.get_ref().as_foreign()
	}
	fn as_foreign_mut(&mut self) -> &mut Foreign {
		unsafe { self.get_unchecked_mut() }.as_foreign_mut()
	}
}

impl<'s, T: 's> InitializedForeignRef<T> {
	pub unsafe fn from_raw(ptr: *const T) -> InitializedForeignRef<&'s T> {
		InitializedForeignRef(Pin::new_unchecked(&*ptr))
	}
	pub unsafe fn from_raw_mut(ptr: *mut T) -> InitializedForeignRef<&'s mut T> {
		InitializedForeignRef(Pin::new_unchecked(&mut *ptr))
	}
}

impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedRef<Native, Foreign> for InitializedForeignRef<&mut Native>
{
	fn as_ref(&self) -> Pin<&Native> {
		unsafe { self.0.as_ref().map_unchecked(|val| val) }
	}
	fn get_ref(&self) -> &Native {
		self.as_ref().get_ref()
	}
}
impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedMut<Native, Foreign> for InitializedForeignRef<&mut Native>
{
	fn as_mut(&mut self) -> Pin<&mut Native> {
		unsafe { self.0.as_mut().map_unchecked_mut(|val| val) }
	}
	unsafe fn get_unchecked_mut(&mut self) -> &mut Native {
		self.as_mut().get_unchecked_mut()
	}
}

impl<'s, Native: 's + AsForeign<Foreign>, Foreign> InitializedRef<Native, Foreign> for InitializedForeignRef<&Native>
{
	fn as_ref(&self) -> Pin<&Native> {
		unsafe { self.0.as_ref().map_unchecked(|val| val) }
	}
	fn get_ref(&self) -> &Native {
		self.as_ref().get_ref()
	}
}

unsafe impl<'s, T> Send for InitializedHeap<'s, T> {}

impl<'s, T> Deref for InitializedLocal<'s, T> {
    type Target = Pin<&'s mut LocalWrapper<T>>;

    fn deref(&self) -> &Self::Target {
		&self.value
    }
}

impl<'s, T> DerefMut for InitializedLocal<'s, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
    }
}

impl<'s, T> Deref for InitializedHeap<'s, T> {
    type Target = Pin<HeapWrapper<T>>;

    fn deref(&self) -> &Self::Target {
		&self.value
    }
}

impl<'s, T> DerefMut for InitializedHeap<'s, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
    }
}

impl<'s, T> Deref for InitializedForeignRef<&'s T> {
    type Target = Pin<&'s T>;

    fn deref(&self) -> &Self::Target {
		&self.0
    }
}

impl<'s, T> Deref for InitializedForeignRef<&'s mut T> {
    type Target = Pin<&'s mut T>;

    fn deref(&self) -> &Self::Target {
		&self.0
    }
}

impl<'s, T> DerefMut for InitializedForeignRef<&'s mut T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
    }
}

impl<'s, T> Drop for InitializedLocal<'s, T> {
	fn drop(&mut self) {
		//println!("Dropping InitializedLocal<{}>", std::any::type_name::<T>());
		unsafe {
			let ref_storage = self.value.as_mut().get_unchecked_mut();
			let ref_content = &mut **ref_storage;
			ManuallyDrop::drop(ref_content);
			// The storage is stack-based, so there is nothing to drop.
		}
	}
}

impl<'s, T> Drop for InitializedHeap<'s, T> {
	fn drop(&mut self) {
		//println!("Dropping InitializedHeap<{}>", std::any::type_name::<T>());
		unsafe {
			let ref_content = self.value.as_mut().get_unchecked_mut();
			let mut storage = ManuallyDrop::new(Box::from_raw(ref_content));
			ManuallyDrop::drop(ref_content);
			ManuallyDrop::drop(&mut storage);
		}
	}
}
