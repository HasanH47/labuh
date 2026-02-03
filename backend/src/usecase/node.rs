use crate::domain::runtime::{RuntimePort, SwarmNode, SwarmTokens};
use crate::error::Result;
use std::sync::Arc;

pub struct NodeUsecase {
    runtime: Arc<dyn RuntimePort>,
}

impl NodeUsecase {
    pub fn new(runtime: Arc<dyn RuntimePort>) -> Self {
        Self { runtime }
    }

    pub async fn list_nodes(&self) -> Result<Vec<SwarmNode>> {
        self.runtime.list_nodes().await
    }

    pub async fn inspect_node(&self, id: &str) -> Result<SwarmNode> {
        self.runtime.inspect_node(id).await
    }

    pub async fn is_swarm_enabled(&self) -> Result<bool> {
        self.runtime.is_swarm_enabled().await
    }

    pub async fn init_swarm(&self, listen_addr: &str) -> Result<String> {
        self.runtime.swarm_init(listen_addr).await
    }

    pub async fn join_swarm(
        &self,
        listen_addr: &str,
        remote_addr: &str,
        token: &str,
    ) -> Result<()> {
        self.runtime
            .swarm_join(listen_addr, remote_addr, token)
            .await
    }

    pub async fn get_tokens(&self) -> Result<SwarmTokens> {
        self.runtime.get_swarm_tokens().await
    }
}
