// [[file:../spdkit-surface.note::9f37ec34][9f37ec34]]
// #![deny(warnings)]
// 9f37ec34 ends here

// [[file:../spdkit-surface.note::25d94f14][25d94f14]]
pub mod probe;
mod sample;

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
}
// af95d88e ends here
