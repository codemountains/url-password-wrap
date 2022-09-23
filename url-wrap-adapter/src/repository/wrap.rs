use crate::model::wrap::WrapDocument;
use crate::repository::MongoDBRepositoryImpl;
use anyhow::anyhow;
use async_trait::async_trait;
use mongodb::bson::doc;
use url_wrap_kernel::model::wrap::{NewWrap, Wrap};
use url_wrap_kernel::model::Id;
use url_wrap_kernel::repository::wrap::WrapRepository;

#[async_trait]
impl WrapRepository for MongoDBRepositoryImpl<Wrap> {
    async fn get(&self, id: &Id<Wrap>) -> anyhow::Result<Option<Wrap>> {
        let collection = self.db.0.collection::<WrapDocument>("wraps");

        let filter = doc! {"_id": id.value.to_string()};
        match collection.find_one(filter, None).await? {
            Some(wd) => Ok(Some(wd.try_into()?)),
            None => Ok(None),
        }
    }

    async fn insert(&self, source: NewWrap) -> anyhow::Result<Wrap> {
        let wrap_doc: WrapDocument = source.try_into()?;

        let collection = self.db.0.collection::<WrapDocument>("wraps");
        let insert_one_result = collection.insert_one(wrap_doc, None).await?;

        let id = insert_one_result
            .inserted_id
            .as_str()
            .ok_or(anyhow!("MongoDB `_id` is None."))?;

        let filter = doc! {"_id": id};
        match collection.find_one(filter, None).await? {
            Some(wd) => Ok(wd.try_into()?),
            None => Err(anyhow!("notting wrap.")),
        }
    }

    async fn find(&self, id: &Id<Wrap>, password: &str) -> anyhow::Result<Wrap> {
        let collection = self.db.0.collection::<WrapDocument>("wraps");

        let filter = doc! {"_id": id.value.to_string()};
        match collection.find_one(filter, None).await? {
            Some(wd) => match wd.verify_password(password) {
                Ok(_) => Ok(wd.try_into()?),
                Err(err) => Err(anyhow!(err)),
            },
            None => Err(anyhow!("notting wrap.")),
        }
    }
}
