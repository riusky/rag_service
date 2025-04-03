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

// region:    --- RagDocumentElements Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct RagDocumentElements {
    pub id: i64,

    // -- Properties
    pub book_id: i64,
    pub doc_type: String,
    pub content: Option<String>,
    pub text_level: Option<i64>,
    pub caption: Option<String>,
    pub footnote: Option<String>,
    pub text_format: Option<String>,
    pub img_path: Option<String>,
    pub page_idx: i64,
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct RagDocumentElementsForCreate {
    pub book_id: i64,
    pub doc_type: String,
    pub content: Option<String>,
    pub text_level: Option<i64>,
    pub caption: Option<String>,
    pub footnote: Option<String>,
    pub text_format: Option<String>,
    pub img_path: Option<String>,
    pub page_idx: i64,
}

#[derive(Fields, Deserialize)]
pub struct RagDocumentElementsForUpdate {
    pub book_id: Option<i64>,
    pub doc_type: Option<String>,
    pub content: Option<String>,
    pub text_level: Option<i64>,
    pub caption: Option<String>,
    pub footnote: Option<String>,
    pub text_format: Option<String>,
    pub img_path: Option<String>,
    pub page_idx: Option<i64>,
}

#[derive(FilterNodes, Default, Deserialize, Debug)]
pub struct RagDocumentElementsFilter {
    pub id: Option<OpValsInt64>,
    pub book_id: Option<OpValsInt64>,
    pub doc_type: Option<OpValsString>,
    pub content: Option<OpValsString>,
    pub text_level: Option<OpValsInt64>,
    pub caption: Option<OpValsString>,
    pub footnote: Option<OpValsString>,
    pub text_format: Option<OpValsString>,
    pub img_path: Option<OpValsString>,
    pub page_idx: Option<OpValsInt64>,
    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

// endregion: --- RagDocumentElements Types

// region:    --- RagDocumentElementsBmc

pub struct RagDocumentElementsBmc;

impl DbBmc for RagDocumentElementsBmc {
    const TABLE: &'static str = "rag_document_elements";

    fn has_owner_id() -> bool {
        false
    }
}

generate_common_bmc_fns!(
    Bmc: RagDocumentElementsBmc,
    Entity: RagDocumentElements,
    ForCreate: RagDocumentElementsForCreate,
    ForUpdate: RagDocumentElementsForUpdate,
    Filter: RagDocumentElementsFilter,
);

// endregion: --- RagDocumentElementsBmc