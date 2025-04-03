// region: --- 文件头
//! 生成时间: 2025-04-02 06:09:13 UTC
//! 工具名称: Rust Model Generator Pro
//! 版本: 
//! 警告: 此文件为自动生成，请勿手动编辑！
//! 版权信息: © 2025 OceanKite。保留所有权利。
// endregion: --- 文件头

// region:    --- import
use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use serde_with::serde_as;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
// endregion: --- import end

// region:    --- RagModel Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RagModel {
    pub id: i64,

    // -- Properties
    pub ip: String,
    pub port: String,
    pub name: String,
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct RagModelForCreate {
    pub ip: String,
    pub port: String,
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct RagModelForUpdate {
    pub ip: Option<String>,
    pub port: Option<String>,
    pub name: Option<String>,
}

#[derive(FilterNodes, Default, Deserialize, Debug)]
pub struct RagModelFilter {
    pub id: Option<OpValsInt64>,
    pub ip: Option<OpValsString>,
    pub port: Option<OpValsString>,
    pub name: Option<OpValsString>,
    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

// endregion: --- RagModel Types

// region:    --- RagModelBmc

pub struct RagModelBmc;

impl DbBmc for RagModelBmc {
    const TABLE: &'static str = "rag_model";

    fn has_owner_id() -> bool {
        false
    }
}

generate_common_bmc_fns!(
    Bmc: RagModelBmc,
    Entity: RagModel,
    ForCreate: RagModelForCreate,
    ForUpdate: RagModelForUpdate,
    Filter: RagModelFilter,
);

// endregion: --- RagModelBmc