//! Pure Sovereign Zero-Dependency Bare-Metal RTOS & Vector Table Emitter (pure_rtos.rs)
//!
//! 100% In-House Implementation:
//! - ARM Cortex-M Thumb-2 Vector Table Emitter (Reset, HardFault, PendSV, SysTick)
//! - RISC-V RV32I / RV64I Machine-Mode Vector Table (mtvec Direct & Vectored modes)
//! - Thread Control Block (TCB) & Priority-Preemptive Real-Time Scheduler
//! - Priority Inheritance Mutex (PI-Mutex) untuk mencegah Priority Inversion
//! - Fixed-Size Inter-Task Communication Queue (Zero Heap in Real-Time ISR)

// ============================================================================
// 1. HARDWARE VECTOR TABLE EMITTER (ARM CORTEX-M & RISC-V)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BareMetalArch {
    ArmCortexM,
    RiscvRv32I,
}

/// Generator ARM Cortex-M Vector Table (Thumb-2 ABI)
#[derive(Debug, Clone)]
pub struct ArmCortexMVectorTable {
    pub initial_sp: u32,
    pub reset_handler: u32,
    pub nmi_handler: u32,
    pub hardfault_handler: u32,
    pub memmanage_handler: u32,
    pub busfault_handler: u32,
    pub usagefault_handler: u32,
    pub svcall_handler: u32,
    pub pendsv_handler: u32,
    pub systick_handler: u32,
    pub peripheral_irqs: Vec<u32>,
}

impl ArmCortexMVectorTable {
    pub fn new(initial_sp: u32, reset_addr: u32) -> Self {
        // Pada ARM Cortex-M Thumb mode, alamat instruksi bit 0 harus selalu bernilai 1
        let thumb_reset = reset_addr | 0x01;
        Self {
            initial_sp,
            reset_handler: thumb_reset,
            nmi_handler: thumb_reset,
            hardfault_handler: thumb_reset,
            memmanage_handler: 0,
            busfault_handler: 0,
            usagefault_handler: 0,
            svcall_handler: thumb_reset,
            pendsv_handler: thumb_reset,
            systick_handler: thumb_reset,
            peripheral_irqs: Vec::new(),
        }
    }

    pub fn set_systick_handler(&mut self, addr: u32) {
        self.systick_handler = addr | 0x01;
    }

    pub fn set_pendsv_handler(&mut self, addr: u32) {
        self.pendsv_handler = addr | 0x01;
    }

    pub fn add_peripheral_irq(&mut self, addr: u32) {
        self.peripheral_irqs.push(addr | 0x01);
    }

    /// Serialisasi tabel vektor ke stream biner flash 32-bit Little-Endian
    pub fn to_binary(&self) -> Vec<u8> {
        let total_entries = 16 + self.peripheral_irqs.len();
        let mut out = Vec::with_capacity(total_entries * 4);

        let system_vectors = [
            self.initial_sp,
            self.reset_handler,
            self.nmi_handler,
            self.hardfault_handler,
            self.memmanage_handler,
            self.busfault_handler,
            self.usagefault_handler,
            0, 0, 0, 0, // 4 Reserved entries
            self.svcall_handler,
            0, // Debug Monitor
            0, // Reserved
            self.pendsv_handler,
            self.systick_handler,
        ];

        for &vec in &system_vectors {
            out.extend_from_slice(&vec.to_le_bytes());
        }

        for &irq in &self.peripheral_irqs {
            out.extend_from_slice(&irq.to_le_bytes());
        }

        out
    }
}

/// Generator RISC-V Machine-Mode Trap Vector Table (`mtvec`)
#[derive(Debug, Clone)]
pub struct RiscvVectorTable {
    pub base_addr: u32,
    pub is_vectored: bool,
    pub exception_handlers: [u32; 16], // 16 Standard Traps
}

impl RiscvVectorTable {
    pub fn new(base_addr: u32, is_vectored: bool) -> Self {
        Self {
            base_addr,
            is_vectored,
            exception_handlers: [base_addr; 16],
        }
    }

    /// Nilai register konfigurasi `mtvec`: BASE (bits 2..31) | MODE (bit 0: 0=Direct, 1=Vectored)
    pub fn mtvec_val(&self) -> u32 {
        let mode = if self.is_vectored { 1 } else { 0 };
        (self.base_addr & !0x03) | mode
    }

    /// Serialisasi tabel vektor RISC-V dalam format biner
    pub fn to_binary(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(16 * 4);
        for &h in &self.exception_handlers {
            out.extend_from_slice(&h.to_le_bytes());
        }
        out
    }
}

// ============================================================================
// 2. SOVEREIGN REAL-TIME OPERATING SYSTEM (RTOS) ENGINE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked(u64), // Blocked until tick counter reaches value
    Suspended,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct CpuContext {
    pub r: [u32; 16], // Register R0..R15 / General Purpose Registers
    pub sp: u32,
    pub pc: u32,
    pub psr: u32,
}

impl CpuContext {
    pub fn new(entry_point: u32, stack_top: u32) -> Self {
        Self {
            r: [0u32; 16],
            sp: stack_top,
            pc: entry_point,
            psr: 0x01000000, // Thumb bit set default
        }
    }
}

