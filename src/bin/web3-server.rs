use actix_web::{get, web, App, HttpServer};
use std::sync::Mutex;

const ENDPOINT: &str = "https://mainnet.infura.io/v3/a7f5bdd129cc4688bb70483556e3799f";

struct AppState {
    web3: Mutex<web3::Web3<web3::transports::Http>>,
}

#[get("/balance/{account}")]
async fn balance(web::Path(account): web::Path<String>, data: web::Data<AppState>) -> String {
    let rt = actix_rt::Runtime::new().unwrap();
    let web3 = data.web3.lock().unwrap();
    let balance = rt
        .block_on(web3.eth().balance(account.parse().unwrap(), None))
        .unwrap();
    return format!("{:?} {}\n", account, balance);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        web3: Mutex::new(web3::Web3::new(
            web3::transports::Http::new(ENDPOINT).unwrap(),
        )),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(balance)
    })
    .workers(1)
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
