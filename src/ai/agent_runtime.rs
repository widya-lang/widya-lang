//! Native Multi-Agent Runtime & Autonomous Tool-Calling Engine (agent_runtime.rs)
//! Mendukung deklarasi agen kelas satu, ReAct Loop (Thought -> Action -> Observation), dan komunikasi antar-agen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definisi Agen Cerdas di Widya-Lang
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeAgent {
    pub name: String,
    pub model_id: String,
    pub system_prompt: String,
    pub tools: Vec<String>,
    pub memory_context: Vec<String>,
}

/// Permintaan Eksekusi Alat oleh Agen (Tool Call Request)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: HashMap<String, String>,
}

/// Hasil Eksekusi Alat (Tool Execution Result)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
}

/// Registri Pustaka Alat (Tool Registry)
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(&HashMap<String, String>) -> ToolResult + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&HashMap<String, String>) -> ToolResult + Send + Sync + 'static,
    {
        self.tools.insert(name.to_string(), Box::new(handler));
    }

    pub fn execute(&self, call: &ToolCall) -> ToolResult {
        if let Some(handler) = self.tools.get(&call.tool_name) {
            handler(&call.arguments)
        } else {
            ToolResult {
                success: false,
                output: format!("Alat '{}' tidak ditemukan dalam registri agen", call.tool_name),
            }
        }
    }
}

/// ReAct Execution Engine (Reasoning + Acting)
pub struct AgentExecutor {
    pub agent: NativeAgent,
    pub tool_registry: ToolRegistry,
}

impl AgentExecutor {
    pub fn new(agent: NativeAgent, tool_registry: ToolRegistry) -> Self {
        Self { agent, tool_registry }
    }

    /// Menjalankan satu putaran observasi ReAct Loop
    pub fn step(&mut self, user_goal: &str, tool_call_opt: Option<ToolCall>) -> String {
        self.agent.memory_context.push(format!("Goal: {}", user_goal));

        if let Some(tool_call) = tool_call_opt {
            let res = self.tool_registry.execute(&tool_call);
            let obs = format!("Observation from {}: {}", tool_call.tool_name, res.output);
            self.agent.memory_context.push(obs.clone());
            obs
        } else {
            let thought = format!("Agent {} berpikir dan menganalisis: {}", self.agent.name, user_goal);
            self.agent.memory_context.push(thought.clone());
            thought
        }
    }
}

/// Saluran Komunikasi Antar-Agen (Multi-Agent Negotiation Channel)
pub struct MultiAgentBus {
    inbox: HashMap<String, Vec<String>>,
}

impl MultiAgentBus {
    pub fn new() -> Self {
        Self {
            inbox: HashMap::new(),
        }
    }

    pub fn send_message(&mut self, recipient: &str, message: &str) {
        self.inbox.entry(recipient.to_string()).or_default().push(message.to_string());
    }

    pub fn receive_messages(&mut self, agent_name: &str) -> Vec<String> {
        self.inbox.remove(agent_name).unwrap_or_default()
    }
}
