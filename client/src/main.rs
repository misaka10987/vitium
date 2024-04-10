use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};
use vitium_client::addtxt;
fn main() {
    let mut siv = Cursive::new();
    let ss = &mut siv;
    addtxt(ss,"Hello,world".to_string());
    addtxt(ss,"aaaa".to_string());
}
