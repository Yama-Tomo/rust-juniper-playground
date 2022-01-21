#[path = "../test_utils/mod.rs"]
mod test_utils;

use insta::*;
use serde_json::json;
use test_context::test_context;

use test_utils::*;

#[test_context(TestContext)]
#[actix_rt::test]
async fn すべてのユーザの一覧をレスポンスすること(ctx: &TestContext) {
    let (app, _) = create_service().await;
    let req_body = json!({
       "query": "query Test {
          users {
            id
            name
            posts {
              id
              title
            }
          }
        }"
    });

    assert_json_snapshot!(graphql_req(&app, req_body).await);
}

#[test_context(TestContext)]
#[actix_rt::test]
async fn ユーザと紐づく投稿をレスポンスすること(ctx: &TestContext) {
    let (app, _) = create_service().await;
    let req_body = json!({
       "query": "query Test {
          user(id: 1) {
            id
            name
            posts {
              id
              title
            }
          }
        }"
    });

    assert_json_snapshot!(graphql_req(&app, req_body).await);
}
