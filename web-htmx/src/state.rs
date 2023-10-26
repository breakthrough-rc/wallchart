use std::sync::Arc;

use worksite_service::service::WorksiteService;

#[derive(Clone)]
pub struct WebHtmxState {
    pub worksite_service: Arc<WorksiteService>,
}
