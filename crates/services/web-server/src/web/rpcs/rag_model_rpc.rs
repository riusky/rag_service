// region: --- 文件头
//! 生成时间: 2025-04-02 06:09:13 UTC
//! 工具名称: Rust Model Generator Pro
//! 版本: 
//! 警告: 此文件为自动生成，请勿手动编辑！
//! 版权信息: © 2025 OceanKite。保留所有权利。
// endregion: --- 文件头


// region:    --- import
use lib_rpc_core::prelude::*;
use lib_core::model::rag_model::{
    RagModel, RagModelBmc, RagModelFilter,
    RagModelForCreate, RagModelForUpdate,
};
// endregion: --- import end

// region:    --- RPC router
pub fn rpc_router_builder() -> RouterBuilder {
    router_builder!(
        // Same as RpcRouter::new().add...
        create_rag_model,
        get_rag_model,
        list_rag_models,
        update_rag_model,
        delete_rag_model,
    )
}

// 生成通用 RPC 函数宏
generate_common_rpc_fns!(
    Bmc: RagModelBmc,
    Entity: RagModel,
    ForCreate: RagModelForCreate,
    ForUpdate: RagModelForUpdate,
    Filter: RagModelFilter,
    Suffix: rag_model
);
// endregion: --- RPC router end