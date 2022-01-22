#[path = "../../../test_utils/mod.rs"]
mod test_utils;

use insta::*;
use serde_json::json;
use test_context::test_context;

use test_utils::*;

#[test_context(TestContext)]
#[actix_rt::test]
async fn ユーザを追加することができること(ctx: &TestContext) {
    let (app, _) = create_service().await;
    let req_body = json!({
       "query": "mutation Test {
          addUser(input:{name: \"John\"}) {
            ... on User {
              id
              name
            }
            ... on ValidationErrors {
              errors {
                field
                message
              }
            }
          }
        }"
    });

    let res = graphql_req(&app, req_body).await;
    assert_json_snapshot!(res);

    let req_body = json!({
       "query": "query ConfirmAddUser($id: Int!) {
          user(id: $id) {
            id
            name
          }
        }",
       "variables": { "id": res["data"]["addUser"]["id"] }
    });

    assert_json_snapshot!(graphql_req(&app, req_body).await);
}

#[test_context(TestContext)]
#[actix_rt::test]
async fn 空のユーザ名の場合はバリデーションエラーをレスポンスすること(
    ctx: &TestContext,
) {
    let (app, _) = create_service().await;
    let req_body = json!({
       "query": "mutation Test {
          addUser(input:{name: \"\"}) {
            ... on User {
              id
              name
            }
            ... on ValidationErrors {
              errors {
                field
                message
              }
            }
          }
        }"
    });

    assert_json_snapshot!(graphql_req(&app, req_body).await);
}
