pub mod prelude {
    pub use crate::{AocError, Result};
    pub use itertools::{iproduct, Itertools};
    pub use nalgebra as na;
    pub type BitVec = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
    pub type BitSlice = bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;
    pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
    pub type HashSet<K> = rustc_hash::FxHashSet<K>;
}

pub type Result<T> = anyhow::Result<T>;
pub type Point2D = (i32, i32);

#[derive(Debug, thiserror::Error)]
pub enum AocError {
    #[error("Parse error")]
    Parse,
    #[error("Logic error")]
    Logic,
}

pub fn parse_point(point: &str) -> Result<Point2D> {
    let (x, y) = point.split_once(',').ok_or(AocError::Parse)?;
    Ok((x.parse()?, y.parse()?))
}
