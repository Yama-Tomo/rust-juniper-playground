use rust_juniper_playground::configure;

#[actix_rt::test]
async fn test_index_get() {
    use actix_web::{test, App};
    use std::str;

    let app = test::init_service(App::new().configure(|cfg| configure(cfg, None))).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    let result = test::read_body(resp).await;
    let result = str::from_utf8(&result).unwrap();

    assert_eq!(result, "Hello world!");
}
