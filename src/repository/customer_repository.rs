use mongodb::{bson::doc, Collection};

use crate::{models::customer_model::Customer, shared::utils::ToDocument, DBClient};

pub struct CustomerRepository {
    collection: Collection<Customer>,
}

impl CustomerRepository {
    pub fn new(client: &DBClient) -> Self {
        Self {
            collection: client.database("rinha").collection("customers"),
        }
    }

    pub async fn get_one(&self, id: &i32) -> Result<Option<Customer>, Box<dyn std::error::Error>> {
        let customer = self
            .collection
            .find_one(
                doc! {
                    "id": id,
                },
                None,
            )
            .await?;

        Ok(customer)
    }

    pub async fn update(
        &self,
        id: &i32,
        new_customer: &Customer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.collection
            .update_one(
                doc! {
                    "id": id,
                },
                doc! {
                    "$set": new_customer.to_document()?,
                },
                None,
            )
            .await?;

        Ok(())
    }
}
