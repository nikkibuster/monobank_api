use monobank_api::models::account;

#[test]
fn account_info() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let token  = "uUhluDpbTLpQMRmGkRIOVsLl_rGwKKbjWAy7Tz4u-2mQ";
    let account = account::Account::get_info(token)?;

    println!("{}", account.white_card().unwrap().balance());

    Ok(())
}
