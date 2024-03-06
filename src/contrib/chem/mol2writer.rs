use crate::ffi;
use crate::graphmol::atom::AtomImplRef;
use crate::graphmol::bond::BondImplRef;
use crate::graphmol::bond::BondType;
use crate::graphmol::conformer::ConformerImplRef;
use crate::new_local;
use crate::general::wrap::*;
use crate::graphmol::partialcharges::gasteigercharges::prelude::*;
use crate::graphmol::romol::*;
use crate::graphmol::rwmol::*;
use crate::graphmol::molops::*;
use crate::prelude::*;

#[derive(Debug)]
pub enum Mol2WriteError {
	AddHs,
	AddCharges,
	GetAtom,
	GetBond,
}

pub trait Mol2WriteImpl {
	fn to_mol2(&self) -> Result<String, Mol2WriteError>;
}

impl<'s, T: AsForeign<ffi::rdkit_ROMol>> Mol2WriteImpl for T {
	fn to_mol2(&self) -> Result<String, Mol2WriteError> {
		let mol = new_local!(RWMol);
		let mut mol = mol.init(RWMolInitParamsROMol {
			romol: self,
		}).unwrap();

		mol.add_hs(
			Default::default(),
			Some(true),
			Default::default())
			.map_err(|_| Mol2WriteError::AddHs)?;

		mol.compute_gasteiger_charges()
			.map_err(|_| Mol2WriteError::AddCharges)?;

		let name = mol.get_prop_str("_Name")
			.unwrap_or("UNK".to_owned());

		let atom_cnt = mol.get_num_atoms();
		let bond_cnt = mol.get_num_bonds();
		let conformer_cnt = mol.get_num_conformers();

		let mut data = String::new();

		for idx_conformer in 0..conformer_cnt {
			let conformer = mol
				.get_conformer(idx_conformer as i32)
				.unwrap();

			let hdr = format!(
				concat!(
					"\n@<TRIPOS>MOLECULE\n",
					"{}_conf{}\n",
					"{} {} 0 0 0\n",
					"SMALL\n",
					"GASTEIGER\n",
				),
				name, idx_conformer,
				atom_cnt, bond_cnt);

			data.push_str(&hdr);

			data.push_str("\n@<TRIPOS>ATOM\n");

			for idx_atom in 0..atom_cnt {
				let atom = mol
					.get_atom(idx_atom)
					.ok_or(Mol2WriteError::GetAtom)?;

				let symbol = atom
					.get_symbol()
					.unwrap_or_else(|| atom.get_atomic_num().to_string());

				let charge = atom
					.get_prop_f64("_GasteigerCharge")
					.unwrap_or(0.0);

				let pos = conformer
					.get_atom_pos(idx_atom)
					.unwrap();

				let atom_line = format!(
					"{:>4} {:>4} {:>13.4} {:>9.4} {:>9.4} {:<5} {} {} {:>7.4}\n",
					idx_atom + 1,
					symbol,
					pos.0, pos.1, pos.2,
					symbol /* TODO: Sybyl atom type */,
					1,
					"UNL",
					charge,
				);

				data.push_str(&atom_line);
			}

			data.push_str("\n@<TRIPOS>BOND\n");

			for idx_bond in 0..bond_cnt {
				let bond = mol
					.get_bond(idx_bond)
					.ok_or(Mol2WriteError::GetBond)?;

				let idx_begin_atom = bond
					.get_begin_atom_idx();

				let idx_end_atom = bond
					.get_end_atom_idx();

				let bond_type = bond
					.get_bond_type();
				let bond_type = match bond_type {
					BondType::Aromatic => "ar".to_owned(),
					_ => bond.get_bond_type_as_f64().to_string(),
				};

				let bond_line = format!(
					"{:>5} {:>5} {:>5} {:>2}\n",
					idx_bond + 1,
					idx_begin_atom + 1,
					idx_end_atom + 1,
					bond_type,
				);

				data.push_str(&bond_line);
			}
		}

		Ok(data)
	}
}
