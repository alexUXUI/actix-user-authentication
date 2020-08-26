#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::health::{status, HealthResponse};
    
    #[actix_rt::test]
    async fn test_health() {
        
        let mut app = test::init_service(
            App::new().service(status)
        ).await;

        let req = test::TestRequest::get()
            .uri("/ping")
            .to_request();

        let resp: HealthResponse = test::read_response_json(&mut app, req).await;

        assert_eq!(resp.status, String::from("pong!"));
    }
}