use http;
use serde_json::Value;
use std::{str, thread, time};

fn main() {
    env_logger::init();
    let url = "https://catfact.ninja/fact".to_string();
    loop {
        let req = http::request::Builder::new()
            .method(http::Method::GET)
            .uri(&url)
            .header("Content-Type", "text/plain");
        let req = req.body(None).unwrap();

        log::debug!("Request: {:?}", req);

        // send request using the experimental bindings for http on wasi
        let mut res = wasi_experimental_http::request(req).expect("cannot make request");

        let response_body = res.body_read_all().unwrap();
        let response_text = str::from_utf8(&response_body).unwrap().to_string();
        let headers = res.headers_get_all().unwrap();

        log::debug!("{}", res.status_code);
        log::debug!("Response: {:?} {:?}", headers, response_text);

        // parse the response to json
        let cat_fact: Value = serde_json::from_str(&response_text).unwrap();

        log::info!("Cat Fact: {}", cat_fact["fact"].as_str().unwrap());

        thread::sleep(time::Duration::new(60, 0));
    }
}
