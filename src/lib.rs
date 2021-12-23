pub mod prelude {
    pub use itertools::Itertools;
    pub use crate::Error;
    pub use crate::Result;

    pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
    pub type HashSet<K> = rustc_hash::FxHashSet<K>;
}

pub type Result<T> = anyhow::Result<T>;
pub type Point2D = (i32, i32);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse input")]
    Parse,
    #[error("Could not parse input")]
    Logic,
}

pub fn parse_point(point: &str) -> Result<Point2D> {
    let (x, y) = point.split_once(',').ok_or(Error::Parse)?;
    Ok((x.parse()?, y.parse()?))
}
