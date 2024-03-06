use crate::ffi;
use crate::general::wrap::*;
use crate::graphmol::romol::ROMolSptrVec;
use crate::graphmol::romol::ROMolSptrVecVec;

/*
TODO: Think about this later

#[repr(transparent)]
pub struct ValidChemicalReaction<'s> {
	reaction: Initialized<'s, ChemicalReaction>,
}

pub trait ChemicalReactionImpl<'s> {
	fn validate(self) -> ValidChemicalReaction<'s>;
}
*/

#[derive(Debug)]
#[repr(transparent)]
pub struct ChemicalReaction {
	pub(crate) foreign: ffi::rdkit_ChemicalReaction,
}

impl InitParams<ChemicalReaction> for () {}
impl Initializable<()> for ChemicalReaction {
	unsafe fn init(value: *mut Self, _params: ()) -> Result<(), ()> {
		ffi::rdkit_chemical_reaction_ctor(std::mem::transmute(value))
			.then_some(())
			.ok_or(())
	}
}

impl Drop for ChemicalReaction {
	fn drop(&mut self) {
		unsafe { ffi::rdkit_chemical_reaction_dtor(&mut self.foreign); }
	}
}

impl FFIBindingSelfForeign for ChemicalReaction {
	type ForeignSelf = ffi::rdkit_ChemicalReaction;
}
impl FFIBindingSelfNative for ffi::rdkit_ChemicalReaction {
	type NativeSelf = ChemicalReaction;
}

impl AsForeign<ffi::rdkit_RDProps> for ChemicalReaction {
	fn as_foreign(&self) -> &ffi::rdkit_RDProps {
		let ptr: *const ffi::rdkit_ChemicalReaction = self.as_foreign();
		unsafe { &*ffi::rdkit_chemical_reaction_to_rdprops(ptr.cast_mut()) }
	}
	fn as_foreign_mut(&mut self) -> &mut ffi::rdkit_RDProps {
		unsafe { &mut *ffi::rdkit_chemical_reaction_to_rdprops(self.as_foreign_mut()) }
	}
}

pub trait ChemicalReactionImplRef<Native>
where
	Self: InitializedRef<Native, ffi::rdkit_ChemicalReaction>,
	Native: AsForeign<ffi::rdkit_ChemicalReaction>,
{
	fn get_reactants(&self) -> InitializedForeignRef<&ROMolSptrVec>;
}
pub trait ChemicalReactionImplMut<Native>
where
	Self: InitializedMut<Native, ffi::rdkit_ChemicalReaction>,
	Native: AsForeign<ffi::rdkit_ChemicalReaction>,
{
	fn init_reactant_matchers(&mut self);
	fn run_reactants<ReactantsNative: AsForeign<ffi::rdkit_ROMol_sptr_vec>, Reactants: InitializedRef<ReactantsNative, ffi::rdkit_ROMol_sptr_vec>>(&self, reactants: &Reactants) -> Option<InitializedHeap<ROMolSptrVecVec>>;
}

impl<'s, Wrapper, Native: 's> ChemicalReactionImplRef<Native> for Wrapper
where
	Wrapper: InitializedRef<Native, ffi::rdkit_ChemicalReaction>,
	Native: AsForeign<ffi::rdkit_ChemicalReaction>,
{
	fn get_reactants(&self) -> InitializedForeignRef<&ROMolSptrVec> {
		unsafe {
			let ptr = ffi::rdkit_chemical_reaction_get_reactants(self.as_ref().as_foreign());
			InitializedForeignRef::from_raw(ptr.as_ref().unwrap().as_native())
		}
	}
}

impl<'s, Wrapper, Native: 's> ChemicalReactionImplMut<Native> for Wrapper
where
	Wrapper: InitializedMut<Native, ffi::rdkit_ChemicalReaction>,
	Native: AsForeign<ffi::rdkit_ChemicalReaction>,
{
	fn init_reactant_matchers(&mut self) {
		unsafe { ffi::rdkit_chemical_reaction_init_reactant_matchers(self.get_unchecked_mut().as_foreign_mut()) }
	}
	fn run_reactants<
		ReactantsNative: AsForeign<ffi::rdkit_ROMol_sptr_vec>,
		Reactants: InitializedRef<ReactantsNative, ffi::rdkit_ROMol_sptr_vec>
	>(&self, reactants: &Reactants) -> Option<InitializedHeap<ROMolSptrVecVec>> {
		let mut products = ROMolSptrVecVec::new_uninit();
		unsafe {
			ffi::rdkit_chemical_reaction_run_reactants(self.as_ref().as_foreign(), reactants.as_ref().as_foreign(), products.as_mut().as_foreign_mut()).then(|| products.assume_init())
		}
	}
}