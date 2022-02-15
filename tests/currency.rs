use monobank_api::models;

#[test]
fn get_currency_list() {
    let currencies = models::currency::Currency::get_list();

    assert_ne!(currencies.len(), 0);

    println!("{:?}", currencies.get(0).unwrap());
}