use std::collections::HashMap;

/// Status Hasil Eksekusi EVM Smart Contract
#[derive(Debug, Clone, PartialEq)]
pub enum EvmExecutionStatus {
    Success,
    Revert,
    OutOfGas,
    InvalidOpcode(u8),
    StackUnderflow,
    StackOverflow,
}

/// Konteks Eksekusi EVM
#[derive(Debug, Clone)]
pub struct EvmContext {
    pub caller: [u8; 20],
    pub contract_address: [u8; 20],
    pub call_value: u64,
    pub calldata: Vec<u8>,
    pub gas_limit: u64,
}

impl Default for EvmContext {
    fn default() -> Self {
        Self {
            caller: [0x11; 20],
            contract_address: [0x22; 20],
            call_value: 0,
            calldata: Vec::new(),
            gas_limit: 1_000_000,
        }
    }
}

/// Hasil Eksekusi Mesin Virtual Ethereum (EVM)
#[derive(Debug, Clone)]
pub struct EvmResult {
    pub status: EvmExecutionStatus,
    pub return_data: Vec<u8>,
    pub gas_used: u64,
    pub logs: Vec<EvmLog>,
}

#[derive(Debug, Clone)]
pub struct EvmLog {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

/// Mesin Virtual EVM Mikro Mandiri (EVM Micro-Runtime)
pub struct EvmRuntime {
    pub stack: Vec<[u8; 32]>,
    pub memory: Vec<u8>,
    pub storage: HashMap<[u8; 32], [u8; 32]>,
    pub pc: usize,
    pub gas_remaining: u64,
}

impl EvmRuntime {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            stack: Vec::with_capacity(1024),
            memory: Vec::new(),
            storage: HashMap::new(),
            pc: 0,
            gas_remaining: gas_limit,
        }
    }

    /// Eksekusi Bytecode EVM dengan Konteks Transaksi
    pub fn execute(&mut self, bytecode: &[u8], ctx: &EvmContext) -> EvmResult {
        let initial_gas = self.gas_remaining;

        while self.pc < bytecode.len() {
            if self.gas_remaining < 3 {
                return EvmResult {
                    status: EvmExecutionStatus::OutOfGas,
                    return_data: Vec::new(),
                    gas_used: initial_gas - self.gas_remaining,
                    logs: Vec::new(),
                };
            }
            self.gas_remaining -= 3; // Biaya gas per instruksi dasar

            let opcode = bytecode[self.pc];
            self.pc += 1;

            match opcode {
                0x00 => { // STOP
                    break;
                }
                0x01 => { // ADD
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let a = self.pop_u64();
                    let b = self.pop_u64();
                    self.push_u64(a.wrapping_add(b));
                }
                0x02 => { // MUL
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let a = self.pop_u64();
                    let b = self.pop_u64();
                    self.push_u64(a.wrapping_mul(b));
                }
                0x03 => { // SUB
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let a = self.pop_u64();
                    let b = self.pop_u64();
                    self.push_u64(a.wrapping_sub(b));
                }
                0x04 => { // DIV
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let a = self.pop_u64();
                    let b = self.pop_u64();
                    self.push_u64(if b == 0 { 0 } else { a / b });
                }
                0x35 => { // CALLDATALOAD
                    if self.stack.len() < 1 { return self.err_underflow(initial_gas); }
                    let offset = self.pop_u64() as usize;
                    let mut word = [0u8; 32];
                    if offset < ctx.calldata.len() {
                        let end = (offset + 32).min(ctx.calldata.len());
                        let copy_len = end - offset;
                        word[0..copy_len].copy_from_slice(&ctx.calldata[offset..end]);
                    }
                    self.stack.push(word);
                }
                0x36 => { // CALLDATASIZE
                    self.push_u64(ctx.calldata.len() as u64);
                }
                0x50 => { // POP
                    if self.stack.pop().is_none() { return self.err_underflow(initial_gas); }
                }
                0x51 => { // MLOAD
                    if self.stack.len() < 1 { return self.err_underflow(initial_gas); }
                    let offset = self.pop_u64() as usize;
                    let mut word = [0u8; 32];
                    if offset + 32 <= self.memory.len() {
                        word.copy_from_slice(&self.memory[offset..offset + 32]);
                    }
                    self.stack.push(word);
                }
                0x52 => { // MSTORE
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let offset = self.pop_u64() as usize;
                    let val = self.stack.pop().unwrap();
                    if offset + 32 > self.memory.len() {
                        self.memory.resize(offset + 32, 0);
                    }
                    self.memory[offset..offset + 32].copy_from_slice(&val);
                }
                0x54 => { // SLOAD
                    if self.stack.len() < 1 { return self.err_underflow(initial_gas); }
                    let key = self.stack.pop().unwrap();
                    let val = self.storage.get(&key).copied().unwrap_or([0u8; 32]);
                    self.stack.push(val);
                }
                0x55 => { // SSTORE
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let key = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    self.storage.insert(key, val);
                }
                0x60..=0x7f => { // PUSH1 .. PUSH32
                    let num_bytes = (opcode - 0x5f) as usize;
                    if self.pc + num_bytes > bytecode.len() {
                        return self.err_underflow(initial_gas);
                    }
                    let mut word = [0u8; 32];
                    let start = 32 - num_bytes;
                    word[start..32].copy_from_slice(&bytecode[self.pc..self.pc + num_bytes]);
                    self.stack.push(word);
                    self.pc += num_bytes;
                }
                0xf3 => { // RETURN
                    if self.stack.len() < 2 { return self.err_underflow(initial_gas); }
                    let offset = self.pop_u64() as usize;
                    let length = self.pop_u64() as usize;
                    let mut data = Vec::new();
                    if offset + length <= self.memory.len() {
                        data.extend_from_slice(&self.memory[offset..offset + length]);
                    }
                    return EvmResult {
                        status: EvmExecutionStatus::Success,
                        return_data: data,
                        gas_used: initial_gas - self.gas_remaining,
                        logs: Vec::new(),
                    };
                }
                0xfd => { // REVERT
                    return EvmResult {
                        status: EvmExecutionStatus::Revert,
                        return_data: Vec::new(),
                        gas_used: initial_gas - self.gas_remaining,
                        logs: Vec::new(),
                    };
                }
                other => {
                    return EvmResult {
                        status: EvmExecutionStatus::InvalidOpcode(other),
                        return_data: Vec::new(),
                        gas_used: initial_gas - self.gas_remaining,
                        logs: Vec::new(),
                    };
                }
            }
        }

        EvmResult {
            status: EvmExecutionStatus::Success,
            return_data: Vec::new(),
            gas_used: initial_gas - self.gas_remaining,
            logs: Vec::new(),
        }
    }

    fn pop_u64(&mut self) -> u64 {
        let word = self.stack.pop().unwrap_or([0u8; 32]);
        let mut b = [0u8; 8];
        b.copy_from_slice(&word[24..32]);
        u64::from_be_bytes(b)
    }

    fn push_u64(&mut self, val: u64) {
        let mut word = [0u8; 32];
        word[24..32].copy_from_slice(&val.to_be_bytes());
        self.stack.push(word);
    }

    fn err_underflow(&self, initial_gas: u64) -> EvmResult {
        EvmResult {
            status: EvmExecutionStatus::StackUnderflow,
            return_data: Vec::new(),
            gas_used: initial_gas - self.gas_remaining,
            logs: Vec::new(),
        }
    }
}
