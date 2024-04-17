use cursive::{views::TextView, Cursive};
use reqwest::Client;

use crate::init::Data;

pub async fn invt(obj: &mut Cursive, clt: &mut Client, mes: String) {
    obj.add_layer(cursive::views::Dialog::new().content(TextView::new("Connecting")));
    match clt
        .post(obj.user_data::<Data>().unwrap().server_ip.to_string())
        .body(mes)
        .send()
        .await
    {
        Ok(resp) => match resp.text().await {
            Ok(strng) => {
                obj.add_layer(
                    cursive::views::Dialog::new()
                        .title("resp")
                        .content(TextView::new(strng))
                );
            }
            Err(_) => {
                obj.add_layer(cursive::views::Dialog::new().title("Failed to Reslove"))
            }
        },
        Err(_) => {
            obj.add_layer(cursive::views::Dialog::new().title("Connection Failed"))
        }
    }
}
