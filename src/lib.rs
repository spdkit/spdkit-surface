// [[file:../spdkit-surface.note::9f37ec34][9f37ec34]]
// #![deny(warnings)]
// 9f37ec34 ends here

// [[file:../spdkit-surface.note::25d94f14][25d94f14]]
pub mod probe;
pub mod sample;

mod layers;
mod common {
    pub use gchemol::prelude::*;
    pub use gchemol::Molecule;
    pub use gut::prelude::*;
}
// 25d94f14 ends here

// [[file:../spdkit-surface.note::af95d88e][af95d88e]]
#[cfg(feature = "adhoc")]
/// Docs for local mods
pub mod docs {
    macro_rules! export_doc {
        ($l:ident) => {
            pub mod $l {
                pub use crate::$l::*;
            }
        };
    }

    export_doc!(probe);
    export_doc!(layers);
    export_doc!(sample);
}
// af95d88e ends here

// [[file:../spdkit-surface.note::027f4836][027f4836]]
pub use crate::layers::fragment_atoms_by_layer;
// 027f4836 ends here
