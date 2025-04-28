use super::Command;
use crate::Server;
use anyhow::bail;
use basileus::Perm;
use clap::Parser;
use dicexp::{DiceBag, new_simple_rng};

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

/// Evaluate an RPG dice expression.
#[derive(Parser)]
#[command(name = "roll")]
pub struct Roll {
    /// The dice expression.
    dice: String,
}

impl Command for Roll {
    async fn exec(self, _: Server) -> anyhow::Result<String> {
        Ok(roll(&self.dice)?.to_string())
    }

    fn perm_req() -> Perm {
        Perm::from("")
    }
}
