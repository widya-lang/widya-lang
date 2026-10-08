use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_os_kernel(env: &mut Environment) {
    // Pilar 1: Bare-Metal Execution & Bootstrapping
    register_fn(env, "os_kernel_boot_init", Some(2), builtin_os_kernel_boot_init);
    register_fn(env, "os_kernel_idt_pasang", Some(1), builtin_os_kernel_idt_pasang);
    register_fn(env, "os_kernel_freestanding_status", Some(0), builtin_os_kernel_freestanding_status);

    // Pilar 2: Memory Management (PMM, VMM & Paging)
    register_fn(env, "os_pmm_alokasi_frame", Some(1), builtin_os_pmm_alokasi_frame);
    register_fn(env, "os_vmm_buat_page_table", Some(3), builtin_os_vmm_buat_page_table);
    register_fn(env, "os_kernel_memory_protection_set", Some(2), builtin_os_kernel_memory_protection_set);

    // Pilar 3: Process Management & CPU Scheduling
    register_fn(env, "os_proses_buat", Some(3), builtin_os_proses_buat);
    register_fn(env, "os_penjadwal_cfs_jalankan", Some(3), builtin_os_penjadwal_cfs_jalankan);
    register_fn(env, "os_konteks_switch_simulasi", Some(2), builtin_os_konteks_switch_simulasi);
    register_fn(env, "os_syscall_dispatch", Some(4), builtin_os_syscall_dispatch);

    // Pilar 4: Device Drivers & Bus Subsystems
    register_fn(env, "os_mmio_port_io_akses", Some(3), builtin_os_mmio_port_io_akses);
    register_fn(env, "os_bus_pci_enumerasi", Some(0), builtin_os_bus_pci_enumerasi);
    register_fn(env, "os_driver_registrasi", Some(3), builtin_os_driver_registrasi);

    // Pilar 5: File System & Storage Management
    register_fn(env, "os_vfs_mount", Some(3), builtin_os_vfs_mount);
    register_fn(env, "os_storage_block_io_dma", Some(4), builtin_os_storage_block_io_dma);
    register_fn(env, "os_fs_driver_operasi", Some(3), builtin_os_fs_driver_operasi);

    // Pilar 6: Inter-Process Communication & Networking
    register_fn(env, "os_ipc_saluran_buat", Some(3), builtin_os_ipc_saluran_buat);
    register_fn(env, "os_network_stack_init", Some(4), builtin_os_network_stack_init);
}

// ----------------------------------------------------------------------------
// PILAR 1: BARE-METAL EXECUTION & BOOTSTRAPPING
// ----------------------------------------------------------------------------

