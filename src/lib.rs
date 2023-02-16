// [[file:../spdkit-surface.note::9f37ec34][9f37ec34]]
// #![deny(warnings)]
// 9f37ec34 ends here

// [[file:../spdkit-surface.note::25d94f14][25d94f14]]
mod probe;

mod common {
    pub use gchemol::Molecule;
    pub use gut::prelude::*;
}
// 25d94f14 ends here

// [[file:../spdkit-surface.note::*docs][docs:1]]
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

    // export_doc!(codec);
}
// docs:1 ends here
