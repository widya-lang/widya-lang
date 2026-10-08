use std::rc::Rc;
use std::cell::RefCell;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::interpreter::Interpreter;
use widya::environment::Environment;
use widya::error::Span;
use widya::value::Value;

fn execute_code(source: &str) -> (Value, Rc<RefCell<Environment>>) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => panic!("Tokenization error: {:?}", e),
    };
    let mut parser = Parser::new(tokens);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => panic!("Parser error: {:?}", e),
    };
    let mut interp = Interpreter::new();
    let env = interp.environment.clone();
    let res = match interp.interpret(&stmts) {
        Ok(v) => v,
        Err(e) => panic!("Interpreter error: {:?}", e),
    };
    (res, env)
}

#[test]
fn test_widyaos_pillar_1_boot_and_idt() {
    let code = r#"
        var boot = os_kernel_boot_init("x86_64", "UEFI_GOP");
        var idt = os_kernel_idt_pasang("PRIMARY_ISR");
        var free = os_kernel_freestanding_status();
    "#;
    let (_, env) = execute_code(code);
    let span = Span::new(0, 0);
    let boot = env.borrow().get("boot", &span).expect("boot variable");
    if let Value::Map(m) = boot {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "BARE_METAL_BOOT_SUCCESSFUL");
    } else {
        panic!("boot should be Map");
    }

    let idt = env.borrow().get("idt", &span).expect("idt variable");
    if let Value::Map(m) = idt {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "INTERRUPT_DESCRIPTOR_TABLE_ACTIVE");
    } else {
        panic!("idt should be Map");
    }

    let free = env.borrow().get("free", &span).expect("free variable");
    if let Value::Map(m) = free {
        assert_eq!(m.borrow().get("freestanding_no_std").unwrap().to_string_repr(), "benar");
    } else {
        panic!("free should be Map");
    }
}

#[test]
fn test_widyaos_pillar_2_pmm_vmm_paging() {
    let code = r#"
        var pmm = os_pmm_alokasi_frame(1024);
        var vmm = os_vmm_buat_page_table("0xFFFF800000000000", "0x0000000002000000", "PRESENT|WRITABLE|NO_EXECUTE");
        var prot = os_kernel_memory_protection_set(0, benar);
    "#;
    let (_, env) = execute_code(code);
    let span = Span::new(0, 0);
    let pmm = env.borrow().get("pmm", &span).expect("pmm variable");
    if let Value::Map(m) = pmm {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "PHYSICAL_MEMORY_FRAMES_ALLOCATED");
    } else {
        panic!("pmm should be Map");
    }

    let vmm = env.borrow().get("vmm", &span).expect("vmm variable");
    if let Value::Map(m) = vmm {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "VIRTUAL_MEMORY_PAGE_TABLE_MAPPED");
    } else {
        panic!("vmm should be Map");
    }

    let prot = env.borrow().get("prot", &span).expect("prot variable");
    if let Value::Map(m) = prot {
        assert_eq!(m.borrow().get("cr4_smep_enabled").unwrap().to_string_repr(), "benar");
    } else {
        panic!("prot should be Map");
    }
}

#[test]
fn test_widyaos_pillar_3_process_cfs_and_syscalls() {
    let code = r#"
        var p1 = os_proses_buat("srv_worker", 3, 120);
        var sched = os_penjadwal_cfs_jalankan([], 4.0, "CFS_RED_BLACK_TREE");
        var ctx = os_konteks_switch_simulasi(1001, 1002);
        var sys = os_syscall_dispatch(1, "/dev/tty0", "test", 4);
    "#;
    let (_, env) = execute_code(code);
    let span = Span::new(0, 0);
    let p1 = env.borrow().get("p1", &span).expect("p1 variable");
    if let Value::Map(m) = p1 {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "PROCESS_CONTROL_BLOCK_SPAWNED");
    } else {
        panic!("p1 should be Map");
    }

    let sched = env.borrow().get("sched", &span).expect("sched variable");
    if let Value::Map(m) = sched {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "CPU_SCHEDULER_ACTIVE");
    } else {
        panic!("sched should be Map");
    }

    let sys = env.borrow().get("sys", &span).expect("sys variable");
    if let Value::Map(m) = sys {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "SYSCALL_HANDLED_SUCCESSFULLY");
    } else {
        panic!("sys should be Map");
    }
}

#[test]
fn test_widyaos_pillar_4_drivers_pci_and_vfs() {
    let code = r#"
        var io = os_mmio_port_io_akses("PORT_IO", "0x3F8", 65);
        var pci = os_bus_pci_enumerasi();
        var drv = os_driver_registrasi("NVMeDriver", "MONOLITHIC_RING0", 11);
        var vfs = os_vfs_mount("/", "WidyaFS", "/dev/nvme0n1p1");
        var dma = os_storage_block_io_dma("/dev/nvme0n1", 2048, 8, "DMA_READ");
        var fs = os_fs_driver_operasi("WidyaFS", "/sys/kernel.bin", "COW_WRITE");
    "#;
    let (_, env) = execute_code(code);
    let span = Span::new(0, 0);
    let pci = env.borrow().get("pci", &span).expect("pci variable");
    if let Value::Map(m) = pci {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "PCI_BUS_ENUMERATION_COMPLETED");
    } else {
        panic!("pci should be Map");
    }

    let vfs = env.borrow().get("vfs", &span).expect("vfs variable");
    if let Value::Map(m) = vfs {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "VFS_ROOT_MOUNTED");
    } else {
        panic!("vfs should be Map");
    }
}

#[test]
fn test_widyaos_pillar_6_ipc_and_network_stack() {
    let code = r#"
        var ipc = os_ipc_saluran_buat("SHARED_MEMORY", "shm_ring", 64);
        var net = os_network_stack_init("eth0", "192.168.1.50", "192.168.1.1", 1500);
    "#;
    let (_, env) = execute_code(code);
    let span = Span::new(0, 0);
    let ipc = env.borrow().get("ipc", &span).expect("ipc variable");
    if let Value::Map(m) = ipc {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "KERNEL_IPC_CHANNEL_ESTABLISHED");
    } else {
        panic!("ipc should be Map");
    }

    let net = env.borrow().get("net", &span).expect("net variable");
    if let Value::Map(m) = net {
        assert_eq!(m.borrow().get("status").unwrap().to_string_repr(), "NATIVE_NETWORK_TCP_IP_STACK_ACTIVE");
    } else {
        panic!("net should be Map");
    }
}
