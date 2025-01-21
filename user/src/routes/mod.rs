use actix_web::{get, HttpResponse};


#[get("/health_check")]
async fn test_route() -> HttpResponse{
    HttpResponse::Ok().body("working")
}
