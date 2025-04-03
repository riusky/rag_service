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

// region:    --- RagPromptConfigurations Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RagPromptConfigurations {
    pub id: i64,

    // -- Properties
    pub name: String,
    pub prompt: String,
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct RagPromptConfigurationsForCreate {
    pub name: String,
    pub prompt: String,
}

#[derive(Fields, Deserialize)]
pub struct RagPromptConfigurationsForUpdate {
    pub name: Option<String>,
    pub prompt: Option<String>,
}

#[derive(FilterNodes, Default, Deserialize, Debug)]
pub struct RagPromptConfigurationsFilter {
    pub id: Option<OpValsInt64>,
    pub name: Option<OpValsString>,
    pub prompt: Option<OpValsString>,
    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

// endregion: --- RagPromptConfigurations Types

// region:    --- RagPromptConfigurationsBmc

pub struct RagPromptConfigurationsBmc;

impl DbBmc for RagPromptConfigurationsBmc {
    const TABLE: &'static str = "rag_prompt_configurations";

    fn has_owner_id() -> bool {
        false
    }
}

generate_common_bmc_fns!(
    Bmc: RagPromptConfigurationsBmc,
    Entity: RagPromptConfigurations,
    ForCreate: RagPromptConfigurationsForCreate,
    ForUpdate: RagPromptConfigurationsForUpdate,
    Filter: RagPromptConfigurationsFilter,
);

// endregion: --- RagPromptConfigurationsBmc