use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use reqwest::header::{USER_AGENT, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use base64::Engine;

#[derive(Clone)]
struct AppState {
    i: u32,
    s: String,
    b64: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            format!(
                "Hello world!🥳     <br>{}     <br><img src=\"data:image/jpeg;base64, {}\"/>       <br>{}",
                data.i,
                data.b64,
                data.s
            )
        )
}

#[get("/api")]
async fn api(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(
        data.i.to_string()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let scryfall_endpoint: String = String::from("https://api.scryfall.com/cards/random");
    let cargo_name = env!("CARGO_PKG_NAME");
    let cargo_version = env!("CARGO_PKG_VERSION");
    let user_agent: String = format!("{}/{}", cargo_name, cargo_version);

    let client = Client::new(); 

    let mut small_url: Option<String> = None;
    let mut small_base64: Option<String> = None;
    while small_base64.is_none() {
        let json: Value = client
            .get(&scryfall_endpoint)
            .header(USER_AGENT, &user_agent)
            .header(ACCEPT, "*/*")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        if let Some(small_str) = json
                .get("image_uris")
                .and_then(|uris| uris.get("small"))
                .and_then(|small| small.as_str()) {
                small_url = Some(small_str.to_string());
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }
        
        let bytes = client
            .get(small_url.clone().unwrap())
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        small_base64 = Some(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }


    let state = AppState {
        i:rand::thread_rng().gen_range(10000..=99999),
        s:small_url.unwrap(),
        b64:small_base64.unwrap(),
    };
    println!("About to run on 0.0.0.0:5100  ~  {}", state.i);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(api)
    })
    .bind(("0.0.0.0", 5100))?
    .run()
    .await
}
