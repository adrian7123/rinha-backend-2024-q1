use bson::Document;
use serde::Serialize;

pub trait ToDocument: Serialize {
    fn to_document(&self) -> Result<Document, Box<dyn std::error::Error>> {
        let serialized_customer = bson::to_bson(&self)?;
        let document = serialized_customer.as_document().unwrap();

        Ok(document.clone())
    }
}
