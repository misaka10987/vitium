use cursive::view::Position;
use cursive::views::{LayerPosition, TextView};
use cursive::{Cursive, CursiveRunnable};
use reqwest::{Client, Error};
pub mod chatbox;
pub mod config;
pub mod func;
pub mod init;
pub mod map;

pub struct Ip(pub String);

pub fn addtxt(obj: &mut CursiveRunnable, mes: String) {
    obj.add_layer(TextView::new(mes));
}
pub fn move_top(c: &mut Cursive, x_in: isize, y_in: isize) {
    let s = c.screen_mut();
    let l = LayerPosition::FromFront(0);
    let pos = s
        .layer_offset(LayerPosition::FromFront(0))
        .unwrap()
        .saturating_add((x_in, y_in));
    let p = Position::absolute(pos);
    s.reposition_layer(l, p);
}
pub async fn getta(
    obj: &mut CursiveRunnable,
    clt: &mut Client,
    mes: String,
) -> Result<String, Error> {
    clt.get(&obj.user_data::<Ip>().unwrap().0)
        .body(mes)
        .send()
        .await?
        .text()
        .await
}
