use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

const ENDPOINT: &str = "https://mainnet.infura.io/v3/a7f5bdd129cc4688bb70483556e3799f";

struct AppState {
    web3: Mutex<web3::Web3<web3::transports::Http>>,
}

async fn balance(data: web::Data<AppState>) -> String {
    let rt = actix_rt::Runtime::new().unwrap();
    let web3 = data.web3.lock().unwrap();
    eprintln!("Calling accounts.");
    let mut accounts = rt.block_on(web3.eth().accounts()).unwrap_or(Vec::new());
    eprintln!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    eprintln!("Calling balance.");
    let mut ret = String::new();
    for account in accounts {
        let balance = rt.block_on(web3.eth().balance(account, None)).unwrap();
        ret.push_str(&format!("{:?} {}\n", account, balance));
    }
    return ret;
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
            .route("/", web::get().to(balance))
    })
    .workers(1)
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
