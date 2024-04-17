use cursive::CursiveRunnable;
use vitium_client::config::initctrl;
use vitium_client::init::{scr_init, Data};
fn main() {
    let mut mainscr = CursiveRunnable::default();
    let mainss = &mut mainscr;
    initctrl(mainss);
    //mainss.add_fullscreen_layer(TextView::new("hello, dddd\neeeee dseff"));
    scr_init(mainss, "style.toml".to_string());
    mainss.run();
    println!("{}", mainss.user_data::<Data>().unwrap().server_ip);
}
