pub use itertools::{iproduct, Itertools};
pub use nalgebra as na;

pub type BitVec = bitvec::vec::BitVec<bitvec::order::Msb0, u8>;
pub type BitSlice = bitvec::slice::BitSlice<bitvec::order::Msb0, u8>;
pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type HashSet<K> = rustc_hash::FxHashSet<K>;
pub type IVec2 = na::Vector2<i32>;
pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, thiserror::Error)]
pub enum AocError {
    #[error("Parse error")]
    Parse,
    #[error("Logic error")]
    Logic,
}

pub fn parse_point2(input: &str) -> Result<IVec2> {
    let (x, y) = input
        .split(',')
        .map(str::parse::<i32>)
        .collect_tuple()
        .ok_or(AocError::Parse)?;
    Ok([x?, y?].into())
}
