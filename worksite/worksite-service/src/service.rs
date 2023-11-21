use std::sync::Arc;

use crate::{
    add_location::{AddLocation, AddLocationInput, AddLocationOutput},
    add_shift::{AddShift, AddShiftInput, AddShiftOutput},
    add_worker::{AddWorker, AddWorkerInput, AddWorkerOutput},
    assign_worker::{AssignWorker, AssignWorkerInput, AssignWorkerOutput},
    //##PLOP INSERT COMMAND IMPORTS HOOK##
    remove_tag::{
      RemoveTag, RemoveTagInput, RemoveTagOutput, 
    },
    get_tag::{
      GetTag, GetTagInput, GetTagOutput, 
    },
    update_tag::{
      UpdateTag, UpdateTagInput, UpdateTagOutput, 
    },
    add_tag::{
      AddTag, AddTagInput, AddTagOutput, 
    },
    assign_tags::{
      AssignTags, AssignTagsInput, AssignTagsOutput, 
    },
    get_tags::{GetTags, GetTagsInput, GetTagsOutput},
    get_worker::{GetWorker, GetWorkerInput, GetWorkerOutput},
    get_workers::{GetWorkers, GetWorkersInput, GetWorkersOutput},
    get_worksite::{GetWorksite, GetWorksiteFailure, GetWorksiteInput},
    models::Worksite,
    ports::worksite_repository::WorksiteRepository,
    remove_worker_from_shift::{
        RemoveWorkerFromShift, RemoveWorkerFromShiftFailure, RemoveWorkerFromShiftInput,
    },
    update_worker::{UpdateWorker, UpdateWorkerInput, UpdateWorkerOutput},
};

#[derive(Clone)]
pub struct WorksiteService {
    //##PLOP INSERT COMMAND HOOK##
    pub remove_tag: RemoveTag,
    pub get_tag: GetTag,
    pub update_tag: UpdateTag,
    pub add_tag: AddTag,
    pub assign_tags: AssignTags,
    pub get_tags: GetTags,
    pub add_worker: AddWorker,
    pub get_workers: GetWorkers,
    pub add_shift: AddShift,
    pub add_location: AddLocation,
    pub update_worker: UpdateWorker,
    pub get_worker: GetWorker,
    pub assign_worker: AssignWorker,
    pub get_worksite: GetWorksite,
    pub remove_worker_from_shift: RemoveWorkerFromShift,
}

impl WorksiteService {
    pub fn new(worksite_repository: Arc<dyn WorksiteRepository>) -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            remove_tag: RemoveTag {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
              worksite_repository: worksite_repository.clone(),
            },
            get_tag: GetTag {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
              worksite_repository: worksite_repository.clone(),
            },
            update_tag: UpdateTag {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
              worksite_repository: worksite_repository.clone(),
            },
            add_tag: AddTag {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
              worksite_repository: worksite_repository.clone(),
            },
            assign_tags: AssignTags {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
              worksite_repository: worksite_repository.clone(),
            },
            get_tags: GetTags {
                worksite_repository: worksite_repository.clone(),
            },
            add_worker: AddWorker {
                worksite_repository: worksite_repository.clone(),
            },
            get_workers: GetWorkers {
                worksite_repository: worksite_repository.clone(),
            },
            add_shift: AddShift {
                worksite_repository: worksite_repository.clone(),
            },
            add_location: AddLocation {
                worksite_repository: worksite_repository.clone(),
            },
            update_worker: UpdateWorker {
                worksite_repository: worksite_repository.clone(),
            },
            get_worker: GetWorker {
                worksite_repository: worksite_repository.clone(),
            },
            assign_worker: AssignWorker::new(worksite_repository.clone()),
            get_worksite: GetWorksite {
                worksite_repository: worksite_repository.clone(),
            },
            remove_worker_from_shift: RemoveWorkerFromShift {
                worksite_repository: worksite_repository.clone(),
            },
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
    pub async fn remove_tag(
        &self,
        input: RemoveTagInput,
    ) -> RemoveTagOutput {
        self.remove_tag.remove_tag(input).await
    }

    pub async fn get_tag(
        &self,
        input: GetTagInput,
    ) -> GetTagOutput {
        self.get_tag.get_tag(input).await
    }

    pub async fn update_tag(
        &self,
        input: UpdateTagInput,
    ) -> UpdateTagOutput {
        self.update_tag.update_tag(input).await
    }

    pub async fn add_tag(
        &self,
        input: AddTagInput,
    ) -> AddTagOutput {
        self.add_tag.add_tag(input).await
    }

    pub async fn assign_tags(
        &self,
        input: AssignTagsInput,
    ) -> AssignTagsOutput {
        self.assign_tags.assign_tags(input).await
    }

    pub async fn get_tags(&self, input: GetTagsInput) -> GetTagsOutput {
        self.get_tags.get_tags(input).await
    }

    pub async fn add_worker(&self, input: AddWorkerInput) -> AddWorkerOutput {
        self.add_worker.add_worker(input).await
    }

    pub async fn get_workers(&self, input: GetWorkersInput) -> GetWorkersOutput {
        self.get_workers.get_workers(input).await
    }

    pub async fn add_shift(&self, input: AddShiftInput) -> AddShiftOutput {
        self.add_shift.add_shift(input).await
    }

    pub async fn add_location(&self, input: AddLocationInput) -> AddLocationOutput {
        self.add_location.add_location(input).await
    }

    pub async fn update_worker(&self, input: UpdateWorkerInput) -> UpdateWorkerOutput {
        self.update_worker.update_worker(input).await
    }

    pub async fn get_worker(&self, input: GetWorkerInput) -> GetWorkerOutput {
        self.get_worker.get_worker(input).await
    }

    pub async fn assign_worker(&self, input: AssignWorkerInput) -> AssignWorkerOutput {
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
