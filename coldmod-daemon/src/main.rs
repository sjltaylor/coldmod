use actix_web::dev::Server;
use actix_web::{
    get, http::header::HttpDate, post, rt, web, App, HttpResponse, HttpServer, Responder,
};
use async_channel::{bounded, RecvError, Sender};
use futures::future;
use num_cpus;
use std::str;
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn udp_listener(socket: UdpSocket, snd: Sender<String>) {
    let mut buf = [0; 1024];
    println!("udp listener started");
    loop {
        let (amt, _) = socket.recv_from(&mut buf).await.expect("recv failed");
        let s = str::from_utf8(&buf[..amt]).expect("bad shit");
        match snd.try_send(s.to_owned()) {
            Ok(_) => println!("dispatched: {}", s),
            Err(e) => println!("dispatch error: {} {}", s, e),
        }
    }
}

#[actix_web::main]
async fn main() {
    let http = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/doink", web::to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))
    .expect("http bind failed")
    .workers(num_cpus::get())
    .run();

    let web_handle = rt::spawn(async move {
        match http.await {
            Ok(_) => println!("http server exited"),
            Err(e) => println!("http server exited with an error: {}", e),
        }
    });

    let (snd, rcv) = bounded::<String>(65536);

    rt::spawn(async move {
        udp_listener(
            UdpSocket::bind("127.0.0.1:7777")
                .await
                .expect("could not create UDP socket"),
            snd,
        )
        .await;
    });

    for idx in 0..num_cpus::get() {
        let rx = rcv.clone();
        rt::spawn(async move {
            println!("udp msg rcv id: {}", idx);
            loop {
                match rx.recv().await {
                    Ok(s) => println!("udp msg handler {} recv: {}", idx, s),
                    Err(e) => println!("udp msg handler {} error", e),
                }
                rt::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        });
    }

    // await only the web server - other works don't respect signals
    match web_handle.await {
        Ok(_) => println!("web server exited"),
        Err(e) => println!("web server exited with an error: {}", e),
    }
}
