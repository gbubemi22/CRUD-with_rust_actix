use actix_web::web;

use super::game::{get_games, create_game, get_game_by_id, delete_game, update_game};

pub fn config(conf: &mut web::ServiceConfig) {
     let scope = web::scope("/api/games")
     .service(get_games)
     .service(create_game)
     .service(get_game_by_id)
     .service(delete_game)
     .service(update_game);

     conf.service(scope);
}