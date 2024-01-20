// 🐻‍❄️🧊 provisionerd: Deploy VMs with code.
// Copyright 2023-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::protos::generated::{
    self, CreateRequest, CreateResponse, DestroyRequest, DestroyResponse, ListRequest, ListResponse, PingRequest,
    PingResponse, StatusRequest, StatusResponse,
};
use remi_fs::StorageService;
use tonic::{Request, Response, Status};

/// Represents an implementation of the `noelware.provisionerd.v1.Daemon` service.
#[derive(Clone)]
pub struct Daemon {
    /// Storage service to hold data about VMs and its volumes.
    pub storage: StorageService,
}

#[async_trait]
impl generated::daemon_server::Daemon for Daemon {
    async fn status(&self, _request: Request<StatusRequest>) -> Result<Response<StatusResponse>, Status> {
        todo!()
    }

    async fn create(&self, _request: Request<CreateRequest>) -> Result<Response<CreateResponse>, Status> {
        todo!()
    }

    async fn destroy(&self, _request: Request<DestroyRequest>) -> Result<Response<DestroyResponse>, Status> {
        todo!()
    }

    async fn list(&self, _request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        todo!()
    }

    async fn ping(&self, _request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        todo!()
    }
}
