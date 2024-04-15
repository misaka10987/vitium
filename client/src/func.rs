use cursive::Cursive;
use futures::executor::block_on;
use reqwest::{Client, Error, Response};

use crate::init::Data;

pub fn invt(obj:&mut Cursive,clt:&mut Client,mes:String){
    let resp: Result<Response, Error> = block_on(clt.post(obj.user_data::<Data>().unwrap().server_ip.to_string()).body(mes).send());
    obj.add_layer(cursive::views::Dialog::new().title("Inventory"));
}