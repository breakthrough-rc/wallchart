use crate::{
    get_worksite::{GetWorksite, GetWorksiteFailure},
    models::Worksite,
    remove_worker_from_shift::{RemoveWorkerFromShift, RemoveWorkerFromShiftFailure},
};

#[derive(Clone)]
pub struct WorksiteService {
    pub get_worksite: GetWorksite,
    pub remove_worker_from_shift: RemoveWorkerFromShift,
}

impl WorksiteService {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, GetWorksiteFailure> {
        self.get_worksite.get_worksite(id).await
    }

    async fn remove_worker_from_shift(
        &self,
        id: String,
        shift_id: String,
        worker_id: String,
    ) -> Result<Worksite, RemoveWorkerFromShiftFailure> {
        self.remove_worker_from_shift
            .remove_worker_from_shift(id, shift_id, worker_id)
            .await
    }
}
