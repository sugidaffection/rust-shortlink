use crate::forms::LoginForm;
use crate::models::User;
use crate::Pool;
use diesel::prelude::*;

pub fn login_user(pool: Pool, form: LoginForm) -> Result<User, diesel::result::Error> {
    use crate::database::schema::users::dsl::*;
    let conn = &pool.get().unwrap();
    let result = users.filter(email.eq(&form.email)).first::<User>(conn);

    result
}
