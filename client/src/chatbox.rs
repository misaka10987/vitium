use cursive::{views, Cursive};

use crate::init::internett_init;

pub fn menjubar(obj: &mut Cursive) {
    obj.add_layer(
        views::Dialog::new()
            .title("Menu")
            .button("Internet Config", |s| {s.pop_layer();internett_init(s);})
            .button("Exit", |s| s.quit())
            .button("KillLayer",|s| {s.pop_layer();s.pop_layer();}),
    )
}
