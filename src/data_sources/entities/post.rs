use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use std::sync::Arc;

use super::errors::{ValidationError, ValidationErrors};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub struct ModelBuilder {
    model: ActiveModel,
    exists_model: Option<Model>,
    primary_keys: Option<i32>,
}
impl ModelBuilder {
    pub fn new() -> Self {
        ModelBuilder {
            model: ActiveModel {
                ..Default::default()
            },
            exists_model: None,
            primary_keys: None,
        }
    }

    pub async fn from_exists_data(conn: &DatabaseConnection, id: i32) -> Result<Self, DbErr> {
        let record = Arc::new(self::Entity::find_by_id(id).one(conn).await?);

        Ok(ModelBuilder {
            model: match Arc::clone(&record).as_ref().to_owned() {
                Some(record) => record.into(),
                None => ActiveModel {
                    ..Default::default()
                },
            },
            exists_model: Arc::clone(&record).as_ref().to_owned(),
            primary_keys: Some(id),
        })
    }

    // TODO: コード生成する仕組みを作りたい
    pub fn title(mut self, val: String) -> Self {
        self.model.title = Set(val);
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.model.user_id = Set(val);
        self
    }

    pub fn created_at(mut self, val: DateTime) -> Self {
        self.model.created_at = Set(val);
        self
    }

    pub fn updated_at(mut self, val: DateTime) -> Self {
        self.model.updated_at = Set(val);
        self
    }

    fn valid_id(self, errors: &mut ValidationErrors) -> Self {
        if let (Some(pk), None) = (self.primary_keys, &self.exists_model) {
            errors.push(ValidationError {
                field: "id".to_string(),
                message: format!("data not found. id = {}", pk),
            })
        }

        self
    }

    fn valid_title(self, errors: &mut ValidationErrors) -> Self {
        if self.model.title.is_set() && self.model.title.as_ref().is_empty() {
            errors.push(ValidationError {
                field: "title".to_string(),
                message: "given empty value".to_string(),
            })
        }

        self
    }

    async fn valid_user_id(
        self,
        conn: &DatabaseConnection,
        errors: &mut ValidationErrors,
        db_err: &mut Option<DbErr>,
    ) -> Self {
        if self.model.user_id.is_set() {
            let user_id = self.model.user_id.as_ref();
            let record = self::Entity::find()
                .filter(self::Column::UserId.eq(*user_id))
                .one(conn)
                .await;

            match record {
                Ok(record) if record.is_none() => errors.push(ValidationError {
                    field: "user_id".to_string(),
                    message: format!("user not found. user_id = {}", user_id),
                }),
                Err(e) => *db_err = Some(e),
                _ => {}
            }
        }

        self
    }

    pub async fn build(
        mut self,
        conn: &DatabaseConnection,
    ) -> Result<Result<ActiveModel, ValidationErrors>, DbErr> {
        let mut errors: ValidationErrors = Vec::new();
        let mut db_err: Option<DbErr> = None;

        self = self.valid_id(&mut errors);
        self = self.valid_title(&mut errors);
        self = self.valid_user_id(conn, &mut errors, &mut db_err).await;
        if let Some(db_err) = db_err {
            return Err(db_err);
        }

        // ほかにバリデーションする場合はここに実装

        if errors.is_empty() {
            Ok(Ok(self.model))
        } else {
            Ok(Err(errors))
        }
    }
}
