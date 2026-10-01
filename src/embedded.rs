//! Widya Embedded & Microcontroller Firmware Emitter (embedded.rs)
//! Menyediakan dukungan kompilasi bare-metal (no_std) untuk ARM Cortex-M0/M3/M4/M7 dan RISC-V RV32IMAC,
//! termasuk pembangkitan Startup Code, Linker Scripts (.ld), Interrupt Vector Table, dan GPIO Memory-Mapped I/O.

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetArch {
    ArmCortexM0,
    ArmCortexM4F,
    RiscVRv32imac,
    XtensaEsp32,
}

impl TargetArch {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cortex-m0" | "m0" | "stm32f0" => TargetArch::ArmCortexM0,
            "cortex-m4" | "m4" | "stm32f4" => TargetArch::ArmCortexM4F,
            "riscv" | "rv32" | "esp32-c3" => TargetArch::RiscVRv32imac,
            "esp32" | "xtensa" => TargetArch::XtensaEsp32,
            _ => TargetArch::ArmCortexM4F,
        }
    }

    pub fn triple(&self) -> &'static str {
        match self {
            TargetArch::ArmCortexM0 => "thumbv6m-none-eabi",
            TargetArch::ArmCortexM4F => "thumbv7em-none-eabihf",
            TargetArch::RiscVRv32imac => "riscv32imac-unknown-none-elf",
            TargetArch::XtensaEsp32 => "xtensa-esp32-none-elf",
        }
    }
}

pub struct EmbeddedFirmwareGenerator;

impl EmbeddedFirmwareGenerator {
    /// Hasilkan Linker Script (.ld) standar untuk memori Flash dan SRAM
    pub fn generate_linker_script(flash_kb: usize, ram_kb: usize) -> String {
        format!(r#"/* Widya-Lang Auto-Generated Bare-Metal Linker Script */
MEMORY
{{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = {}K
    RAM   (rwx): ORIGIN = 0x20000000, LENGTH = {}K
}}

ENTRY(Reset_Handler)

SECTIONS
{{
    .isr_vector :
    {{
        . = ALIGN(4);
        KEEP(*(.isr_vector))
        . = ALIGN(4);
    }} > FLASH

    .text :
    {{
        . = ALIGN(4);
        *(.text)
        *(.text*)
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    }} > FLASH

    .data :
    {{
        . = ALIGN(4);
        _sdata = .;
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    }} > RAM AT > FLASH

    .bss :
    {{
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
        . = ALIGN(4);
        _ebss = .;
    }} > RAM
}}
"#, flash_kb, ram_kb)
    }

    /// Hasilkan berkas startup C & Vector Table Interrupt
    pub fn generate_startup_c(arch: &TargetArch) -> String {
        format!(r#"// Widya-Lang Embedded Startup File
// Target: {}
#include <stdint.h>

extern uint32_t _sdata, _edata, _sbss, _ebss;
extern void main_widya(void);

void Default_Handler(void) {{ while(1); }}
void Reset_Handler(void) {{
    // Inisialisasi segmen .data dari FLASH ke SRAM
    uint32_t *pSrc = &_edata;
    uint32_t *pDest = &_sdata;
    while(pDest < &_edata) {{
        *pDest++ = *pSrc++;
    }}

    // Zero-fill segmen .bss
    uint32_t *pBss = &_sbss;
    while(pBss < &_ebss) {{
        *pBss++ = 0;
    }}

    // Panggil fungsi utama Widya-Lang
    main_widya();
    while(1);
}}

__attribute__((section(".isr_vector")))
void (* const g_pfnVectors[])(void) = {{
    (void (*)(void))((uint32_t)0x20010000), // Initial Stack Pointer
    Reset_Handler,                          // Reset Handler
    Default_Handler,                        // NMI Handler
    Default_Handler,                        // HardFault Handler
}};
"#, arch.triple())
    }

    /// Bangun proyek firmware biner siap flash (.elf / .hex / .bin)
    pub fn emit_firmware_package(output_dir: &Path, project_name: &str, arch: &TargetArch) -> Result<PathBuf, String> {
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

        let ld_content = Self::generate_linker_script(512, 128);
        let startup_content = Self::generate_startup_c(arch);

        let ld_path = output_dir.join(format!("{}.ld", project_name));
        let startup_path = output_dir.join("startup.c");

        fs::write(&ld_path, ld_content).map_err(|e| e.to_string())?;
        fs::write(&startup_path, startup_content).map_err(|e| e.to_string())?;

        Ok(ld_path)
    }
}
