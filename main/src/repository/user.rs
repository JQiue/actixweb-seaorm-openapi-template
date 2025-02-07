use crate::entity::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
  pub async fn get_users(&self) -> Result<Vec<UserModel>, DbErr> {
    UserEntity::find().all(self.db).await
  }
  pub async fn get_user_by_id(&self, id: u32) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find_by_id(id).one(self.db).await
  }
  pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find()
      .filter(UserColumn::Email.eq(email))
      .one(self.db)
      .await
  }
  pub async fn is_first_user(&self) -> Result<bool, DbErr> {
    let users = UserEntity::find().all(self.db).await?;
    Ok(users.is_empty())
  }
  pub async fn is_admin_user(&self, email: &str) -> Result<bool, DbErr> {
    let user = UserEntity::find()
      .filter(UserColumn::Email.eq(email))
      .one(self.db)
      .await?;
    match user {
      Some(user) => Ok(user.r#type == "admin"),
      None => Ok(false),
    }
  }
  pub async fn is_root_user(&self, user_id: u32) -> Result<bool, DbErr> {
    let user = UserEntity::find_by_id(user_id).one(self.db).await?;
    match user {
      Some(user) => Ok(user.r#type == "root"),
      None => Ok(false),
    }
  }
  pub async fn create_user(&self, user: UserActiveModel) -> Result<UserModel, DbErr> {
    user.insert(self.db).await
  }
  pub async fn update_user(&self, user: UserActiveModel) -> Result<UserModel, DbErr> {
    user.update(self.db).await
  }
  pub async fn has_user(&self, email: &str) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find()
      .filter(UserColumn::Email.eq(email))
      .one(self.db)
      .await
  }
}
