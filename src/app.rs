use actix::prelude::*;
use actix_web::*;
use actix_web::{middleware::Logger, App, http::Method, HttpRequest, HttpResponse, State, Responder};
use models::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}
//(signup_invitation, state): (Json<CreateInvitation>, State<AppState>)
//fn index2((_req, _state): (&HttpRequest, State<AppState>)) -> HttpResponse {//HttpResponse {
//    let s = "Hello";
//    HttpResponse::Ok().content_type("text/html").body(s)
//}

// /// simple handle
// fn index(
//     //(state, req): (State<AppState>, &HttpRequest),
//     state: &HttpRequest<<AppState>>
//     ) -> HttpResponse {
//     println!("{:?}", req);

//     HttpResponse::Ok().content_type("text/html").body("Hello")
// }


// /// creates and returns the app after mounting all routes/resources
// pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
//     App::with_state(AppState { db })
//         .middleware(Logger::default())

//         // test
//         .resource("/hello/{id}", |r| {
//             r.method(Method::GET).with(index);
//         })
// }