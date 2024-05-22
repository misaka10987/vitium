use cursive::CursiveRunnable;
use reqwest::Client;
pub fn loadFile(obj: &mut CursiveRunnable) {
    obj.add_layer(cursive::views::Dialog::new().title("Enter file source"));
}
pub fn downloadFile(clt: &mut Client) {}
