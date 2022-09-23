use std::sync::Arc;
use url_wrap_adapter::modules::{RepositoriesModule, RepositoriesModuleExt};
use url_wrap_adapter::persistence::mongodb::Db;
use url_wrap_adapter::repository::health_check::HealthCheckRepository;
use url_wrap_app::usecase::health_check::HealthCheckUseCase;
use url_wrap_app::usecase::wrap::WrapUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
    wrap_use_case: WrapUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
    fn wrap_use_case(&self) -> &WrapUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }

    fn wrap_use_case(&self) -> &WrapUseCase<Self::RepositoriesModule> {
        &self.wrap_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let health_check_use_case = HealthCheckUseCase::new(HealthCheckRepository::new(db));
        let wrap_use_case = WrapUseCase::new(repositories_module.clone());

        Self {
            health_check_use_case,
            wrap_use_case,
        }
    }
}
