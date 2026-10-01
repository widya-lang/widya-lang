#!/usr/bin/env python3
"""
Penggabung LoRA & Konversi ke GGUF (Untuk Ollama di AMD Radeon)
==============================================================
Menggabungkan bobot adapter LoRA dengan model dasar Qwen2.5-Coder
menjadi satu model utuh (Merged Model) yang siap diekspor ke format GGUF.
"""

import os
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from peft import PeftModel

BASE_MODEL = "Qwen/Qwen2.5-Coder-7B-Instruct"
LORA_PATH = "widya-coder-lora-amd"
MERGED_OUTPUT = "widya-coder-merged"

def main():
    print(f"[*] Memuat model dasar: {BASE_MODEL}...")
    base_model = AutoModelForCausalLM.from_pretrained(
        BASE_MODEL,
        torch_dtype=torch.float16,
        low_cpu_mem_usage=True,
        trust_remote_code=True,
    )
    tokenizer = AutoTokenizer.from_pretrained(BASE_MODEL, trust_remote_code=True)

    print(f"[*] Memuat adapter LoRA dari: {LORA_PATH}...")
    model = PeftModel.from_pretrained(base_model, LORA_PATH)

    print("[*] Menggabungkan bobot (Merge and Unload)...")
    merged_model = model.merge_and_unload()

    print(f"[*] Menyimpan model gabungan ke: {MERGED_OUTPUT}...")
    merged_model.save_pretrained(MERGED_OUTPUT, safe_serialization=True)
    tokenizer.save_pretrained(MERGED_OUTPUT)

    print("\n[✓] Sukses! Model hasil training siap dikonversi ke GGUF.")
    print("    Langkah konversi ke GGUF via llama.cpp:")
    print(f"    python llama.cpp/convert_hf_to_gguf.py {MERGED_OUTPUT} --outfile widya-coder-7b-q4_k_m.gguf --outtype q4_k_m")

if __name__ == "__main__":
    main()
