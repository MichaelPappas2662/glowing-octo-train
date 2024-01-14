use diesel::Insertable;

use serde::Deserialize;
use validator::Validate;

use crate::schema::users;

#[derive(Insertable, Validate, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}
