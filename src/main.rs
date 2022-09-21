use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, get, web::Bytes, App, HttpResponse, HttpServer, Responder};
use async_stream::try_stream;

#[get("/get-broken")]
async fn get_broken(session: Session) -> impl Responder {
    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from(format!(
                "{:?}",
                session.get::<String>("session").unwrap()
            )))
            .await;
        Ok::<(), std::io::Error>(())
    });
    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(stream)
}

#[get("/get-working")]
async fn get_working(session: Session) -> impl Responder {
    let body = Bytes::from(format!("{:?}", session.get::<String>("session").unwrap()));
    HttpResponse::Ok().content_type("text/plain").body(body)
}

#[get("/set-broken")]
async fn set_broken(session: Session) -> impl Responder {
    let stream = try_stream(move |mut stream| async move {
        stream
            .yield_item(Bytes::from(format!(
                "{:?}",
                session.insert("session", "broken".to_string()).unwrap()
            )))
            .await;
        Ok::<(), std::io::Error>(())
    });
    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(stream)
}

#[get("/set-working")]
async fn set_working(session: Session) -> impl Responder {
    let body = Bytes::from(format!(
        "{:?}",
        session.insert("session", "working".to_string()).unwrap()
    ));
    HttpResponse::Ok().content_type("text/plain").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("1. http://127.0.0.1:8080/set-working");
    println!("2. http://127.0.0.1:8080/get-working");
    println!("3. http://127.0.0.1:8080/get-broken");
    println!("4. http://127.0.0.1:8080/set-broken");
    println!("5. http://127.0.0.1:8080/get-working");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .build(),
            )
            .service(get_broken)
            .service(set_broken)
            .service(get_working)
            .service(set_working)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
