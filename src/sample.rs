// [[file:../spdkit-surface.note::1b57a2f8][1b57a2f8]]
use rand::prelude::*;

/// return random 2D fractional coords in xy surface
///
/// x,y: (0, 1]
pub fn random_frac_xy() -> [f64; 2] {
    let xdist = rand::distributions::Uniform::new(0.0, 1.0);
    let ydist = rand::distributions::Uniform::new(0.0, 1.0);

    let x: f64 = thread_rng().sample(xdist);
    let y: f64 = thread_rng().sample(ydist);
    [x, y]
}

/// return random 3D fractional coords in xyz space
/// x,y: (0, 1]
/// z: (zlow, zhigh)
pub fn random_frac_xyz(zlow: f64, zhigh: f64) -> [f64; 3] {
    let xdist = rand::distributions::Uniform::new(0.0, 1.0);
    let ydist = rand::distributions::Uniform::new(0.0, 1.0);
    let zdist = rand::distributions::Uniform::new(zlow, zhigh);

    let x: f64 = thread_rng().sample(xdist);
    let y: f64 = thread_rng().sample(ydist);
    let z: f64 = thread_rng().sample(zdist);
    [x, y, z]
}
// 1b57a2f8 ends here
