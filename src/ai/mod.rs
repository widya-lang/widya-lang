pub mod gguf;
pub mod rag;
pub mod tensor;
pub mod vector_db;
pub mod agent;
pub mod inference;
pub mod quantization;
pub mod agent_runtime;
pub mod agent_market;
pub mod python_bridge;
pub mod dlpack;
pub mod autograd;
pub mod dataframe;
pub mod federated_learning;
pub mod autonomous_hosting;
pub mod voice_stream;
pub mod lora;
pub mod federated_rag;

pub use gguf::{
    parse_gguf_header, GgufModelInfo, GgufTensorInfo, GgufTensorType, GgufValue,
    dequantize_q4_0, dequantize_q8_0, vec_dot_q4_0_f32, build_synthetic_gguf,
};
pub use rag::{RagDocument, RagIndex};
pub use tensor::Tensor;
pub use vector_db::{VectorDatabase, VectorItem, SearchResult, DistanceMetric};
pub use agent::{AgentRuntime, AgentProfile, SovereignTool, ToolParameter, ToolCallRequest, ToolCallResponse};
pub use inference::{
    SovereignInferenceEngine, ModelGenerationConfig, KvCache, rms_norm, apply_rope, softmax,
    TokenSampler,
};
pub use quantization::{QuantizedTensor, QuantType};
pub use agent_runtime::{NativeAgent, ToolCall, ToolResult, ToolRegistry, AgentExecutor, MultiAgentBus};
pub use agent_market::{AgentSkill, MicroSettlement, AgentSkillMarket};
pub use python_bridge::{PythonBridge, PyValue};
pub use dlpack::{DLTensor, DLDataType, DLDataTypeCode, DLDevice, DLDeviceType, SafetensorsParser};
pub use autograd::{AutoGradEngine, AutoGradNode, GraphOp};
pub use dataframe::{DataFrame, DataColumn, ColumnType};
pub use federated_learning::{FederatedLearningCoordinator, ClientWeightUpdate, FederatedRoundStatus};
pub use autonomous_hosting::{AutonomousAgentHostManager, AutonomousAgentState};
pub use voice_stream::{VoiceStreamEngine, AudioPcmChunk, AudioSampleRate, VoiceSessionState};
pub use lora::LoraLinearLayer;
pub use federated_rag::{FederatedRagEngine, FederatedSearchResult, LocalVectorDoc};
pub mod pure_weights;
pub use pure_weights::{
    PureGgufEngine, PureGgufModel, PureGgufTensor, PureGgufType, PureGgufValue,
    PureSafeTensorsEngine, SafeTensorRecord, SafeTensorsModel,
};

