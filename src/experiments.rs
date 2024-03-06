
/*
pub fn bli() {
	let rwmol = uninitialized_new!(RRWMol);
	let rwmol = rwmol.init(());
	//let a: &FFIRRWMol = rwmol.as_foreign_self();
	//let b: &ffi::rdkit_RWMol = rwmol.as_foreign_base();
	//let c: &ffi::rdkit_ROMol = rwmol.as_foreign_indirect();
	//let d: &ffi::rdkit_RDProps = rwmol.as_foreign_indirect();
	let e: &FFIRRWMol = rwmol.as_foreign();
	let f: &ffi::rdkit_RWMol = rwmol.as_foreign();
	//let g: &ffi::rdkit_ROMol = rwmol.as_foreign();
	//let h: &ffi::rdkit_RDProps = rwmol.as_foreign();
	//let h: &ffi::rdkit_ChemicalReaction = rwmol.as_foreign();
}
*/
/*
pub mod bla0 {
	trait FFIBindingKind {}

    struct KSelf {}
    struct KBase {}
    struct KIndirect {}

    impl FFIBindingKind for KSelf {}
    impl FFIBindingKind for KBase {}
    impl FFIBindingKind for KIndirect {}

    trait FFIBindingSelf {
        type ForeignSelf;
    }

    trait FFIBinding<Foreign> {
        type Kind: FFIBindingKind;
    }

    impl<Native, Foreign> FFIBinding<Foreign> for Native
    where
        Native: FFIBindingSelf<ForeignSelf = Foreign>
    {
        type Kind = KSelf;
    }

    trait AsForeignSelf: FFIBindingSelf {
        fn as_foreign_self(&self) -> &Self::ForeignSelf;
        fn as_foreign_self_mut(&mut self) -> &mut Self::ForeignSelf;
    }
    
    impl<N0, F0> AsForeignSelf for N0
    where
        N0: FFIBindingSelf<ForeignSelf = F0>,
    {
        fn as_foreign_self(&self) -> &F0 {
            unsafe { std::mem::transmute(self) }
        }
        fn as_foreign_self_mut(&mut self) -> &mut F0 {
            unsafe { std::mem::transmute(self) }
        }
    }
    
    trait AsNativeSelf<N0>
    where
        N0: FFIBindingSelf,
    {
        fn as_native_self(&self) -> &N0;
        fn as_native_self_mut(&mut self) -> &mut N0;
    }
    
    impl<N0, F0> AsNativeSelf<N0> for F0
    where
        N0: FFIBindingSelf<ForeignSelf = F0>,
    {
        fn as_native_self(&self) -> &N0 {
            unsafe { std::mem::transmute(self) }
        }
        fn as_native_self_mut(&mut self) -> &mut N0 {
            unsafe { std::mem::transmute(self) }
        }
    }

    trait AsForeignBase<F1>
    where
        Self: FFIBinding<F1, Kind = KBase>,
    {
        fn as_foreign_base(&self) -> &F1;
        fn as_foreign_base_mut(&mut self) -> &mut F1;
    }

    trait AsNativeBase<F1, N1>
    where
        Self: FFIBinding<F1, Kind = KBase>,
        N1: FFIBinding<F1, Kind = KSelf>,
    {
        fn as_foreign_base(&self) -> &N1;
        fn as_foreign_base_mut(&mut self) -> &mut N1;
    }

    trait AsForeignIndirect<F2>
    where
        Self: FFIBinding<F2, Kind = KIndirect>,
    {
        fn as_foreign_indirect(&self) -> &F2;
        fn as_foreign_indirect_mut(&mut self) -> &mut F2;
    }

    struct OSelf {}
    struct OBase {}
    struct OIndirect {}

    trait AsForeign<O, F> {
        fn as_foreign(&self) -> &F;
        fn as_foreign_mut(&mut self) -> &mut F;
    }

    impl<N0, F0> AsForeign<OSelf, F0> for N0
    where
        N0: AsForeignSelf<ForeignSelf = F0>,
    {
        fn as_foreign(&self) -> &F0 {
            self.as_foreign_self()
        }
        fn as_foreign_mut(&mut self) -> &mut F0 {
            self.as_foreign_self_mut()
        }
    }

    impl<N0, F1> AsForeign<OBase, F1> for N0
    where
        N0: AsForeignBase<F1>,
    {
        fn as_foreign(&self) -> &F1 {
            self.as_foreign_base()
        }
        fn as_foreign_mut(&mut self) -> &mut F1 {
            self.as_foreign_base_mut()
        }
    }

}
*/
/*
pub mod bla {
    use std::marker::PhantomData;

	pub struct TNative {}
	pub struct TForeign {}
	
	pub struct KSelf {}
	pub struct KBase {}

	// BindingSelf

	pub trait FFIBindingSelf {
		type TargetSelf;
		type RemoteSelf;
	}
	
	pub trait FFIBindingSelfNative: FFIBindingSelf<TargetSelf = TNative> {}
	pub trait FFIBindingSelfForeign: FFIBindingSelf<TargetSelf = TForeign> {}

	// BindingBase

	pub trait FFIBindingBase<Remote> {
		type TargetBase;
		type RemoteBase;
	}

	pub trait FFIBindingBaseForeign<F1>: FFIBindingBase<F1, TargetBase = TForeign, RemoteBase = F1>
	{
		fn as_remote_base(&self) -> &Self::RemoteBase;
	}

	// AsRemote

	pub trait AsRemote<Target, Kind, Remote> {
		fn as_remote(&self) -> &Remote;
	}

	mod private_as_remote {
		use super::*;

		trait AsRemoteHelper<Target, Kind, Remote> {
			type Target;
			type Kind;
			type Native;
			type Foreign;

			fn as_remote_imp(&self) -> &Remote;
		}
	
		impl<T: FFIBindingSelfNative> AsRemoteHelper<TNative, KSelf, T::RemoteSelf> for T {
			type Target = T::TargetSelf;
			type Kind = KSelf;
			type Native = T::RemoteSelf;
			type Foreign = T;

			fn as_remote_imp(&self) -> &T::RemoteSelf {
				unsafe { std::mem::transmute(self) }
			}
		}
	
		impl<T: FFIBindingSelfForeign> AsRemoteHelper<TForeign, KSelf, T::RemoteSelf> for T {
			type Target = T::TargetSelf;
			type Kind = KSelf;
			type Native = T;
			type Foreign = T::RemoteSelf;

			fn as_remote_imp(&self) -> &T::RemoteSelf {
				unsafe { std::mem::transmute(self) }
			}
		}

		impl<Foreign, T: FFIBindingBaseForeign<Foreign>> AsRemoteHelper<T::TargetBase, KBase, Foreign> for T {
			type Target = T::TargetBase;
			type Kind = KBase;
			type Native = T;
			type Foreign = T::RemoteBase;

			fn as_remote_imp(&self) -> &T::RemoteBase {
				self.as_remote_base()
			}
		}

		impl<Target, Kind, Remote, T: AsRemoteHelper<Target, Kind, Remote>> AsRemote<Target, Kind, Remote> for T {
			fn as_remote(&self) -> &Remote {
				self.as_remote_imp()
			}
		}
	}

	struct Native0 {}
	struct Foreign0 {}

	struct Native00 {}
	struct Foreign00 {}
	struct Native01 {}
	struct Foreign01 {}

	struct Native000 {}
	struct Foreign000 {}

	// 0

	impl FFIBindingSelf for Native0 {
		type TargetSelf = TForeign;
		type RemoteSelf = Foreign0;
	}
	impl FFIBindingSelfForeign for Native0 {}

	impl FFIBindingSelf for Foreign0 {
		type TargetSelf = TNative;
		type RemoteSelf = Native0;
	}
	impl FFIBindingSelfNative for Foreign0 {}

	impl FFIBindingBase<Foreign00> for Native0 {
		type TargetBase = TForeign;
		type RemoteBase = Foreign00;
	}
	impl FFIBindingBaseForeign<Foreign00> for Native0 {
		fn as_remote_base(&self) -> &Self::RemoteBase {
			unsafe { std::mem::transmute(self) }
		}
	}

	impl FFIBindingBase<Foreign01> for Native0 {
		type TargetBase = TForeign;
		type RemoteBase = Foreign01;
	}
	impl FFIBindingBaseForeign<Foreign01> for Native0 {
		fn as_remote_base(&self) -> &Self::RemoteBase {
			unsafe { std::mem::transmute(self) }
		}
	}

	// 1

	impl FFIBindingSelf for Native00 {
		type TargetSelf = TForeign;
		type RemoteSelf = Foreign00;
	}
	impl FFIBindingSelfForeign for Native00 {}

	impl FFIBindingSelf for Foreign00 {
		type TargetSelf = TNative;
		type RemoteSelf = Native00;
	}
	impl FFIBindingSelfNative for Foreign00 {}

	impl FFIBindingBase<Foreign000> for Native00 {
		type TargetBase = TForeign;
		type RemoteBase = Foreign000;
	}
	impl FFIBindingBaseForeign<Foreign000> for Native00 {
		fn as_remote_base(&self) -> &Self::RemoteBase {
			unsafe { std::mem::transmute(self) }
		}
	}

	impl FFIBindingSelf for Native01 {
		type TargetSelf = TForeign;
		type RemoteSelf = Foreign01;
	}
	impl FFIBindingSelfForeign for Native01 {}

	impl FFIBindingSelf for Foreign01 {
		type TargetSelf = TNative;
		type RemoteSelf = Native01;
	}
	impl FFIBindingSelfNative for Foreign01 {}

	// 2

	impl FFIBindingSelf for Native000 {
		type TargetSelf = TForeign;
		type RemoteSelf = Foreign000;
	}
	impl FFIBindingSelfForeign for Native000 {}

	impl FFIBindingSelf for Foreign000 {
		type TargetSelf = TNative;
		type RemoteSelf = Native000;
	}
	impl FFIBindingSelfNative for Foreign000 {}

	// test out the blanket impls
	// we expect to see the output
	// ```
	// impl Foo
	// impl Bar
	// ```
	fn bla() {
		let n0 = Native0{};
		let f0 = Foreign0{};
		let n00 = Native00{};
		let f00 = Foreign00{};
		let n000 = Native000{};
		let f000 = Foreign000{};
	
		let tmp: &Foreign0 = n0.as_remote();
		let tmp: &Foreign0 = n0.as_foreign();
		let tmp: &Foreign00 = n0.as_remote();
		let tmp: &Foreign00 = n0.as_foreign();
		let tmp: &Foreign01 = n0.as_remote();
		let tmp: &Foreign01 = n0.as_foreign();
	
		let tmp: &Native0 = f0.as_remote();
		let tmp: &Foreign00 = f0.as_remote().as_foreign();
		let tmp: &Foreign000 = f0.as_remote().as_foreign();
	}
}
*/
/*
	trait AsForeignSelf: FFIBindingSelf {
		fn as_foreign_self(&self) -> &Self::ForeignSelf;
		fn as_foreign_self_mut(&mut self) -> &mut Self::ForeignSelf;
	}
	
	impl<N0, F0> AsForeignSelf for N0
	where
		N0: FFIBindingSelf<ForeignSelf = F0>,
	{
		fn as_foreign_self(&self) -> &F0 {
			unsafe { std::mem::transmute(self) }
		}
		fn as_foreign_self_mut(&mut self) -> &mut F0 {
			unsafe { std::mem::transmute(self) }
		}
	}
	
	trait AsNativeSelf: FFIBindingSelf {
		fn as_native_self(&self) -> &Self::NativeSelf;
		fn as_native_self_mut(&mut self) -> &mut Self::NativeSelf;
	}
	
	impl<N0, F0> AsNativeSelf for F0
	where
		F0: FFIBindingSelf<NativeSelf = N0>,
	{
		fn as_native_self(&self) -> &N0 {
			unsafe { std::mem::transmute(self) }
		}
		fn as_native_self_mut(&mut self) -> &mut N0 {
			unsafe { std::mem::transmute(self) }
		}
	}

	trait AsForeignBase<F1>
	where
		Self: FFIBinding<F1, Kind = KBase>,
	{
		fn as_foreign_base(&self) -> &F1;
		fn as_foreign_base_mut(&mut self) -> &mut F1;
	}

	trait AsNativeBase<F1, N1>
	where
		Self: FFIBinding<F1, Kind = KBase>,
		N1: FFIBinding<F1, Kind = KSelf>,
	{
		fn as_foreign_base(&self) -> &N1;
		fn as_foreign_base_mut(&mut self) -> &mut N1;
	}

	trait AsForeignIndirect<F2>
	where
		Self: FFIBinding<F2, Kind = KIndirect>,
	{
		fn as_foreign_indirect(&self) -> &F2;
		fn as_foreign_indirect_mut(&mut self) -> &mut F2;
	}

	trait Origin {}

	struct OSelf {}
	struct OBase {}
	struct OIndirect {}

	impl Origin for OSelf {}
	impl Origin for OBase {}
	impl Origin for OIndirect {}

	trait AsForeign<O: Origin, F> {
		fn as_foreign(&self) -> &F;
		fn as_foreign_mut(&mut self) -> &mut F;
	}

	impl<N0, F0> AsForeign<OSelf, F0> for N0
	where
		N0: AsForeignSelf<ForeignSelf = F0>,
	{
		fn as_foreign(&self) -> &F0 {
			self.as_foreign_self()
		}
		fn as_foreign_mut(&mut self) -> &mut F0 {
			self.as_foreign_self_mut()
		}
	}

	impl<N0, F1> AsForeign<OBase, F1> for N0
	where
		N0: AsForeignBase<F1>,
	{
		fn as_foreign(&self) -> &F1 {
			self.as_foreign_base()
		}
		fn as_foreign_mut(&mut self) -> &mut F1 {
			self.as_foreign_base_mut()
		}
	}
/*
	impl<N0, N1, F1, F2> AsForeign<OIndirect, F2> for N0
	where
		N0: AsForeignBase<F1>,
		N1: AsForeignSelf<ForeignSelf = F1> + AsForeignBase<F2>,
	{
		fn as_foreign(&self) -> &F2 {
			self.as_foreign_base().as_native_self().as_foreign_base()
		}
		fn as_foreign_mut(&mut self) -> &mut F2 {
			self.as_foreign_base_mut().as_native_self_mut().as_foreign_base_mut()
		}
	}
*/
	struct Native0 {}
	struct Foreign0 {}
	struct Native1 {}
	struct Foreign1 {}
	struct Native2 {}
	struct Foreign2 {}
	struct Native3 {}
	struct Foreign3 {}

	impl FFIBindingSelf for Native0 {
		type ForeignSelf = Foreign0;
	}

	impl FFIBindingSelf for Native1 {
		type ForeignSelf = Foreign1;
	}

	impl FFIBindingSelf for Native2 {
		type ForeignSelf = Foreign2;
	}

	impl FFIBindingSelf for Native3 {
		type ForeignSelf = Foreign3;
	}

	impl FFIBinding<Foreign1> for Native0 {
		type Kind = KBase;
	}

	impl AsForeignBase<Foreign1> for Native0 {
		fn as_foreign_base(&self) -> &Foreign1 {
			unsafe { std::mem::transmute(self) }
		}
		fn as_foreign_base_mut(&mut self) -> &mut Foreign1 {
			unsafe { std::mem::transmute(self) }
		}
	}

	impl FFIBinding<Foreign2> for Native1 {
		type Kind = KBase;
	}

	impl AsForeignBase<Foreign2> for Native1 {
		fn as_foreign_base(&self) -> &Foreign2 {
			unsafe { std::mem::transmute(self) }
		}
		fn as_foreign_base_mut(&mut self) -> &mut Foreign2 {
			unsafe { std::mem::transmute(self) }
		}
	}

	impl FFIBinding<Foreign3> for Native2 {
		type Kind = KBase;
	}

	impl AsForeignBase<Foreign3> for Native2 {
		fn as_foreign_base(&self) -> &Foreign3 {
			unsafe { std::mem::transmute(self) }
		}
		fn as_foreign_base_mut(&mut self) -> &mut Foreign3 {
			unsafe { std::mem::transmute(self) }
		}
	}

	pub fn bla() {
		let n0: Native0 = Native0{};
		let n1: Native1 = Native1{};
		let n2: Native2 = Native2{};
		let n3: Native3 = Native3{};

		let f0: &Foreign0 = n0.as_foreign();
		let f1: &Foreign1 = n1.as_foreign();
		let f2: &Foreign2 = n2.as_foreign();
		let f3: &Foreign3 = n3.as_foreign();

		let f1: &Foreign1 = n0.as_foreign();
		//let f2: &Foreign2 = n0.as_foreign();
		//let f3: &Foreign3 = n0.as_foreign();
	}
}
*/
