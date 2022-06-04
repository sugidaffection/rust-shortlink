#[cfg(test)]
mod environment_tests {

    use dotenv;
    use std::env;

    #[test]
    fn test_secret_key_must_be_set() {
        dotenv::dotenv().ok();
        let secret_key = env::var("SECRET_KEY");
        assert!(secret_key.is_ok());
    }

    #[test]
    fn test_secret_key_must_be_atleast_32_bytes() {
        dotenv::dotenv().ok();
        let secret_key = env::var("SECRET_KEY").unwrap();
        assert!(secret_key.len() >= 32);
    }

    #[test]
    fn test_database_url_must_be_set() {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL");
        assert!(database_url.is_ok());
    }
}
