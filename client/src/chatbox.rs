use vitium_client::Scr;
impl Chatbox for Scr{
    fn inpt(mes:String) -> String{
        siv.add_layer(
            cursive::views::Dialog::new()
            .title(mes)
            .content(cursive::views::EditView::new().on_edit(|s,content,_| {s.set_window_title(content);}))
            .button("Quit",|s| s.quit()),
        )
    }
}