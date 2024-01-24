async fn exit(Json(req): Json<Exit>) -> StatusCode {
    use crate::chara::chara as game_chara;
    use crate::chara::exit;
    if !game_chara().await.contains_key(&req.chara) {
        return StatusCode::NOT_FOUND;
    }
    match game_chara().await.get(&req.chara) {
        Some(c) => {
            if req.token.id != c.player {
                return StatusCode::UNAUTHORIZED;
            }
            if !verify(&req.token).await {
                return StatusCode::FORBIDDEN;
            }
            exit(req.chara).await;
            StatusCode::OK
        }
        None => StatusCode::NOT_FOUND,
    }
}

pub mod root {
    use super::{banned, banned_player, op, player, pswd};
    use tracing::info;
    pub async fn ban(arg: &str) -> i8 {
        let id = arg.trim();
        if !player().await.contains_key(id) {
            println!("  Failure>> player[id=\"{}\"] not found", id);
            return -1;
        }
        if banned(id).await {
            println!("  Failure>> player[id=\"{}\"] is already banned", id);
            return -1;
        }
        player().await.remove(id);
        pswd().await.remove(id);
        banned_player().await.insert(id.to_string());
        println!("  Success>> banned player[id=\"{}\"]", id);
        0
    }
}
