use actix_web::{web, App, HttpServer};
mod infrastructure;
mod interface;
use infrastructure::routers;
use listenfd::ListenFd;
// use mysql as my;

fn main() {
    // let pool = my::Pool::new("mysql://root@localhost:3306/sample").unwrap();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        // move counter into the closure
        App::new()
            .configure(routers::todo_routers::config)
            .configure(routers::index_routers::config)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };

    server.run().unwrap();
}
