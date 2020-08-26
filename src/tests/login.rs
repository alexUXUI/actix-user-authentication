#[cfg(test)]
mod tests {

    #[actix_rt::test]
    async fn login() {
        use actix_web::{App, test, http::header};
        use crate::db::db_connection::establish_connection;
        use crate::routes::login::login;
        use crate::handlers::user::{UserLoginResponse};

        let mut app = test::init_service(
            App::new()
                .data(establish_connection())
                .service(login())
        ).await;

        let payload = r#"{"name": "miguel", "password": "123"}"#.as_bytes();

        let req = test::TestRequest::post()
            .uri("/app/login")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let response: UserLoginResponse = test::read_response_json(&mut app, req).await;

        assert_eq!(response.user_logged_in.name, "miguel");
        assert_eq!(response.user_logged_in.email, "miguel@email.com");
    }
}