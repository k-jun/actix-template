// use actix_web::{web, App, HttpRequest, HttpServer, Responder};
// use std::sync::Mutex;
use mysql as my;
// テンプレート

// struct AppStateWithCounter {
//     counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
// }
//
// fn _index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
//     *counter += 1; // <- access counter inside MutexGuard
//
//     format!("Request number: {}", counter) // <- response with count
// }

fn main() {
    let pool = my::Pool::new("mysql://root@localhost:3306/sample").unwrap();
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });
    //
    // HttpServer::new(move || { // move counter into the closure
    //     App::new()
    //         .register_data(counter.clone()) // <- register the created data
    //         .route("/", web::get().to(_index))
    // })
    //     .bind("127.0.0.1:8088")
    //     .unwrap()
    //     .run()
    //     .unwrap();
}
