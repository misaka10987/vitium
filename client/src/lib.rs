use cursive::views::TextView;
use cursive::CursiveRunnable;
pub fn init(obj:&mut CursiveRunnable,rd:String) -> bool{
    let a = obj.load_theme_file(rd);
    if a.is_err() {
        return false;
    }else{
        a.unwrap();
        return true;
    }
}
pub fn addtxt(obj:&mut CursiveRunnable,mes:String){
    obj.add_layer(TextView::new(mes));
}
pub fn setquit(obj:&mut CursiveRunnable,key:char){
    obj.add_global_callback(key,|s| s.quit());
}