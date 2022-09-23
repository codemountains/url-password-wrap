use crate::model::wrap::{NewWrap, Wrap};
use crate::model::Id;
use async_trait::async_trait;

#[async_trait]
pub trait WrapRepository {
    async fn get(&self, id: &Id<Wrap>) -> anyhow::Result<Option<Wrap>>;
    async fn insert(&self, source: NewWrap) -> anyhow::Result<Wrap>;
    async fn find(&self, id: &Id<Wrap>, password: &str) -> anyhow::Result<Wrap>;
}
