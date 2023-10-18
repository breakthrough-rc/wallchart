use std::sync::Arc;

use usecases::service::WorksiteService;

#[derive(Clone)]
pub struct WebHtmxState {
    pub worksite_service: Arc<WorksiteService>,
}
