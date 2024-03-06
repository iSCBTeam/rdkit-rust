use std::mem::MaybeUninit;

use itertools::Itertools;
pub use rdkit_rust_sys as ffi;

use crate::general::string::*;
use crate::general::foreign::*;
use crate::general::wrap::*;
use crate::graphmol::romol::ROMol;
use crate::graphmol::rwmol::RWMol;
use crate::graphmol::chemreactions::reaction::ChemicalReaction;

pub mod contrib;
pub mod general;
pub mod geometry;
pub mod graphmol;

pub mod prelude {
	pub use crate::general::prelude::*;
	pub use super::new_local;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MolSupplier {
	foreign: ffi::rdkit_MolSupplier
}

impl Drop for MolSupplier {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_mol_supplier_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MolSupplier {
	type ForeignSelf = ffi::rdkit_MolSupplier;
}
impl FFIBindingSelfNative for ffi::rdkit_MolSupplier {
	type NativeSelf = MolSupplier;
}

pub trait MolSupplierImpl<Native> {
}

impl<'s, Wrapper, Native: 's> MolSupplierImpl<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_MolSupplier>,
	Native: AsForeign<ffi::rdkit_MolSupplier>,
{

}

#[derive(Debug)]
#[repr(transparent)]
pub struct MultithreadedMolSupplier {
	foreign: ffi::rdkit_MultithreadedMolSupplier
}

impl Drop for MultithreadedMolSupplier {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_multithreaded_mol_supplier_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MultithreadedMolSupplier {
	type ForeignSelf = ffi::rdkit_MultithreadedMolSupplier;
}
impl FFIBindingSelfNative for ffi::rdkit_MultithreadedMolSupplier {
	type NativeSelf = MultithreadedMolSupplier;
}

impl AsForeign<ffi::rdkit_MolSupplier> for MultithreadedMolSupplier {
	fn as_foreign(&self) -> &ffi::rdkit_MolSupplier {
		let ptr: *const ffi::rdkit_MultithreadedMolSupplier = self.as_foreign();
		unsafe { &*ffi::rdkit_multithreaded_mol_supplier_to_mol_supplier(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MolSupplier {
		unsafe { &mut *ffi::rdkit_multithreaded_mol_supplier_to_mol_supplier(self.as_foreign_mut()) }
	}
}

pub trait MultithreadedMolSupplierImpl {
}

impl<Native> MultithreadedMolSupplierImpl for Native
where
	Native: AsForeign<ffi::rdkit_MultithreadedMolSupplier>,
{

}

#[derive(Debug)]
#[repr(transparent)]
pub struct MultithreadedSDMolSupplier {
	foreign: ffi::rdkit_MultithreadedSDMolSupplier
}

impl InitParams<MultithreadedSDMolSupplier> for () {}
impl Initializable<()> for MultithreadedSDMolSupplier {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_multithreaded_sd_mol_supplier_ctor(std::ptr::addr_of_mut!((*value).foreign));
		Ok(())
	}
}

#[derive(Debug)]
pub struct MultithreadedSDMolSupplierInitParamsFilename<'s> {
	pub filename: &'s str,
}

impl InitParams<MultithreadedSDMolSupplier> for MultithreadedSDMolSupplierInitParamsFilename<'_> {}
impl Initializable<MultithreadedSDMolSupplierInitParamsFilename<'_>> for MultithreadedSDMolSupplier {
	unsafe fn init(value: *mut Self, params: MultithreadedSDMolSupplierInitParamsFilename) -> Result<(), ()> {
		let filename = std::ffi::CString::new(params.filename).unwrap();

		ffi::rdkit_multithreaded_sd_mol_supplier_ctor_filename(
				std::ptr::addr_of_mut!((*value).foreign),
				filename.as_ptr(),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub struct MultithreadedSDMolSupplierInitParamsFilenameEx<'s> {
	pub filename: &'s str,
	pub sanitize: Option<bool>,
	pub remove_hs: Option<bool>,
	pub strict_parsing: Option<bool>,
	pub num_writer_threads: Option<usize>,
	pub size_input_queue: Option<usize>,
	pub size_output_queue: Option<usize>,
}

impl InitParams<MultithreadedSDMolSupplier> for MultithreadedSDMolSupplierInitParamsFilenameEx<'_> {}
impl Initializable<MultithreadedSDMolSupplierInitParamsFilenameEx<'_>> for MultithreadedSDMolSupplier {
	unsafe fn init(value: *mut Self, params: MultithreadedSDMolSupplierInitParamsFilenameEx) -> Result<(), ()> {
		let filename = std::ffi::CString::new(params.filename).unwrap();

		ffi::rdkit_multithreaded_sd_mol_supplier_ctor_filename_ex(
				std::ptr::addr_of_mut!((*value).foreign),
				filename.as_ptr(),
				params.sanitize.as_foreign(),
				params.remove_hs.as_foreign(),
				params.strict_parsing.as_foreign(),
				params.num_writer_threads.as_foreign(),
				params.size_input_queue.as_foreign(),
				params.size_output_queue.as_foreign(),
			)
			.then_some(())
			.ok_or(())
	}
}

impl Drop for MultithreadedSDMolSupplier {
	fn drop(&mut self) {
		//println!("Dropping MultithreadedSDMolSupplier");
		unsafe { ffi::rdkit_multithreaded_sd_mol_supplier_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MultithreadedSDMolSupplier {
	type ForeignSelf = ffi::rdkit_MultithreadedSDMolSupplier;
}
impl FFIBindingSelfNative for ffi::rdkit_MultithreadedSDMolSupplier {
	type NativeSelf = MultithreadedSDMolSupplier;
}

impl AsForeign<ffi::rdkit_MultithreadedMolSupplier> for MultithreadedSDMolSupplier {
	fn as_foreign(&self) -> &ffi::rdkit_MultithreadedMolSupplier {
		let ptr: *const ffi::rdkit_MultithreadedSDMolSupplier = self.as_foreign();
		unsafe { &*ffi::rdkit_multithreaded_sd_mol_supplier_to_multithreaded_mol_supplier(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MultithreadedMolSupplier {
		unsafe { &mut *ffi::rdkit_multithreaded_sd_mol_supplier_to_multithreaded_mol_supplier(self.as_foreign_mut()) }
	}
}

impl AsForeign<ffi::rdkit_MolSupplier> for MultithreadedSDMolSupplier {
	fn as_foreign(&self) -> &ffi::rdkit_MolSupplier {
		let s: &ffi::rdkit_MultithreadedMolSupplier = self.as_foreign();
		s.as_native().as_foreign()
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MolSupplier {
		let s: &mut ffi::rdkit_MultithreadedMolSupplier = self.as_foreign_mut();
		s.as_native_mut().as_foreign_mut()
	}
}

pub trait MultithreadedSDMolSupplierImpl {}

impl<Native> MultithreadedSDMolSupplierImpl for Native
where
	Native: AsForeign<ffi::rdkit_MultithreadedSDMolSupplier>,
{}

impl<'s> Iterator for &'s mut MultithreadedSDMolSupplier {
	type Item = InitializedHeap<'s, ROMol>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut uninit = ROMol::new_uninit();
		let mut got_next = false;
		let mut at_end = false;

		while !got_next && !at_end {
			got_next = unsafe { ffi::rdkit_mol_supplier_next(self.as_foreign_mut(), uninit.as_mut().as_foreign_mut()) };
			if !got_next {
				at_end = unsafe { ffi::rdkit_mol_supplier_at_end(self.as_foreign_mut()) };
			}
		}

		if !got_next {
			return None;
		}

		Some(unsafe { uninit.assume_init() })
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MultithreadedSmilesMolSupplier {
	foreign: ffi::rdkit_MultithreadedSmilesMolSupplier
}

impl InitParams<MultithreadedSmilesMolSupplier> for () {}
impl Initializable<()> for MultithreadedSmilesMolSupplier {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_multithreaded_smiles_mol_supplier_ctor(std::ptr::addr_of_mut!((*value).foreign))
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub struct MultithreadedSmilesMolSupplierInitParamsFilenameEx<'s> {
	pub filename: &'s str,
	pub delimiter: Option<&'s str>,
	pub smiles_column: Option<i32>,
	pub name_column: Option<i32>,
	pub title_line: Option<bool>,
	pub sanitize: Option<bool>,
	pub num_writer_threads: Option<usize>,
	pub size_input_queue: Option<usize>,
	pub size_output_queue: Option<usize>,
}

impl InitParams<MultithreadedSmilesMolSupplier> for MultithreadedSmilesMolSupplierInitParamsFilenameEx<'_> {}
impl Initializable<MultithreadedSmilesMolSupplierInitParamsFilenameEx<'_>> for MultithreadedSmilesMolSupplier {
	unsafe fn init(value: *mut Self, params: MultithreadedSmilesMolSupplierInitParamsFilenameEx) -> Result<(), ()> {
		let filename = std::ffi::CString::new(params.filename).unwrap();
		let delimiter = params.delimiter.map(|e| crate::RDstr::from(e));

		ffi::rdkit_multithreaded_smiles_mol_supplier_ctor_filename_ex(
				std::ptr::addr_of_mut!((*value).foreign),
				filename.as_ptr(),
				delimiter.as_foreign(),
				params.smiles_column.as_foreign(),
				params.name_column.as_foreign(),
				params.title_line.as_foreign(),
				params.sanitize.as_foreign(),
				params.num_writer_threads.as_foreign(),
				params.size_input_queue.as_foreign(),
				params.size_output_queue.as_foreign(),
			)
			.then_some(())
			.ok_or(())
	}
}

impl Drop for MultithreadedSmilesMolSupplier {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_multithreaded_smiles_mol_supplier_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for MultithreadedSmilesMolSupplier {
	type ForeignSelf = ffi::rdkit_MultithreadedSmilesMolSupplier;
}
impl FFIBindingSelfNative for ffi::rdkit_MultithreadedSmilesMolSupplier {
	type NativeSelf = MultithreadedSmilesMolSupplier;
}

impl AsForeign<ffi::rdkit_MultithreadedMolSupplier> for MultithreadedSmilesMolSupplier {
	fn as_foreign(&self) -> &ffi::rdkit_MultithreadedMolSupplier {
		let ptr: *const ffi::rdkit_MultithreadedSmilesMolSupplier = self.as_foreign();
		unsafe { &*ffi::rdkit_multithreaded_smiles_mol_supplier_to_multithreaded_mol_supplier(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MultithreadedMolSupplier {
		unsafe { &mut *ffi::rdkit_multithreaded_smiles_mol_supplier_to_multithreaded_mol_supplier(self.as_foreign_mut()) }
	}
}

impl AsForeign<ffi::rdkit_MolSupplier> for MultithreadedSmilesMolSupplier {
	fn as_foreign(&self) -> &ffi::rdkit_MolSupplier {
		let s: &ffi::rdkit_MultithreadedMolSupplier = self.as_foreign();
		s.as_native().as_foreign()
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_MolSupplier {
		let s: &mut ffi::rdkit_MultithreadedMolSupplier = self.as_foreign_mut();
		s.as_native_mut().as_foreign_mut()
	}
}

pub trait MultithreadedSmilesMolSupplierImpl {}

impl<Native> MultithreadedSmilesMolSupplierImpl for Native
where
	Native: AsForeign<ffi::rdkit_MultithreadedSmilesMolSupplier>,
{}

impl<'s> Iterator for &'s mut MultithreadedSmilesMolSupplier {
	type Item = InitializedHeap<'s, ROMol>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut uninit = ROMol::new_uninit();
		let mut got_next = false;
		let mut at_end = false;

		while !got_next && !at_end {
			got_next = unsafe { ffi::rdkit_mol_supplier_next(self.as_foreign_mut(), uninit.as_mut().as_foreign_mut()) };
			if !got_next {
				at_end = unsafe { ffi::rdkit_mol_supplier_at_end(self.as_foreign_mut()) };
			}
		}

		if !got_next {
			return None;
		}

		Some(unsafe { uninit.assume_init() })
	}
}

#[derive(Debug)]
pub enum SmilesWriteError {
	Error
}

pub trait SmilesWriteImpl {
	fn to_smiles(&self) -> Result<String, SmilesWriteError>;
	//fn to_smiles_ex(&self) -> Result<String, SmilesWriteError>;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> SmilesWriteImpl for T {
	fn to_smiles(&self) -> Result<String, SmilesWriteError> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_mol_to_smiles(
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()),
					self.as_foreign())
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
				.ok_or(SmilesWriteError::Error)
		}
	}
	/*fn to_smiles_ex(&self) -> Result<String, SmilesWriteError> {
		unsafe {
			let mut uninit: MaybeUninit<RDString> = MaybeUninit::uninit();
			let ptr = uninit.as_mut_ptr();
			// TODO: Handle return
			ffi::rdkit_mol_to_smiles_ex(std::ptr::addr_of_mut!((*ptr).foreign), self.as_foreign()
		);
			uninit.assume_init().as_str().map(|s| s.to_owned())
		}
	}*/
}

#[derive(Debug)]
pub enum SDWriteError {
	Error
}

pub trait SDWriteImpl {
	fn to_sd(&self) -> Result<String, SDWriteError>;
	fn to_sd_ex(&self, conf_id: Option<i32>, kekulize: Option<bool>, force_v3000: Option<bool>, mol_id: Option<i32>, prop_names: Option<&[&str]>) -> Result<String, SDWriteError>;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> SDWriteImpl for T {
	fn to_sd(&self) -> Result<String, SDWriteError> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_sd_writer_get_text(
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()),
					self.as_foreign())
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
				.ok_or(SDWriteError::Error)
		}
	}
	fn to_sd_ex(&self, conf_id: Option<i32>, kekulize: Option<bool>, force_v3000: Option<bool>, mol_id: Option<i32>, prop_names: Option<&[&str]>) -> Result<String, SDWriteError> {
		let mut uninit = MaybeUninit::<RDString>::uninit();
		let prop_names_bound = prop_names.map(|names|
			names.iter().map(|name| std::ffi::CString::new(*name).unwrap()).collect_vec());
		let prop_names = prop_names_bound.as_ref().map(|names|
			names.iter().map(|name| name.as_ptr()).collect_vec());

		unsafe {
			ffi::rdkit_sd_writer_get_text_ex(
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()),
					self.as_foreign(),
					conf_id.as_foreign(),
					kekulize.as_foreign(),
					force_v3000.as_foreign(),
					mol_id.as_foreign(),
					prop_names.as_ref().map(|p| p.len()).as_foreign(),
					prop_names.as_ref().map(|p| p.as_slice().as_ptr()).unwrap_or(std::ptr::null()),
				)
				.then(|| uninit.assume_init())
				.and_then(|s| s.as_str().ok().map(|s| s.to_owned()))
				.ok_or(SDWriteError::Error)
		}
	}
}

#[derive(Debug)]
pub struct ParseSmartsParamsChemicalReaction<'s> {
	pub text: &'s str,
}

impl InitParams<ChemicalReaction> for ParseSmartsParamsChemicalReaction<'_> {}
impl Initializable<ParseSmartsParamsChemicalReaction<'_>> for ChemicalReaction {
	unsafe fn init(value: *mut Self, params: ParseSmartsParamsChemicalReaction) -> Result<(), ()> {
		ffi::rdkit_rxn_smarts_to_chemical_reaction(
				std::ptr::addr_of_mut!((*value).foreign),
				crate::RDstr::from(params.text).as_foreign(),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub struct ParseSmartsParams<'s> {
	pub text: &'s str,
	pub debug_parse: Option<i32>,
	pub merge_hs: Option<bool>,
	pub replacements: (),
}

impl InitParams<RWMol> for ParseSmartsParams<'_> {}
impl Initializable<ParseSmartsParams<'_>> for RWMol {
	unsafe fn init(value: *mut Self, params: ParseSmartsParams) -> Result<(), ()> {
		ffi::rdkit_smarts_to_mol_ex(
				std::ptr::addr_of_mut!((*value).foreign),
				crate::RDstr::from(params.text).as_foreign(),
				params.debug_parse.as_foreign(),
				params.merge_hs.as_foreign(),
				std::ptr::null(),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub struct ParseSmilesParams<'s> {
	pub text: &'s str,
	pub debug_parse: Option<i32>,
	pub sanitize: Option<bool>,
	pub replacements: (),
}

impl InitParams<RWMol> for ParseSmilesParams<'_> {}
impl Initializable<ParseSmilesParams<'_>> for RWMol {
	unsafe fn init(value: *mut Self, params: ParseSmilesParams) -> Result<(), ()> {
		ffi::rdkit_smiles_to_mol_ex(
				std::ptr::addr_of_mut!((*value).foreign),
				crate::RDstr::from(params.text).as_foreign(),
				params.debug_parse.as_foreign(),
				params.sanitize.as_foreign(),
				std::ptr::null(),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub struct ParseMolBlockParams<'s> {
	pub mol_block: &'s str,
	pub sanitize: Option<bool>,
	pub remove_hs: Option<bool>,
	pub strict_parsing: Option<bool>,
}

impl InitParams<RWMol> for ParseMolBlockParams<'_> {}
impl Initializable<ParseMolBlockParams<'_>> for RWMol {
	unsafe fn init(value: *mut Self, params: ParseMolBlockParams) -> Result<(), ()> {
		ffi::rdkit_mol_block_to_mol_ex(
				std::ptr::addr_of_mut!((*value).foreign),
				crate::RDstr::from(params.mol_block).as_foreign(),
				params.sanitize.as_foreign(),
				params.remove_hs.as_foreign(),
				params.strict_parsing.as_foreign(),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub enum MolPicklerError {
	Error
}

#[derive(Clone, Copy, Debug)]
pub struct PickleMolOptions {
	pub mol_props: bool,
	pub atom_props: bool,
	pub bond_props: bool,
	pub private_props: bool,
	pub computed_props: bool,
	pub coords_as_double: bool,
	pub no_conformers: bool,
}

impl PickleMolOptions {
	fn as_foreign(self) -> ffi::rdkit_mol_pickler_OptionFlags {
		let mut res = 0;

		if self.mol_props {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_MOL_PROPS;
		}
		if self.atom_props {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_ATOM_PROPS;
		}
		if self.bond_props {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_BOND_PROPS;
		}
		if self.private_props {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_PRIVATE_PROPS;
		}
		if self.computed_props {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_COMPUTED_PROPS;
		}
		if self.coords_as_double {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_COORDS_AS_DOUBLE;
		}
		if self.no_conformers {
			res |= ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_NO_CONFORMERS;
		}

		res
	}

	unsafe fn from_foreign(flags: ffi::rdkit_mol_pickler_OptionFlags) -> Self {
		Self {
			mol_props: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_MOL_PROPS) != 0,
			atom_props: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_ATOM_PROPS) != 0,
			bond_props: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_BOND_PROPS) != 0,
			private_props: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_PRIVATE_PROPS) != 0,
			computed_props: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_COMPUTED_PROPS) != 0,
			coords_as_double: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_COORDS_AS_DOUBLE) != 0,
			no_conformers: (flags & ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_NO_CONFORMERS) != 0,
		}
	}
}

impl Default for PickleMolOptions {
	fn default() -> Self {
		unsafe { Self::from_foreign(ffi::rdkit_mol_pickler_OptionFlags_RDKIT_MOL_PICKLER_OPTION_DEFAULT) }
	}
}

pub trait PickleMolImpl {
	fn to_pickle(&self, flags: Option<PickleMolOptions>) -> Result<Vec<u8>, MolPicklerError>;
}

impl<Native> PickleMolImpl for Native
where
	Native: AsForeign<ffi::rdkit_ROMol>,
{
	fn to_pickle(&self, flags: Option<PickleMolOptions>) -> Result<Vec<u8>, MolPicklerError> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		let flags = flags.unwrap_or_default();

		unsafe {
			ffi::rdkit_mol_pickler_pickle_mol_ex(
					self.as_foreign(),
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()),
					flags.as_foreign())
				.then(|| uninit.assume_init())
				.and_then(|pickle| Some(pickle.as_rdstr().as_bytes().to_vec()))
				.ok_or(MolPicklerError::Error)
		}
	}
}

#[derive(Debug)]
pub struct ROMolFromPickleParams<'s> {
	pub pickle: &'s [u8],
}

impl InitParams<ROMol> for ROMolFromPickleParams<'_> {}
impl Initializable<ROMolFromPickleParams<'_>> for ROMol {
	unsafe fn init(value: *mut Self, params: ROMolFromPickleParams) -> Result<(), ()> {
		ffi::rdkit_mol_pickler_mol_from_pickle(
				RDstr::from(params.pickle).as_foreign(),
				std::ptr::addr_of_mut!((*value).foreign),
			)
			.then_some(())
			.ok_or(())
	}
}

#[derive(Debug)]
pub enum ChemicalReactionPicklerError {
	Error
}

pub trait PickleChemicalReactionImpl {
	fn to_pickle(&self) -> Result<Vec<u8>, ChemicalReactionPicklerError>;
}

impl<Native> PickleChemicalReactionImpl for Native
where
	Native: AsForeign<ffi::rdkit_ChemicalReaction>,
{
	fn to_pickle(&self) -> Result<Vec<u8>, ChemicalReactionPicklerError> {
		let mut uninit = MaybeUninit::<RDString>::uninit();

		unsafe {
			ffi::rdkit_reaction_pickler_pickle_reaction(
					self.as_foreign(),
					RDString::as_foreign_ptr_mut(uninit.as_mut_ptr()))
				.then(|| uninit.assume_init())
				.and_then(|pickle| Some(pickle.as_rdstr().as_bytes().to_vec()))
				.ok_or(ChemicalReactionPicklerError::Error)
		}	
	}
}

#[derive(Debug)]
pub struct ChemicalReactionFromPickleParams<'s> {
	pub pickle: &'s [u8],
}

impl InitParams<ChemicalReaction> for ChemicalReactionFromPickleParams<'_> {}
impl Initializable<ChemicalReactionFromPickleParams<'_>> for ChemicalReaction {
	unsafe fn init(value: *mut Self, params: ChemicalReactionFromPickleParams) -> Result<(), ()> {
		ffi::rdkit_reaction_pickler_reaction_from_pickle(
				crate::RDstr::from(params.pickle).as_foreign(),
				std::ptr::addr_of_mut!((*value).foreign),
			)
			.then_some(())
			.ok_or(())
	}
}
