use cursive::views::TextView;
use cursive::CursiveRunnable;
use vitium_client::init::{scr_init, Data};
fn main() {
    let mut mainscr = CursiveRunnable::default();
    let mainss = &mut mainscr;
    mainss.add_fullscreen_layer(TextView::new("hello, dddd\neeeee dseff"));
    scr_init(mainss, "style.toml".to_string());
    mainss.set_global_callback('q', |s| s.quit());
    mainss.set_global_callback('z', |s| {
        s.pop_layer();
    });
    println!("{}", mainss.user_data::<Data>().unwrap().server_ip);
}