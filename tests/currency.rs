use monobank_api::models::currency::Currency;

#[test]
fn get_currency_list() {
    let currencies = Currency::get_list().unwrap();

    assert_ne!(currencies.len(), 0);

    println!("{:?}", currencies.get(0).unwrap());
}