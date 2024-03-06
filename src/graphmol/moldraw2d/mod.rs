use std::ptr;

use itertools::Itertools;

use crate::ffi;
use crate::general::string::*;
use crate::general::wrap::*;

pub mod moldraw2dsvg;

pub mod prelude {
	pub use super::MolDraw2D;
	pub use super::MolDraw2DImplRef;
	pub use super::MolDraw2DImplMut;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MolDraw2D {
	foreign: ffi::rdkit_MolDraw2D,
}

impl Drop for MolDraw2D {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_draw_2d_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MolDraw2D {
	type ForeignSelf = ffi::rdkit_MolDraw2D;
}
impl FFIBindingSelfNative for ffi::rdkit_MolDraw2D {
	type NativeSelf = MolDraw2D;
}

pub trait MolDraw2DImplRef<Native> {}
pub trait MolDraw2DImplMut<Native>
{
	fn clear_drawing(&mut self);
	fn draw_molecule<
		MolNative: AsForeign<ffi::rdkit_ROMol>,
		Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>
	>(&mut self, mol: &Mol, legend: Option<&str>);
	fn draw_molecules<
		MolNative: AsForeign<ffi::rdkit_ROMol>,
		Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>
	>(&mut self, mol: &[&Mol], legend: Option<&[&str]>);
}

impl<'s, Wrapper, Native: 's> MolDraw2DImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_MolDraw2D>,
	Native: AsForeign<ffi::rdkit_MolDraw2D>,
{}

impl<'s, Wrapper, Native: 's> MolDraw2DImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_MolDraw2D>,
	Native: AsForeign<ffi::rdkit_MolDraw2D>,
{
	fn clear_drawing(&mut self) {
		unsafe { ffi::rdkit_mol_draw_2d_clear_drawing(self.get_unchecked_mut().as_foreign_mut()); }
	}

	fn draw_molecule<
			MolNative: AsForeign<ffi::rdkit_ROMol>,
			Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>
		>(&mut self, mol: &Mol, legend: Option<&str>)
	{
		let legend = legend.unwrap_or("");

		unsafe { ffi::rdkit_mol_draw_2d_draw_molecule_ex(self.get_unchecked_mut().as_foreign_mut(), mol.as_ref().as_foreign(), RDstr::from(legend).as_foreign()); }
	}

	fn draw_molecules<
			MolNative: AsForeign<ffi::rdkit_ROMol>,
			Mol: InitializedRef<MolNative, ffi::rdkit_ROMol>
		>(&mut self, mols: &[&Mol], legends: Option<&[&str]>)
	{
		if let Some(legends) = legends {
			assert!(mols.len() == legends.len());
		}

		let cmols = mols.iter().map(|mol| mol.as_ref().as_foreign() as *const ffi::rdkit_ROMol).collect_vec();

		let legends = legends.map(|legends| legends.iter().map(|legend| RDstr::from(*legend)).collect_vec());
		let clegends = legends.as_ref().map(|legends| legends.iter().map(|legend| legend.as_foreign() as *const ffi::rdkit_string).collect_vec());

		unsafe { ffi::rdkit_mol_draw_2d_draw_molecules_ex(self.get_unchecked_mut().as_foreign_mut(), cmols.len(), cmols.as_ptr(), clegends.as_ref().map_or(ptr::null(), |legends| legends.as_ptr())); }

		// Prevent it from being drop too early
		drop(legends);
	}
}
