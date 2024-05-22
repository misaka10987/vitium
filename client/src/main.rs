use std::fs;

use cursive::CursiveRunnable;
use reqwest::Client;
use vitium_client::{
    addtxt,
    config::theme_adj,
    init::{initctrl, theme_init},
};

fn main() {
    let mut mainscr = CursiveRunnable::default();
    let mainss = &mut mainscr;
    let mut clt = Client::new();
    let mainclt = &mut clt;
    initctrl(mainss);
    theme_init(mainss, "style.toml");
    theme_adj(mainss);
    //mainss.add_fullscreen_layer(TextView::new("hello, dddd\neeeee dseff"));
    //internet_init(mainss);
    //fs::write("text.txt", "FROM RUST PROGRAM").unwrap();
    //mainss.add_layer(cursive::views::Dialog::new().button("First",|s| s.quit()).button("Second",|s| s.quit()));
    mainss.run();
    //println!("{}", mainss.user_data::<Data>().unwrap().server_ip);
}
