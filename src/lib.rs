pub use itertools::Itertools;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Point2D = (i32, i32);

#[derive(Debug, thiserror::Error)]
#[error("Some expected, None found")]
pub struct NoneError;

pub fn parse_point(point: &str) -> Result<Point2D> {
    let (x, y) = point.split_once(',').ok_or(NoneError)?;
    Ok((x.parse()?, y.parse()?))
}
