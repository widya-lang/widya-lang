use widya::sys::pure_rtos::{
    ArmCortexMVectorTable, RiscvVectorTable, RtMutex, SovereignRtos, TaskControlBlock, TaskState,
};

#[test]
fn test_arm_cortex_m_vector_table_binary_layout() {
    let initial_sp = 0x20020000;
    let reset_addr = 0x08000100;

    let mut vt = ArmCortexMVectorTable::new(initial_sp, reset_addr);
    vt.set_systick_handler(0x08000200);
    vt.set_pendsv_handler(0x08000300);
    vt.add_peripheral_irq(0x08000400); // IRQ 0

    let bin = vt.to_binary();
    // 16 system vectors + 1 peripheral IRQ = 17 words * 4 bytes = 68 bytes
    assert_eq!(bin.len(), 68);

    // Initial SP at offset 0
    let sp = u32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]);
    assert_eq!(sp, initial_sp);

    // Reset Handler at offset 4 (bit 0 set for Thumb mode: 0x08000101)
    let reset = u32::from_le_bytes([bin[4], bin[5], bin[6], bin[7]]);
    assert_eq!(reset, 0x08000101);

    // SysTick at offset 15 * 4 = 60
    let systick = u32::from_le_bytes([bin[60], bin[61], bin[62], bin[63]]);
    assert_eq!(systick, 0x08000201);

    // IRQ 0 at offset 16 * 4 = 64
    let irq0 = u32::from_le_bytes([bin[64], bin[65], bin[66], bin[67]]);
    assert_eq!(irq0, 0x08000401);
}

#[test]
fn test_riscv_vector_table_and_mtvec() {
    let base = 0x80000000;
    let vt_direct = RiscvVectorTable::new(base, false);
    assert_eq!(vt_direct.mtvec_val(), 0x80000000); // Mode 0 = Direct

    let vt_vectored = RiscvVectorTable::new(base, true);
    assert_eq!(vt_vectored.mtvec_val(), 0x80000001); // Mode 1 = Vectored

    let bin = vt_vectored.to_binary();
    assert_eq!(bin.len(), 64); // 16 * 4 bytes
}

#[test]
fn test_sovereign_rtos_preemptive_scheduling_and_delays() {
    let mut rtos = SovereignRtos::new();

    // Task 1: Low Priority (1)
    let t1 = TaskControlBlock::new(1, "LowPrioTask", 1, 0x08001000, 0x20001000);
    // Task 2: High Priority (5)
    let t2 = TaskControlBlock::new(2, "HighPrioTask", 5, 0x08002000, 0x20002000);

    rtos.spawn_task(t1);
    rtos.spawn_task(t2);

    // Tick 1: High Priority Task (Task 2) must be scheduled immediately
    let scheduled_id = rtos.tick();
    assert_eq!(scheduled_id, Some(2));
    assert_eq!(rtos.tasks[1].state, TaskState::Running);

    // High Priority Task now delays for 3 ticks
    rtos.delay_current_task(3);
    assert!(matches!(rtos.tasks[1].state, TaskState::Blocked(_)));

    // Tick 2: Since Task 2 is Blocked, Task 1 (Low Priority) runs
    let scheduled_id = rtos.tick();
    assert_eq!(scheduled_id, Some(1));
    assert_eq!(rtos.tasks[0].state, TaskState::Running);

    // Tick 3: Task 1 continues running
    let scheduled_id = rtos.tick();
    assert_eq!(scheduled_id, Some(1));

    // Tick 4: Timeout expired! Task 2 wakes up and immediately PREEMPTS Task 1!
    let scheduled_id = rtos.tick();
    assert_eq!(scheduled_id, Some(2));
    assert_eq!(rtos.tasks[1].state, TaskState::Running);
    assert_eq!(rtos.tasks[0].state, TaskState::Ready); // Task 1 preempted back to Ready
}

#[test]
fn test_priority_inheritance_mutex() {
    let mut tasks = vec![
        TaskControlBlock::new(1, "LowPrio", 1, 0x1000, 0x2000),   // Index 0, prio 1
        TaskControlBlock::new(2, "HighPrio", 10, 0x3000, 0x4000), // Index 1, prio 10
    ];

    let mut mutex = RtMutex::new();

    // 1. Low Priority task acquires mutex
    let acquired = mutex.lock(0, &mut tasks);
    assert!(acquired);
    assert_eq!(mutex.owner_task_idx, Some(0));

    // 2. High Priority task attempts to acquire mutex -> blocked!
    let acquired_high = mutex.lock(1, &mut tasks);
    assert!(!acquired_high);

    // Priority Inheritance: Low priority task inherits priority 10 to prevent priority inversion!
    assert_eq!(tasks[0].current_priority, 10);
    assert_eq!(tasks[1].state, TaskState::Suspended);

    // 3. Low Priority task finishes critical section and releases mutex
    mutex.unlock(0, &mut tasks);

    // Owner priority restored to base priority (1)
    assert_eq!(tasks[0].current_priority, 1);
    // Mutex handed over to High Priority task, which becomes Ready
    assert_eq!(mutex.owner_task_idx, Some(1));
    assert_eq!(tasks[1].state, TaskState::Ready);
}
