use crate::ffi;
use crate::general::wrap::*;

pub mod prelude {
	pub use super::GasteigerChargesImpl;
}

#[derive(Debug)]
pub enum ComputeGasteigerChargesError {
	Error
}

pub trait GasteigerChargesImpl {
	fn compute_gasteiger_charges(&mut self) -> Result<(), ComputeGasteigerChargesError>;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> GasteigerChargesImpl for T {
	fn compute_gasteiger_charges(&mut self) -> Result<(), ComputeGasteigerChargesError> {
		unsafe { ffi::rdkit_compute_gasteiger_charges(self.as_foreign_mut()) }
			.then_some(())
			.ok_or(ComputeGasteigerChargesError::Error)
	}
}
