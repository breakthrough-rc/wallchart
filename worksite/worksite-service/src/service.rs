use crate::{
    get_worksite::{GetWorksite, GetWorksiteFailure, GetWorksiteInput},
    //##PLOP INSERT COMMAND IMPORTS HOOK##
    assign_worker::{
      AssignWorker, AssignWorkerInput, AssignWorkerOutput, 
    },
    models::Worksite,
    remove_worker_from_shift::{
        RemoveWorkerFromShift, RemoveWorkerFromShiftFailure, RemoveWorkerFromShiftInput,
    },
};

#[derive(Clone)]
pub struct WorksiteService {
    //##PLOP INSERT COMMAND HOOK##
    pub assign_worker: AssignWorker,
    pub get_worksite: GetWorksite,
    pub remove_worker_from_shift: RemoveWorkerFromShift,
}

impl WorksiteService {
    //##PLOP INSERT DELEGATE HOOK##
    pub async fn assign_worker(
        &self,
        input: AssignWorkerInput,
    ) -> AssignWorkerOutput {
        self.assign_worker.assign_worker(input).await
    }

    pub async fn get_worksite(
        &self,
        input: GetWorksiteInput,
    ) -> Result<Option<Worksite>, GetWorksiteFailure> {
        self.get_worksite.get_worksite(input).await
    }

    pub async fn remove_worker_from_shift(
        &self,
        input: RemoveWorkerFromShiftInput,
    ) -> Result<Worksite, RemoveWorkerFromShiftFailure> {
        self.remove_worker_from_shift
            .remove_worker_from_shift(input)
            .await
    }
}
