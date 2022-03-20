use monobank_api::models::statement;

#[test]
fn list_of_statement() {
    dotenv::dotenv().ok();
    let token  = std::env::var("MONOBANK_TOKEN").unwrap();
    let account = "0";
    let from = 1645422807;
    let to = chrono::Utc::now().timestamp();

    let statements = statement::Statement::get_list(token.as_str(), account, from, to).unwrap();

    println!("{:?}", statements)
}

#[test]
fn parse_date() {
    let temp = "1645487061";

    let date = chrono::NaiveDateTime::from_timestamp(temp.parse().unwrap(), 0);
    print!("{}", date)
}