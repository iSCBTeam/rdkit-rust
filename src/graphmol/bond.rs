use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::ffi;
use crate::general::wrap::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct Bond {
	foreign: ffi::rdkit_Bond,
}

impl InitParams<Bond> for () {}
impl Initializable<()> for Bond {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_bond_ctor(std::mem::transmute(value));
		Ok(())
	}
}

impl Drop for Bond {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_bond_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for Bond {
	type ForeignSelf = ffi::rdkit_Bond;
}
impl FFIBindingSelfNative for ffi::rdkit_Bond {
	type NativeSelf = Bond;
}

impl AsForeign<ffi::rdkit_RDProps> for Bond {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let ptr: *const ffi::rdkit_Bond = self.as_foreign();
		unsafe { &*ffi::rdkit_bond_to_rdprops(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		unsafe { &mut *ffi::rdkit_bond_to_rdprops(self.as_foreign_mut()) }
	}
}

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum BondType {
	Unspecified = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_UNSPECIFIED,
	Single = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_SINGLE,
	Double = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_DOUBLE,
	Triple = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_TRIPLE,
	Quadruple = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_QUADRUPLE,
	Quintuple = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_QUINTUPLE,
	Hextuple = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_HEXTUPLE,
	OneAndAHalf = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_ONEANDAHALF,
	TwoAndAHalf = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_TWOANDAHALF,
	ThreeAndAHalf = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_THREEANDAHALF,
	FourAndAHalf = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_FOURANDAHALF,
	FiveAndAHalf = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_FIVEANDAHALF,
	Aromatic = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_AROMATIC,
	Ionic = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_IONIC,
	Hydrogen = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_HYDROGEN,
	ThreeCenter = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_THREECENTER,
	DativeOne = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_DATIVEONE,
	Dative = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_DATIVE,
	DativeL = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_DATIVEL,
	DativeR = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_DATIVER,
	Other = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_OTHER,
	Zero = ffi::rdkit_bond_BondType_RDKIT_BOND_BOND_TYPE_ZERO,
}

pub trait BondImplRef<Native> {
	fn get_bond_type(&self) -> BondType;
	fn get_bond_type_as_f64(&self) -> f64;
	fn get_begin_atom_idx(&self) -> u32;
	fn get_end_atom_idx(&self) -> u32;
}

impl<'s, Wrapper, Native: 's> BondImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_Bond>,
	Native: AsForeign<ffi::rdkit_Bond>,
{
	fn get_bond_type(&self) -> BondType {
		let bond_type = unsafe { ffi::rdkit_bond_get_bond_type(self.as_ref().as_foreign()) };
		BondType::try_from(bond_type)
			.unwrap()
	}
	fn get_bond_type_as_f64(&self) -> f64 {
		unsafe {
			ffi::rdkit_bond_get_bond_type_as_double(self.as_ref().as_foreign())
		}
	}
	fn get_begin_atom_idx(&self) -> u32 {
		unsafe {
			ffi::rdkit_bond_get_begin_atom_idx(self.as_ref().as_foreign())
		}
	}
	fn get_end_atom_idx(&self) -> u32 {
		unsafe {
			ffi::rdkit_bond_get_end_atom_idx(self.as_ref().as_foreign())
		}
	}
}

pub trait BondImplMut<Native> {}

impl<'s, Wrapper, Native: 's> BondImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_Bond>,
	Native: AsForeign<ffi::rdkit_Bond>,
{}
