use cursive::CursiveRunnable;
use vitium_client::config::initctrl;
use vitium_client::init::{scr_init, Data};
fn main() {
    let mut mainscr = CursiveRunnable::default();
    let mainss = &mut mainscr;
    cursive::logger::init();
    log::error!("Something serious probably happened!");
    log::warn!("Or did it?");
    log::debug!("Logger initialized.");
    log::info!("Starting!");initctrl(mainss);
    //mainss.add_fullscreen_layer(TextView::new("hello, dddd\neeeee dseff"));
    scr_init(mainss, "style.toml".to_string());
    //mainss.add_layer(cursive::views::Dialog::new().button("First",|s| s.quit()).button("Second",|s| s.quit()));
    mainss.run();
    println!("{}", mainss.user_data::<Data>().unwrap().server_ip);
}
