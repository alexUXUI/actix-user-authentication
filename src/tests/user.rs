#[cfg(test)] 
mod tests {
    use actix_web::{App, test, http::header};
    use crate::db::db_connection::establish_connection;
    use crate::routes::user::user_routes;
    use crate::handlers::user::{CreateUserResponse};

    #[actix_rt::test]
    async fn create() {
        let mut app = test::init_service(
            App::new()
                .data(establish_connection())
                .service(user_routes())
        ).await;

        let payload = r#"{"name": "alex z", "email": "alexz@email.com", "password": "123" }"#.as_bytes();

        let request = test::TestRequest::post()
            .uri("/users/create")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let response: CreateUserResponse = test::read_response_json(&mut app, request).await;

        assert_eq!(response.name, "alex z");
        assert_eq!(response.email, "alexz@email.com");
        
    }
}