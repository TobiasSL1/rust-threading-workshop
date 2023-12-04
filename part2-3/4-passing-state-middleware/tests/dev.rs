

use http::StatusCode;
use reqwest::{ blocking::*, header} ;

#[test] fn simple_test()  {
    let client = Client::new();
    let response = client
    .get("http://localhost:3000/")
    .header(header::AUTHORIZATION, "Bearer xyz")
    .send()
    .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
