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

#[test]
#[ignore]
fn it_works_with_proxy() {
    let proxy_addr = std::env::var("PROXY").unwrap();
    let proxy = reqwest::Proxy::all(proxy_addr.as_str()).unwrap();
    let http_client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let client = Client::new_with_client(http_client);
    let res = client.get_random().unwrap();
    res.iter_resolved_page_urls().for_each(|url| {
        dbg!(url);
    });
}

#[test]
#[ignore]
fn get_title_with_proxy() {
    let proxy_addr = std::env::var("PROXY").unwrap();
    let proxy = reqwest::Proxy::all(proxy_addr.as_str()).unwrap();
    let http_client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let client = Client::new_with_client(http_client);
    let res = client.get_random().unwrap();
    dbg!(res.get_english_title());
}
