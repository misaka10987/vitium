use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};
pub fn addtxt(Obj:&mut Cursive,mes:String){
    Obj.add_layer(TextView::new(mes));
    Obj.run();
}