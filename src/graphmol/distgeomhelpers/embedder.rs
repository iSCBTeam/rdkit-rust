use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::EmbedderImpl;
}

#[derive(Debug)]
pub enum EmbedMoleculeError {
	Error
}

pub trait EmbedderImpl {
	fn embed_molecule(&mut self) -> Result<i32, EmbedMoleculeError>;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> EmbedderImpl for T {
	fn embed_molecule(&mut self) -> Result<i32, EmbedMoleculeError> {
		let res = unsafe { ffi::rdkit_dist_geom_helpers_embed_molecule(self.as_foreign_mut()) };
        (res >= 0)
			.then_some(res)
			.ok_or(EmbedMoleculeError::Error)
	}
}
