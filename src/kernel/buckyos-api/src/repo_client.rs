use async_trait::async_trait;

use ::kRPC::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, str::FromStr};
use log::*;

pub struct RepoClient {
    krpc_client: kRPC,
}

impl RepoClient {
    pub fn new(krpc_client: kRPC) -> Self {
        Self { krpc_client }
    }

    pub async fn pub_index(&self) -> Result<()> {
        let params = json!({});
        let _result = self.krpc_client.call("pub_index", params).await?;
        Ok(())
    }

    pub async fn pub_pkg(&self,pkg_meta_jwt_map: HashMap<String,String>) -> Result<()> {
        let params = json!({
            "pkg_list": pkg_meta_jwt_map
        });
        let _result = self.krpc_client.call("pub_pkg", params).await?;
        Ok(())
    }

    pub async fn sync_from_remote_source(&self) -> Result<()> {
        let params = json!({});
        let _result = self.krpc_client.call("sync_from_remote_source", params).await?;
        Ok(())
    }
}