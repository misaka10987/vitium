use cursive::CursiveRunnable;
use cursive::views::TextView;
pub mod init;
pub struct Scr{
    pub id:usize,
}
// impl Scr{
//     pub fn 
// }
pub fn addtxt(obj: &mut CursiveRunnable, mes: String) {
    obj.add_layer(TextView::new(mes));
}
// pub fn setquit(obj: &mut CursiveRunnable, key: char) {
//     obj.add_global_callback(key, |s| s.quit());
// }
// pub fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
//     let s = c.screen_mut();
//     let l = LayerPosition::FromFront(0);
//     let pos = s
//         .layer_offset(LayerPosition::FromFront(0))
//         .unwrap()
//         .saturating_add((x_in, y_in));
//     let p = Position::absolute(pos);
//     s.reposition_layer(l, p);
// }

