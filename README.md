# rust-juniper-playground

# setup
1. .env を作成する
2. docker-compose.override.yml を作成し graphql サーバの公開ポートを設定する
  ```yml
  version: "3.9"
   
  services:
   graphql:
     ports:
        - "{任意のポート}:8088"
  ```
3. 起動
  ```bash
  docker-compose up -d
  ```
4. database migration
  ```bash
  docker-compose exec graphql sqlx migrate run
  ```
5. go to graphql playground
  ```bash
  open http://localhost:{設定したポート}/graphql
  ```
