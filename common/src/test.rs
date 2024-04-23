pub const DEBUG_DESCR: &str = "If you see this in game, it is a bug.";

pub trait Example
where
    Self: Clone + Sized,
{
    fn examples() -> Vec<Self>;
    fn example() -> Self {
        Self::examples()[0].clone()
    }
}
