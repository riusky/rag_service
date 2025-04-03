// region: --- 文件头
//! 生成时间: 2025-04-02 06:09:13 UTC
//! 工具名称: Rust Model Generator Pro
//! 版本: 
//! 警告: 此文件为自动生成，请勿手动编辑！
//! 版权信息: © 2025 OceanKite。保留所有权利。
// endregion: --- 文件头


// region:    --- import
use lib_rpc_core::prelude::*;
use lib_core::model::rag_prompt_configurations::{
    RagPromptConfigurations, RagPromptConfigurationsBmc, RagPromptConfigurationsFilter,
    RagPromptConfigurationsForCreate, RagPromptConfigurationsForUpdate,
};
// endregion: --- import end

// region:    --- RPC router
pub fn rpc_router_builder() -> RouterBuilder {
    router_builder!(
        // Same as RpcRouter::new().add...
        create_rag_prompt_configurations,
        get_rag_prompt_configurations,
        list_rag_prompt_configurationss,
        update_rag_prompt_configurations,
        delete_rag_prompt_configurations,
    )
}

// 生成通用 RPC 函数宏
generate_common_rpc_fns!(
    Bmc: RagPromptConfigurationsBmc,
    Entity: RagPromptConfigurations,
    ForCreate: RagPromptConfigurationsForCreate,
    ForUpdate: RagPromptConfigurationsForUpdate,
    Filter: RagPromptConfigurationsFilter,
    Suffix: rag_prompt_configurations
);
// endregion: --- RPC router end