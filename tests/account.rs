use monobank_api::models::account;

#[test]
fn account_info() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let token  = std::env::var("MONOBANK_TOKEN").unwrap();
    let account = account::Account::get_info(token.as_str())?;

    print!("{:?}", account);

    let main = account.main_card().unwrap();

    assert_eq!(main.id(), "UckaonuaE2K4fD8jRlahvA");

    Ok(())
}
