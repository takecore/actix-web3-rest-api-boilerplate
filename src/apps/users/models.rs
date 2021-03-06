use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::db::connection;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub company_id: i32, // TODO relation
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub company_id: i32,
    pub name: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
}

impl User {
    pub fn all() -> Result<Vec<Self>, Error> {
        use crate::schema::users::dsl::users;
        let items = users.load::<Self>(&connection::connect())?;
        Ok(items)
    }

    pub fn id(id: i32) -> Result<Self, Error> {
        use crate::schema::users::dsl::users;
        let item = users.find(id).get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn id_with_company_id(id: i32, company_id: i32) -> Result<Self, Error> {
        use crate::schema::users::dsl;
        let item = dsl::users
            .find(id)
            .filter(dsl::company_id.eq(company_id))
            .get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn create(create: &CreateUser) -> Result<Self, Error> {
        use crate::schema::users::dsl::users;
        let item = diesel::insert_into(users)
            .values(create)
            .get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn update(&self, update: &UpdateUser) -> Result<Self, Error> {
        let item = diesel::update(self)
            .set(update)
            .get_result::<Self>(&connection::connect())?;
        Ok(item)
    }

    pub fn delete(&self) -> Result<usize, Error> {
        let count = diesel::delete(self).execute(&connection::connect())?;
        Ok(count)
    }
}
