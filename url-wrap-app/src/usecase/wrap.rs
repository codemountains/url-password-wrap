use crate::model::wrap::{CreateWrap, WrapView};
use chrono::{Duration, Utc};
use std::sync::Arc;
use url_wrap_adapter::modules::RepositoriesModuleExt;
use url_wrap_kernel::repository::wrap::WrapRepository;

pub struct WrapUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WrapUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }

    pub async fn get_wrap(&self, id: String) -> anyhow::Result<Option<WrapView>> {
        let res = self
            .repositories
            .wrap_repository()
            .get(&id.try_into()?)
            .await?;
        match res {
            Some(wrap) => Ok(Some(wrap.into())),
            None => Ok(None),
        }
    }

    pub async fn register_wrap(&self, source: CreateWrap) -> anyhow::Result<WrapView> {
        let wrap = self
            .repositories
            .wrap_repository()
            .insert(source.try_into()?)
            .await?;
        Ok(wrap.into())
    }

    pub async fn verify_wrap(
        &self,
        id: String,
        password: String,
    ) -> anyhow::Result<Option<WrapView>> {
        let now = Utc::now();

        let wrap = self
            .repositories
            .wrap_repository()
            .find(&id.try_into()?, &password)
            .await?;

        let duration: Duration = now - wrap.expiration_at;
        if duration.num_seconds() > 0 {
            Ok(None)
        } else {
            Ok(Some(wrap.into()))
        }
    }
}
