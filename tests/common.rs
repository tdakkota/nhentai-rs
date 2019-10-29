use nhentai::Client;

#[test]
fn it_works() {
    let client = Client::new();
    let res = client.get_random().unwrap();
    res.iter_resolved_page_urls().for_each(|url| {
        dbg!(url);
    });
}

#[test]
fn get_title() {
    let client = Client::new();
    let res = client.get_random().unwrap();
    dbg!(res.get_english_title());
}
