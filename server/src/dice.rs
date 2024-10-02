use anyhow::bail;
use dicexp::{new_simple_rng, DiceBag};

pub fn roll(dice: &str) -> anyhow::Result<i32> {
    // `dicexp` does not perform checks for 0-sided dice.
    if dice.contains("d0") {
        bail!("0-sided dice are not supported");
    }
    match DiceBag::new(new_simple_rng()).eval(dice) {
        Ok(n) => Ok(n.total as i32),
        Err(e) => bail!("{e}"),
    }
}
