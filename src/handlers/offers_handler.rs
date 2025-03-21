use actix_web::{web, HttpResponse};

use crate::{
    models::offers::{InsertOffer, Offer},
    modules::stock_image::StockImage,
    state::AppState,
};

pub struct OffersHandler;

impl OffersHandler {
    pub async fn get_offers(app_state: web::Data<AppState>) -> HttpResponse {
        let result = sqlx::query_file_as!(Offer, "src/queries/get_offers.sql")
            .fetch_all(&app_state.pool)
            .await;

        match result {
            Ok(mut offers) => {
                for offer in &mut offers {
                    let image = StockImage {
                        data: offer.image.clone(),
                    };
                    let base64_image = image.to_base64();
                    offer.image = base64_image.into();
                }

                HttpResponse::Ok().json(offers)
            }
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }

    pub async fn insert_offer(
        app_state: web::Data<AppState>,
        payload: web::Json<InsertOffer>,
    ) -> HttpResponse {
        let stock_image = StockImage::get_stock_image(&payload.image);

        let result = sqlx::query_file!(
            "src/queries/insert_offer.sql",
            &payload.title,
            payload.description,
            &payload.link,
            stock_image.data
        )
        .execute(&app_state.pool)
        .await;

        match result {
            Ok(_) => HttpResponse::Created().into(),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }
}
