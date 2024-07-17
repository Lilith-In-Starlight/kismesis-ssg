use actix_web::{App, HttpServer};

pub(super) async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(actix_files::Files::new("/", "./output").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

