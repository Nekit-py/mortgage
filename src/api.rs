pub mod api {
    use crate::mortgage::mortgage::Mortgage;
    use actix_web::{post, web, HttpResponse, Result, Responder};

    #[post("/schedule")]
    pub async fn payment_schedule(mortgage: String) -> Result<impl Responder>  {
        let mortgage: Mortgage = serde_json::from_str(&mortgage[..]).unwrap();
        let response = mortgage.show_payment_schedule();
        Ok(web::Json(response))
    }

    pub async fn index() -> impl Responder {
        HttpResponse::Ok().body("Welcome to the Mortgage Calculator!")
    }

    #[post("/overpayment")]
    pub async fn overpayment(mortgage: String) -> impl Responder{
        let mortgage: Mortgage = serde_json::from_str(&mortgage[..]).unwrap();
        HttpResponse::Ok().body(
            format!(
                "{} {:.2}",
                "Total amount:".to_string(),
                mortgage.calculate_total_amount()
            )
        )
    }
}