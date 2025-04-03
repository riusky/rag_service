// region:    --- Modules

pub mod agent_rpc;
pub mod conv_rpc;

use rpc_router::{Router, RouterBuilder};


pub mod rag_book_rpc;
pub mod rag_document_elements_rpc;
pub mod rag_model_rpc;
pub mod rag_prompt_configurations_rpc;
// endregion: --- Modules

pub fn all_rpc_router_builder() -> RouterBuilder {
	Router::builder()
		.extend(agent_rpc::rpc_router_builder())
		.extend(conv_rpc::rpc_router_builder())
		.extend(rag_prompt_configurations_rpc::rpc_router_builder())
		.extend(rag_model_rpc::rpc_router_builder())
		.extend(rag_document_elements_rpc::rpc_router_builder())
		.extend(rag_book_rpc::rpc_router_builder())
}