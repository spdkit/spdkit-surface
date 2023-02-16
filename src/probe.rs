// [[file:../spdkit-surface.note::db9e4081][db9e4081]]
use crate::common::*;

use gchemol::neighbors::{Neighbor, Neighborhood};
use gchemol::Lattice;
use vecfx::*;
// db9e4081 ends here

// [[file:../spdkit-surface.note::1fc3542c][1fc3542c]]
fn guess_appropriate_zstep(lat: &Lattice, radius: f64) -> f64 {
    assert!(radius.is_sign_positive(), "invalid radius: {}", radius);
    let [_, _, zwidth] = lat.widths();
    radius / zwidth
}

fn probe_surface_neighgbors_along_z(
    probe: &Neighborhood,
    lat: &gchemol::Lattice,
    x: f64,
    y: f64,
    zhigh: f64,
    zstep: f64,
    r_cutoff: f64,
) -> Result<Vec<Neighbor>> {
    assert!(zhigh >= 0.0, "invalid zhigh: {}", zhigh);

    // test if current probe is above or below the surface
    let p = lat.to_cart([x, y, zhigh]);
    let n = probe.search(p.into(), r_cutoff).collect_vec();
    let mut i = 0;
    if n.is_empty() {
        // above the surface: move down step by step until touch the surface atoms
        let mut z = zhigh;
        loop {
            i += 1;
            z -= zstep;
            let p = lat.to_cart([x, y, z]);
            let n = probe.search(p.into(), r_cutoff).collect_vec();
            if n.len() > 0 || z <= 0.0 {
                trace!("probe surface atoms by moving down done in {} iterations", i);
                break Ok(n);
            }
        }
    } else {
        // below the surface: move up step by step until move away the surface atoms
        let mut z = zhigh;
        let mut n_pre = n;
        loop {
            i += 1;
            z += zstep;
            let p = lat.to_cart([x, y, z]);
            let n = probe.search(p.into(), r_cutoff).collect_vec();
            if n.len() == 0 || z >= 1.0 {
                trace!("probe surface atoms by moving up done in {} iterations", i);
                break Ok(n_pre);
            }
            n_pre = n;
        }
    }
}
// 1fc3542c ends here

// [[file:../spdkit-surface.note::c4191fe3][c4191fe3]]
/// Probe surface atoms approaching slab surface from top in z-axis
/// gradually
///
/// # Parameters
/// * mol: the molecule to be probed (slab model)
/// * r_cutoff: the cutoff radius for probing surface atoms
/// * n_probes: the number of random probe atoms
pub fn probe_surface_atoms(mol: &Molecule, r_cutoff: f64, n_probes: usize) -> Result<Vec<usize>> {
    let n_probes = 800;
    ensure!(mol.lattice.is_some(), "only work for periodic system");

    let zhigh = mol.get_scaled_positions().unwrap().map(|[_, _, fz]| fz).float_max();
    let probe = mol.create_neighbor_probe();
    let lat = mol.get_lattice().ok_or(anyhow!("no lattice"))?;

    let zstep = guess_appropriate_zstep(mol.lattice.as_ref().unwrap(), 0.5);
    debug!("probe step size along z = {}", zstep);
    let mut surface_nodes = std::collections::HashSet::new();
    for _ in 0..n_probes {
        let [x, y] = crate::sample::random_frac_xy();
        let nn = probe_surface_neighgbors_along_z(&probe, lat, x, y, zhigh, zstep, r_cutoff)?;
        for n in nn {
            surface_nodes.insert(n.node);
        }
    }
    let nodes = surface_nodes.into_iter().collect();
    Ok(nodes)
}
// c4191fe3 ends here

// [[file:../spdkit-surface.note::c797229e][c797229e]]
#[test]
fn test_probe_surface_atoms() -> Result<()> {
    gut::cli::setup_logger_for_test();

    let file = "./tests/files/Ni211.cif";
    let mol = Molecule::from_file(file)?;
    let surface_atoms = probe_surface_atoms(&mol, 1.8, 500)?;
    let s = gut::utils::abbreviate_numbers_human_readable(&surface_atoms)?;
    assert_eq!(
        "2,4,8,10,12,16,18,20,24,26,28,32,34,36,40,42,44,48,50,52,56,58,60,64,66,68,72,74,76,80",
        s
    );

    Ok(())
}
// c797229e ends here
