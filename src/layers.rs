// [[file:../spdkit-surface.note::135ef16f][135ef16f]]
use crate::common::*;

use gchemol::Lattice;
use vecfx::*;
// 135ef16f ends here

// [[file:../spdkit-surface.note::2378149a][2378149a]]
fn reorder_atoms_by_zfrac_coords(mol: &Molecule) -> Option<Vec<usize>> {
    let mut zfrac_coords_numbers: Vec<_> = mol.get_scaled_positions()?.map(|[_, _, fz]| fz).zip(mol.numbers()).collect();
    zfrac_coords_numbers.sort_by_cached_key(|k| k.0.as_ordered_float());
    zfrac_coords_numbers.into_iter().map(|(_, i)| i).collect_vec().into()
}

// Check if mol contains atom `i` by atom label set previously
fn contains_atom(mol: &Molecule, i: usize) -> bool {
    mol.atoms().find(|(_, a)| a.label() == format!("{i}")).is_some()
}

#[track_caller]
/// Return atoms in bottom layer which contains `bottom_atom`
fn process_bottom_layer(layers: &mut Vec<Molecule>, bottom_atom: usize) -> Vec<usize> {
    let i = layers
        .iter()
        .position(|mol| contains_atom(mol, bottom_atom))
        .expect("no layer for bottom atom");
    let mol = layers.remove(i);
    mol.atoms().map(|(_, a)| a.label().parse().unwrap()).collect()
}
// 2378149a ends here

// [[file:../spdkit-surface.note::4299ffbb][4299ffbb]]
#[track_caller]
/// Fragment molecule into connected parts by layer for periodic slab model.
pub fn fragment_atoms_by_layer(mol: &Molecule) -> Result<impl Iterator<Item = Vec<usize>>> {
    let mut mol = mol.clone();
    ensure!(mol.is_periodic(), "only works for periodic structure!");

    // reorder atoms by their zfrac coords
    let reorder_atoms = reorder_atoms_by_zfrac_coords(&mol).unwrap();

    // label each atom with its serial number so we can find it when
    // fragmented with this label
    let numbers = mol.numbers().collect_vec();
    for i in numbers {
        let a = mol.get_atom_unchecked_mut(i);
        a.set_label(format!("{i}"));
    }

    // scale the lattice along c direction
    let frac_coords: Vec<_> = mol.get_scaled_positions().unwrap().collect();
    mol.get_lattice_mut().map(|lat| lat.scale_by_c(2.0));
    mol.set_scaled_positions(frac_coords);
    // the recalculate connectivity without considiering lattice
    std::env::set_var("GCHEMOL_REBOND_IGNORE_PBC", format!("true"));
    mol.rebond();
    let mut layers = mol.fragmented().collect_vec();

    let mut remained = reorder_atoms.clone();
    let iter = std::iter::from_fn(move || {
        // find bottom layer
        if remained.is_empty() {
            None
        } else {
            let bottom_atom = remained[0];
            let bottom_atoms = process_bottom_layer(&mut layers, bottom_atom);
            // remove atoms in bottom layer
            remained.retain(|sn| !bottom_atoms.contains(sn));
            Some(bottom_atoms)
        }
    });

    Ok(iter)
}
// 4299ffbb ends here

// [[file:../spdkit-surface.note::9d23e311][9d23e311]]
#[test]
fn test_atom_layer() -> Result<()> {
    let f = "tests/files/R.xsd";
    let m = Molecule::from_file(f)?;
    let layers = fragment_atoms_by_layer(&m)?.collect_vec();

    assert!(layers.len() > 4);
    assert_eq!(layers[0].len(), 24);
    assert_eq!(layers[1].len(), 27);
    assert_eq!(layers[2].len(), 24);
    assert_eq!(layers[3].len(), 12);

    Ok(())
}
// 9d23e311 ends here
