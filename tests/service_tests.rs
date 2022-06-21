#[cfg(test)]
mod service_tests {

    use diesel::{delete, insert_into, prelude::*};
    use lazy_static::lazy_static;
    use serial_test::serial;
    use shortlink::{
        create_pool,
        forms::{LoginForm, RegisterForm},
        hash_password,
        models::{NewUser, User},
        PConn, Pool,
    };

    lazy_static! {
        static ref POOL: Pool = {
            dotenv::dotenv().ok();
            create_pool(None)
        };
        static ref EMAIL: String = String::from("test_email@mail.com");
        static ref USERNAME: String = String::from("test_email@mail.com");
        static ref PASSWORD: secrecy::Secret<String> =
            secrecy::Secret::new(String::from("test123"));
        static ref NEW_USER: NewUser = NewUser {
            email: EMAIL.to_owned(),
            username: USERNAME.to_owned(),
            password_hash: hash_password(PASSWORD.to_owned()).unwrap(),
        };
        static ref REGISTER_FORM: RegisterForm = RegisterForm {
            email: EMAIL.clone(),
            password: PASSWORD.clone(),
            username: USERNAME.clone(),
        };
        static ref LOGIN_FORM: LoginForm = LoginForm {
            email: EMAIL.clone(),
            password: PASSWORD.clone(),
            remember: None,
        };
    }

    fn _get_conn() -> PConn {
        let pool = POOL.clone();
        pool.get().ok().expect("Pool Connection doesnt exists")
    }

    fn _create_email(conn: &PConn) -> Result<User, diesel::result::Error> {
        use shortlink::database::schema::users::dsl::users;
        insert_into(users).values(NEW_USER.clone()).get_result(conn)
    }

    fn _delete_email(conn: &PConn) -> Result<bool, diesel::result::Error> {
        use shortlink::database::schema::users::dsl::{email, users};
        delete(users.filter(email.eq(EMAIL.to_owned())))
            .execute(conn)
            .map(|x| x == 1)
    }

    #[cfg(test)]
    mod auth_service_tests {
        use shortlink::services::auth::AuthService;

        use crate::service_tests::{_create_email, _delete_email};

        #[test]
        #[super::serial(auth)]
        fn register_test() {
            let conn = &super::_get_conn();
            let register_form = super::REGISTER_FORM.clone();

            let result = AuthService::register(conn, register_form);

            let _del = _delete_email(conn);

            assert!(result.ok().is_some());
        }

        #[test]
        #[super::serial(auth)]
        fn login_test() {
            let conn = &super::_get_conn();
            let login_form = super::LOGIN_FORM.clone();
            let _new = _create_email(conn);
            let result = AuthService::login(conn, login_form);
            let _del = _delete_email(conn);
            assert!(result.ok().is_some());
        }
    }

    #[cfg(test)]
    mod user_service_tests {

        use shortlink::services::user::UserService;

        use crate::service_tests::{_create_email, _delete_email};

        #[test]
        #[super::serial(user)]
        fn get_by_email_test() {
            let conn = &super::_get_conn();
            let _new = _create_email(conn);
            let result = UserService::get_one_by_email(conn, "test_email@mail.com".to_owned());
            assert!(result.ok().is_some());
            let _del = _delete_email(conn);
        }

        #[test]
        #[super::serial(user)]
        fn delete_by_email_test() {
            let conn = &super::_get_conn();
            let _new = _create_email(conn);
            let result = UserService::delete_by_email(conn, "test_email@mail.com".to_owned());

            assert!(result);
        }
    }
}
