use crate::persistence::mongodb::Db;
use crate::repository::MongoDBRepositoryImpl;
use url_wrap_kernel::model::wrap::Wrap;
use url_wrap_kernel::repository::wrap::WrapRepository;

pub struct RepositoriesModule {
    wrap_repository: MongoDBRepositoryImpl<Wrap>,
}

pub trait RepositoriesModuleExt {
    type WrapRepo: WrapRepository;

    fn wrap_repository(&self) -> &Self::WrapRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type WrapRepo = MongoDBRepositoryImpl<Wrap>;

    fn wrap_repository(&self) -> &Self::WrapRepo {
        &self.wrap_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let wrap_repository = MongoDBRepositoryImpl::new(db.clone());

        Self { wrap_repository }
    }
}
