#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new(
        "https://mainnet.infura.io/v3/a7f5bdd129cc4688bb70483556e3799f",
    )?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}
