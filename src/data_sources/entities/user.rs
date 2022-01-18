use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use std::sync::Arc;

use super::errors::{ValidationError, ValidationErrors};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
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
        let record = self::Entity::find_by_id(id).one(conn).await;
        match record {
            Ok(record) => {
                let record = Arc::new(record);
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
            Err(e) => Err(e),
        }
    }

    // TODO: コード生成する仕組みを作りたい
    pub fn name(mut self, val: String) -> Self {
        self.model.name = Set(val);
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

    fn valid_name(self, errors: &mut ValidationErrors) -> Self {
        if self.model.name.is_set() && self.model.name.as_ref().is_empty() {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "given empty value".to_string(),
            })
        }

        self
    }

    pub fn build(mut self) -> Result<Result<ActiveModel, ValidationErrors>, DbErr> {
        let mut errors: ValidationErrors = Vec::new();

        self = self.valid_id(&mut errors);
        self = self.valid_name(&mut errors);

        // ほかにバリデーションする場合はここに実装

        if errors.is_empty() {
            Ok(Ok(self.model))
        } else {
            Ok(Err(errors))
        }
    }
}
