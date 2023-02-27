use crate::models::models::{Post, Session};
use crate::AppState;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use crate::api::helpers::get_user_id_from_jwt_token;


pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(user_posts);
    cfg.service(scope);
}


#[get("/posts")]
pub async fn user_posts(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let token_header = req.headers().get("Authorization");
    let token = match token_header {
        Some(header_value) => {
            let token_str = header_value.to_str().unwrap();
            if token_str.starts_with("Bearer ") {
                Some(&token_str[8..])
            } else {
                None
            }
        }
        None => { return HttpResponse::NotFound().json(
            "Authentication credentials were not provided.")}
    };

    let claims = get_user_id_from_jwt_token(token.unwrap()).await;
    let user_id = match claims {
        Ok(user_id) => user_id,
        Err(e) => { return HttpResponse::BadRequest().json(e) }
    };

    let posts = Post::get_all_posts_by_author(
        state, user_id as i32).await;
    match posts {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json("Bad request"),
    }
}
