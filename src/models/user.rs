/*************************************************************************
    "task_server"
    main.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
use serde::{
    Deserialize,
    Serialize
};
use diesel::{
    Queryable,
    Insertable,
    AsChangeset
};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub utc_offset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserPatch {
    pub password: Option<String>,
    pub utc_offset: Option<String>,
}
