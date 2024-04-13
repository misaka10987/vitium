use cursive::CursiveRunnable;
use vitium_client::init::scr_init;
use vitium_client::Scr;
fn main(){
    let mut mainscr = CursiveRunnable::default();
    let mainss = &mut mainscr;
    scr_init(mainss,"style.toml".to_string());
    let mscr = Scr{id:mainss.add_screen()};
    println!("{}",mscr.id);
    // mainss.add_fullscreen_layer(TextView::new("hello, dddd\neeeee dseff"));
    // mainss.add_screen();
    mainss.set_global_callback('q',|s| s.quit());
    mainss.set_global_callback('z',|s| {s.pop_layer();});
    mainss.run();
}