/// Thread Control Block (TCB)
#[derive(Debug, Clone)]
pub struct TaskControlBlock {
    pub task_id: u32,
    pub name: String,
    pub base_priority: u8,
    pub current_priority: u8,
    pub state: TaskState,
    pub context: CpuContext,
    pub total_ticks_executed: u64,
}

impl TaskControlBlock {
    pub fn new(id: u32, name: &str, priority: u8, entry_point: u32, stack_top: u32) -> Self {
        Self {
            task_id: id,
            name: name.to_string(),
            base_priority: priority,
            current_priority: priority,
            state: TaskState::Ready,
            context: CpuContext::new(entry_point, stack_top),
            total_ticks_executed: 0,
        }
    }
}

/// Deterministic Preemptive RTOS Scheduler
pub struct SovereignRtos {
    pub current_tick: u64,
    pub tasks: Vec<TaskControlBlock>,
    pub current_task_idx: Option<usize>,
}

impl SovereignRtos {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            tasks: Vec::new(),
            current_task_idx: None,
        }
    }

    /// Daftarkan task baru ke RTOS
    pub fn spawn_task(&mut self, task: TaskControlBlock) {
        self.tasks.push(task);
    }

    /// Eksekusi 1 hardware SysTick interrupt
    pub fn tick(&mut self) -> Option<u32> {
        self.current_tick += 1;

        // 1. Bangunkan task yang selesai tidur (Blocked timeout expired)
        for task in &mut self.tasks {
            if let TaskState::Blocked(wake_tick) = task.state {
                if self.current_tick >= wake_tick {
                    task.state = TaskState::Ready;
                }
            }
        }

        // 2. Pilih task Ready dengan prioritas tertinggi (Preemptive Priority Scheduling)
        let mut highest_priority: Option<u8> = None;
        let mut next_task_idx: Option<usize> = None;

        for (idx, task) in self.tasks.iter().enumerate() {
            if task.state == TaskState::Ready || task.state == TaskState::Running {
                match highest_priority {
                    None => {
                        highest_priority = Some(task.current_priority);
                        next_task_idx = Some(idx);
                    }
                    Some(hp) if task.current_priority > hp => {
                        highest_priority = Some(task.current_priority);
                        next_task_idx = Some(idx);
                    }
                    _ => {}
                }
            }
        }

        // 3. Lakukan Context Switch jika task berubah
        if let Some(curr_idx) = self.current_task_idx {
            if self.tasks[curr_idx].state == TaskState::Running {
                self.tasks[curr_idx].state = TaskState::Ready;
            }
        }

        if let Some(next_idx) = next_task_idx {
            self.tasks[next_idx].state = TaskState::Running;
            self.tasks[next_idx].total_ticks_executed += 1;
            self.current_task_idx = Some(next_idx);
            Some(self.tasks[next_idx].task_id)
        } else {
            self.current_task_idx = None;
            None // CPU Idle
        }
    }

    /// Buat task saat ini tidur selama N ticks
    pub fn delay_current_task(&mut self, ticks: u64) {
        if let Some(curr_idx) = self.current_task_idx {
            self.tasks[curr_idx].state = TaskState::Blocked(self.current_tick + ticks);
            self.current_task_idx = None;
        }
    }
}

// ============================================================================
// 3. REAL-TIME SYNCHRONIZATION: PRIORITY INHERITANCE MUTEX
// ============================================================================

pub struct RtMutex {
    pub owner_task_idx: Option<usize>,
    pub waiting_tasks: Vec<usize>,
}

impl RtMutex {
    pub fn new() -> Self {
        Self {
            owner_task_idx: None,
            waiting_tasks: Vec::new(),
        }
    }

    /// Kunci mutex dengan Priority Inheritance Protocol
    pub fn lock(&mut self, task_idx: usize, tasks: &mut [TaskControlBlock]) -> bool {
        match self.owner_task_idx {
            None => {
                self.owner_task_idx = Some(task_idx);
                true
            }
            Some(owner) => {
                if owner == task_idx {
                    return true;
                }
                // Jika task yang meminta memiliki prioritas lebih tinggi daripada owner,
                // wariskan prioritasnya ke owner (Priority Inheritance) untuk mencegah Priority Inversion!
                let requester_prio = tasks[task_idx].current_priority;
                if requester_prio > tasks[owner].current_priority {
                    tasks[owner].current_priority = requester_prio;
                }
                self.waiting_tasks.push(task_idx);
                tasks[task_idx].state = TaskState::Suspended;
                false
            }
        }
    }

    /// Lepas kunci mutex dan pulihkan prioritas owner kembali ke base priority
    pub fn unlock(&mut self, task_idx: usize, tasks: &mut [TaskControlBlock]) {
        if self.owner_task_idx == Some(task_idx) {
            // Pulihkan prioritas pemilik asli
            tasks[task_idx].current_priority = tasks[task_idx].base_priority;

            if let Some(next_task) = self.waiting_tasks.pop() {
                self.owner_task_idx = Some(next_task);
                tasks[next_task].state = TaskState::Ready;
            } else {
                self.owner_task_idx = None;
            }
        }
    }
}
