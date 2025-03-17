#[cfg(test)]
mod home_tests {

    use actix_web::{test, web, App};
    use dotenvy::dotenv;
    use shortlink::home::handlers::home;
    use shortlink::prelude::*;

    use ::lazy_static::lazy_static;

    lazy_static! {
        static ref APP_DATA: AppData = AppData::new(None);
    }

    #[actix_web::test]
    async fn test_index_get() {
        dotenv().ok();

        let app = App::new()
            .app_data(web::Data::new(APP_DATA.clone()))
            .service(home);
        let app = test::init_service(app).await;
        let req = test::TestRequest::get().to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_get_no_template() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new().service(home)).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
