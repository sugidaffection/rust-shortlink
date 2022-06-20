#[cfg(test)]
mod service_tests {

    use lazy_static::lazy_static;
    use shortlink::{create_pool, Pool};

    lazy_static! {
        static ref POOL: Pool = {
            dotenv::dotenv().ok();
            create_pool(None)
        };
    }

    #[cfg(test)]
    mod auth_service_tests {
        use secrecy::Secret;
        use shortlink::forms::{LoginForm, RegisterForm};
        use shortlink::services::auth::AuthService;

        #[test]
        fn register_test() {
            let pool = super::POOL.clone();
            let conn = &pool.get().ok().expect("Pool Connection doesnt exists");

            let register_form = RegisterForm {
                email: "test_email@mail.com".to_owned(),
                username: "test".to_owned(),
                password: Secret::new("test123".to_owned()),
            };

            let result = AuthService::register(conn, register_form);

            assert!(result.ok().is_some());
        }

        #[test]
        fn login_test() {
            let pool = super::POOL.clone();
            let conn = &pool.get().ok().expect("Pool Connection doesnt exists");
            let login_form = LoginForm {
                email: "test_email@mail.com".to_owned(),
                password: Secret::new("test123".to_owned()),
                remember: None,
            };
            let result = AuthService::login(conn, login_form);

            assert!(result.ok().is_some());
        }
    }

    #[cfg(test)]
    mod user_service_tests {

        use shortlink::services::user::UserService;

        #[test]
        fn get_by_email_test() {
            let pool = super::POOL.clone();
            let conn = &pool.get().ok().expect("Pool Connection doesnt exists");
            let result = UserService::get_one_by_email(conn, "test_email@mail.com".to_owned());

            assert!(result.ok().is_some());
        }

        #[test]
        fn delete_by_email_test() {
            let pool = super::POOL.clone();
            let conn = &pool.get().ok().expect("Pool Connection doesnt exists");
            let result = UserService::delete_by_email(conn, "test_email@mail.com".to_owned());

            assert!(result);
        }
    }
}
