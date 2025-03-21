use crate::{
    models::offers::{Offer, OfferRes},
    modules::stock_image::StockImage,
};

pub struct ChangeOfferResponse;

impl ChangeOfferResponse {
    pub fn change_offer_response(offer: Offer) -> OfferRes {
        let image_base64: String = StockImage { data: offer.image }.to_base64();

        OfferRes {
            offer_id: offer.offer_id,
            title: offer.title,
            description: offer.description,
            link: offer.link,
            image: image_base64,
        }
    }
}
