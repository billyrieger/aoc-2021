pub mod prelude {
    pub use crate::{Error, Result};
    pub use itertools::Itertools;

    pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
    pub type HashSet<K> = rustc_hash::FxHashSet<K>;
}

pub type Result<T> = anyhow::Result<T>;
pub type Point2D = (i32, i32);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error")]
    Parse,
    #[error("Logic error")]
    Logic,
}

pub fn parse_point(point: &str) -> Result<Point2D> {
    let (x, y) = point.split_once(',').ok_or(Error::Parse)?;
    Ok((x.parse()?, y.parse()?))
}