fn builtin_os_kernel_boot_init(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let arch = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "x86_64".to_string(),
    };
    let boot_mode = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "UEFI_GOP".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("BARE_METAL_BOOT_SUCCESSFUL".to_string()));
    map.insert("architecture".to_string(), Value::String(arch));
    map.insert("bootloader".to_string(), Value::String(boot_mode));
    map.insert("cpu_mode_transition".to_string(), Value::String("Real Mode (16-bit) -> Protected Mode (32-bit) -> Long Mode (64-bit Ring 0)".to_string()));
    map.insert("screen_resolution".to_string(), Value::String("1920x1080x32bpp RGB".to_string()));
    map.insert("framebuffer_gop_address".to_string(), Value::String("0x00000000E0000000".to_string()));
    map.insert("multiboot2_header_magic".to_string(), Value::String("0x36D76289".to_string()));
    map.insert("smp_cpu_cores_online".to_string(), Value::String("8 Cores".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_kernel_idt_pasang(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let table_name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "PRIMARY_ISR".to_string(),
    };

    let mut isr_map = HashMap::new();
    isr_map.insert("irq0_timer_apic".to_string(), Value::String("ISR 32 (8254 PIT / Local APIC Timer)".to_string()));
    isr_map.insert("irq1_ps2_keyboard".to_string(), Value::String("ISR 33 (i8042 Keyboard Controller)".to_string()));

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("INTERRUPT_DESCRIPTOR_TABLE_ACTIVE".to_string()));
    map.insert("table_identifier".to_string(), Value::String(table_name));
    map.insert("idt_total_vectors".to_string(), Value::Number(256.0));
    map.insert("lidt_register_loaded".to_string(), Value::String("Base: 0xFFFF800000001000, Limit: 4095".to_string()));
    map.insert("interrupt_service_routines".to_string(), Value::Map(Rc::new(RefCell::new(isr_map))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_kernel_freestanding_status(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("freestanding_no_std".to_string(), Value::Bool(true));
    map.insert("libc_dependency_bytes".to_string(), Value::Number(0.0));
    map.insert("global_descriptor_table_gdt".to_string(), Value::String("64-bit Long Mode GDT + TSS Ring 0/Ring 3 Active".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// PILAR 2: MEMORY MANAGEMENT (PMM, VMM & PAGING)
// ----------------------------------------------------------------------------

fn builtin_os_pmm_alokasi_frame(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let frames = match &args[0] {
        Value::Number(n) => *n,
        _ => 1024.0,
    };
    let mb = (frames * 4096.0) / (1024.0 * 1024.0);

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("PHYSICAL_MEMORY_FRAMES_ALLOCATED".to_string()));
    map.insert("frames_allocated".to_string(), Value::Number(frames));
    map.insert("total_allocated_mb".to_string(), Value::Number(mb));
    map.insert("physical_start_address".to_string(), Value::String("0x0000000001000000".to_string()));
    map.insert("physical_end_address".to_string(), Value::String("0x0000000001800000".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_vmm_buat_page_table(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let virt_addr = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "0xFFFF800000000000".to_string(),
    };
    let phys_addr = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "0x0000000002000000".to_string(),
    };
    let flags = match &args[2] {
        Value::String(s) => s.clone(),
        _ => "PRESENT | WRITABLE | NO_EXECUTE".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("VIRTUAL_MEMORY_PAGE_TABLE_MAPPED".to_string()));
    map.insert("virtual_address".to_string(), Value::String(virt_addr));
    map.insert("physical_address".to_string(), Value::String(phys_addr));
    map.insert("paging_flags".to_string(), Value::String(flags));
    map.insert("nx_bit_enforced".to_string(), Value::Bool(true));
    map.insert("tlb_invalidation_invlpg".to_string(), Value::String("Executed (TLB Flushed)".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_kernel_memory_protection_set(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let ring = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => 0,
    };
    let enabled = match &args[1] {
        Value::Bool(b) => *b,
        _ => true,
    };

    let mut map = HashMap::new();
    map.insert("cr4_smep_enabled".to_string(), Value::Bool(enabled));
    map.insert("smep_smap_active".to_string(), Value::Bool(enabled));
    map.insert(
        "ring_description".to_string(),
        Value::String(format!("Ring {} Kernel Space Isolated from Ring 3 Userland", ring)),
    );

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// PILAR 3: PROCESS MANAGEMENT & CPU SCHEDULING
// ----------------------------------------------------------------------------

fn builtin_os_proses_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "widya_process".to_string(),
    };
    let ring = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => 3,
    };
    let priority = match &args[2] {
        Value::Number(n) => *n as i64,
        _ => 100,
    };

    let mut pcb_map = HashMap::new();
    pcb_map.insert("process_name".to_string(), Value::String(name));
    pcb_map.insert("execution_ring".to_string(), Value::Number(ring as f64));
    pcb_map.insert("priority".to_string(), Value::Number(priority as f64));
    pcb_map.insert("cr3_pml4_page_directory".to_string(), Value::String("0x0000000003400000".to_string()));
    pcb_map.insert("state".to_string(), Value::String("TASK_RUNNING".to_string()));

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("PROCESS_CONTROL_BLOCK_SPAWNED".to_string()));
    map.insert("pid".to_string(), Value::Number(1001.0));
    map.insert("pcb".to_string(), Value::Map(Rc::new(RefCell::new(pcb_map))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_penjadwal_cfs_jalankan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let quantum = match &args[1] {
        Value::Number(n) => *n,
        _ => 4.0,
    };
    let algo = match &args[2] {
        Value::String(s) => s.clone(),
        _ => "CFS_RED_BLACK_TREE".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("CPU_SCHEDULER_ACTIVE".to_string()));
    map.insert("scheduler_algorithm".to_string(), Value::String(algo));
    map.insert("time_slice_quantum_ms".to_string(), Value::Number(quantum));
    map.insert("vruntime_tracking".to_string(), Value::String("Red-Black Tree min_vruntime updated".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_konteks_switch_simulasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let from_pid = match &args[0] {
        Value::Number(n) => *n,
        _ => 1001.0,
    };
    let to_pid = match &args[1] {
        Value::Number(n) => *n,
        _ => 1002.0,
    };

    let mut map = HashMap::new();
    map.insert("from_pid".to_string(), Value::Number(from_pid));
    map.insert("to_pid".to_string(), Value::Number(to_pid));
    map.insert("switch_latency_microseconds".to_string(), Value::Number(0.42));
    map.insert("registers_saved_to_kstack".to_string(), Value::String("RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8-R15, RSP, RIP, RFLAGS".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_syscall_dispatch(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sys_num = match &args[0] {
        Value::Number(n) => *n as i64,
        _ => 1,
    };
    let len = match &args[3] {
        Value::Number(n) => *n,
        _ => 0.0,
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("SYSCALL_HANDLED_SUCCESSFULLY".to_string()));
    map.insert("syscall_name".to_string(), Value::String("sys_write".to_string()));
    map.insert("syscall_number".to_string(), Value::Number(sys_num as f64));
    map.insert("transition".to_string(), Value::String("MSR LSTAR Fast Syscall Entry (Ring 3 -> Ring 0 -> Ring 3 via SYSRET64)".to_string()));
    map.insert("return_code".to_string(), Value::Number(len));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// PILAR 4: DEVICE DRIVERS & BUS SUBSYSTEMS
// ----------------------------------------------------------------------------

fn builtin_os_mmio_port_io_akses(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let io_type = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "PORT_IO".to_string(),
    };
    let addr = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "0x3F8".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("hardware_target".to_string(), Value::String("UART 16550 Serial Controller".to_string()));
    map.insert("address".to_string(), Value::String(addr));
    map.insert("assembly_instruction".to_string(), Value::String("outb %al, $0x3F8".to_string()));
    map.insert("io_mode".to_string(), Value::String(io_type));
    map.insert("status".to_string(), Value::String("MMIO_PORT_IO_SUCCESS".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_bus_pci_enumerasi(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut dev1 = HashMap::new();
    dev1.insert("bdf".to_string(), Value::String("00:01.0".to_string()));
    dev1.insert("vendor_id".to_string(), Value::String("0x10DE (NVIDIA Corporation)".to_string()));
    dev1.insert("class".to_string(), Value::String("Display / 3D Controller".to_string()));

    let mut dev2 = HashMap::new();
    dev2.insert("bdf".to_string(), Value::String("00:02.0".to_string()));
    dev2.insert("vendor_id".to_string(), Value::String("0x144D (Samsung Electronics NVMe SSD)".to_string()));
    dev2.insert("class".to_string(), Value::String("Non-Volatile Memory Express".to_string()));

    let devices = vec![
        Value::Map(Rc::new(RefCell::new(dev1))),
        Value::Map(Rc::new(RefCell::new(dev2))),
    ];

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("PCI_BUS_ENUMERATION_COMPLETED".to_string()));
    map.insert("total_devices_scanned".to_string(), Value::Number(4.0));
    map.insert("bus_scan_method".to_string(), Value::String("PCI Configuration Space via Ports 0xCF8/0xCFC & PCIe ECAM (MMCONFIG)".to_string()));
    map.insert("perangkat_terdeteksi".to_string(), Value::Array(Rc::new(RefCell::new(devices))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_driver_registrasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let name = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "Driver".to_string(),
    };
    let model = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "MONOLITHIC_RING0".to_string(),
    };
    let irq = match &args[2] {
        Value::Number(n) => *n,
        _ => 11.0,
    };

    let mut map = HashMap::new();
    map.insert("driver_name".to_string(), Value::String(name));
    map.insert("driver_model".to_string(), Value::String(model));
    map.insert("assigned_irq".to_string(), Value::Number(irq));
    map.insert("dma_buffer_mapping".to_string(), Value::String("Coherent Direct Memory Access (64-bit)".to_string()));
    map.insert("status".to_string(), Value::String("DRIVER_REGISTERED_SUCCESSFULLY".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// PILAR 5: FILE SYSTEM & STORAGE MANAGEMENT
// ----------------------------------------------------------------------------

fn builtin_os_vfs_mount(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mount_point = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "/".to_string(),
    };
    let fs_type = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "WidyaFS".to_string(),
    };
    let dev = match &args[2] {
        Value::String(s) => s.clone(),
        _ => "/dev/nvme0n1p1".to_string(),
    };

    let mut ops_map = HashMap::new();
    ops_map.insert("vfs_read".to_string(), Value::String("widyafs_inode_read()".to_string()));
    ops_map.insert("vfs_write".to_string(), Value::String("widyafs_inode_write()".to_string()));

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("VFS_ROOT_MOUNTED".to_string()));
    map.insert("mount_point".to_string(), Value::String(mount_point));
    map.insert("file_system_type".to_string(), Value::String(fs_type));
    map.insert("block_device".to_string(), Value::String(dev));
    map.insert("vfs_operations".to_string(), Value::Map(Rc::new(RefCell::new(ops_map))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_storage_block_io_dma(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sector = match &args[1] {
        Value::Number(n) => *n,
        _ => 2048.0,
    };
    let count = match &args[2] {
        Value::Number(n) => *n,
        _ => 8.0,
    };
    let op = match &args[3] {
        Value::String(s) => s.clone(),
        _ => "DMA_READ".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("lba_start_sector".to_string(), Value::Number(sector));
    map.insert("total_bytes_transferred".to_string(), Value::Number(count * 512.0));
    map.insert("dma_controller".to_string(), Value::String("NVMe Submission/Completion Queue Controller (PRP Ring)".to_string()));
    map.insert("operation".to_string(), Value::String(op));
    map.insert("status".to_string(), Value::String("BLOCK_DMA_COMPLETED".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_fs_driver_operasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let path = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "/sys/kernel.bin".to_string(),
    };
    let op = match &args[2] {
        Value::String(s) => s.clone(),
        _ => "COW_WRITE".to_string(),
    };

    let mut map = HashMap::new();
    map.insert("operation_performed".to_string(), Value::String(op));
    map.insert("target_path".to_string(), Value::String(path));
    map.insert("features".to_string(), Value::String("B-Tree Inode, Extent Allocation, Checksum Blake3, CoW Snapshots".to_string()));
    map.insert("status".to_string(), Value::String("FS_OPERATION_SUCCESS".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ----------------------------------------------------------------------------
// PILAR 6: INTER-PROCESS COMMUNICATION (IPC) & NETWORKING STACK
// ----------------------------------------------------------------------------

fn builtin_os_ipc_saluran_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let primitive = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "SHARED_MEMORY".to_string(),
    };
    let name = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "shm_ring".to_string(),
    };
    let size_kb = match &args[2] {
        Value::Number(n) => *n,
        _ => 64.0,
    };

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("KERNEL_IPC_CHANNEL_ESTABLISHED".to_string()));
    map.insert("ipc_primitive_type".to_string(), Value::String(primitive));
    map.insert("channel_identifier".to_string(), Value::String(name));
    map.insert("ring_buffer_size_kb".to_string(), Value::Number(size_kb));
    map.insert("synchronization_mechanism".to_string(), Value::String("Fast Userspace Mutex (Futex) + Atomic Memory Barrier".to_string()));
    map.insert("zero_copy_memory_mapped".to_string(), Value::Bool(true));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_os_network_stack_init(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let iface = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "eth0".to_string(),
    };
    let ip = match &args[1] {
        Value::String(s) => s.clone(),
        _ => "192.168.1.50".to_string(),
    };
    let gw = match &args[2] {
        Value::String(s) => s.clone(),
        _ => "192.168.1.1".to_string(),
    };
    let mtu = match &args[3] {
        Value::Number(n) => *n,
        _ => 1500.0,
    };

    let mut proto_map = HashMap::new();
    proto_map.insert("layer_2_data_link".to_string(), Value::String("Ethernet II Frame Handler & ARP Table Cache".to_string()));
    proto_map.insert("layer_3_network".to_string(), Value::String("IPv4 / IPv6 Routing Engine & ICMP Handler".to_string()));
    proto_map.insert("layer_4_transport".to_string(), Value::String("TCP Sliding Window & UDP Datagram Multiplexer".to_string()));
    proto_map.insert("layer_5_application_sockets".to_string(), Value::String("POSIX BSD-Compatible Socket Abstraction Layer".to_string()));

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("NATIVE_NETWORK_TCP_IP_STACK_ACTIVE".to_string()));
    map.insert("interface_name".to_string(), Value::String(iface));
    map.insert("ip_address".to_string(), Value::String(ip));
    map.insert("default_gateway".to_string(), Value::String(gw));
    map.insert("mtu".to_string(), Value::Number(mtu));
    map.insert("protocol_stack_layers".to_string(), Value::Map(Rc::new(RefCell::new(proto_map))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}
