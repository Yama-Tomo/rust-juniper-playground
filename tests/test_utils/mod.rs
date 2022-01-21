use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, App};
use dotenv::dotenv;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use sqlx_core::migrate::MigrateDatabase;
use sqlx_core::mysql::MySql;
use std::process::Command;
use std::sync::Arc;
use std::{env, panic, str};
use test_context::AsyncTestContext;

use rust_juniper_playground::{configure, create_db_connection};

pub async fn setup_database() -> String {
    dotenv().ok();

    let database_name_suffix = "%PARALLELISM_SUFFIX%";
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    if !database_url.contains(&database_name_suffix) {
        panic!("not contains `{}` in database name", database_name_suffix);
    }

    let suffix = thread_id::get().to_string();
    let database_url = database_url.replace(&database_name_suffix, &suffix);

    MySql::create_database(&database_url)
        .await
        .expect("create test database");

    // DBマイグレーションをプログラム内で実行しようとするとライフタイム絡みでコンパイルがとおらず、以下のworkaroundも動かないので
    // コマンドを直接実行してマイグレーションする
    // https://github.com/launchbadge/sqlx/issues/954#issuecomment-767080149
    let res = Command::new("sqlx")
        .env("DATABASE_URL", &database_url)
        .args(["migrate", "run"])
        .output()
        .unwrap();

    if !res.status.success() {
        panic!("{}", std::str::from_utf8(&res.stderr).unwrap());
    }

    // FIXME: https://github.com/launchbadge/sqlx/issues/954 が修正されたら以下のコードでマイグレーションするように修正
    // let migrator = Migrator::new(Path::new("migrations")).await.unwrap();
    // let mut conn = MySqlConnectOptions::from_str(&database_url)
    //     .expect("correct TEST_DATABASE_URL")
    //     .connect()
    //     .await
    //     .expect("connection test database");
    //
    // migrator.run(&mut conn).await;
    // conn.close();

    // プロダクションコード内で参照している環境変数にテスト用のDBのURLをセットする
    env::set_var("DATABASE_URL", database_url.clone());

    database_url
}

pub struct TestContext {
    test_database_url: String,
}
#[async_trait::async_trait]
impl AsyncTestContext for TestContext {
    async fn setup() -> TestContext {
        let test_database_url = setup_database().await;

        Self { test_database_url }
    }

    async fn teardown(self) {
        let _drop_result = MySql::drop_database(&self.test_database_url).await;
    }
}

// NOTE: TestContextのsetupで生成したかったが implキーワードの型は構造体に保持できないため単に関数定義することとした
pub async fn create_service() -> (
    impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    DatabaseConnection,
) {
    let conn = create_db_connection().await;
    let conn = Arc::new(conn);

    let app = test::init_service(
        App::new().configure(|cfg| configure(cfg, Some(Arc::clone(&conn).as_ref().to_owned()))),
    )
    .await;

    (app, Arc::clone(&conn).as_ref().to_owned())
}

pub async fn graphql_req(
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    req_body: Value,
) -> Value {
    let req = test::TestRequest::post()
        .append_header(("content-type", "application/json"))
        .set_payload(req_body.to_string())
        .uri("/graphql")
        .to_request();

    let resp = test::call_service(&app, req).await;

    let result = test::read_body(resp).await;
    let result = str::from_utf8(&result).unwrap();
    let result: Value = serde_json::from_str(result).unwrap();

    result
}
