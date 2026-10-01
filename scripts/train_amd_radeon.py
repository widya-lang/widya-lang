#!/usr/bin/env python3
"""
Widya-Lang AI Fine-Tuning Script untuk AMD Radeon GPU (16GB VRAM)
================================================================
Dirancang khusus sebagai alternatif Unsloth untuk kartu grafis AMD Radeon:
(RX 6800, RX 6800 XT, RX 6900 XT, RX 7800 XT, RX 7900 GRE/XT, Radeon Pro, dll.)

Mendukung 2 Mode:
1. Native Windows dengan Microsoft DirectML (torch-directml)
2. Linux / WSL2 dengan AMD ROCm (PyTorch ROCm)
"""

import os
import sys
import torch
from datasets import load_dataset
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    TrainingArguments,
)
from peft import LoraConfig, get_peft_model, TaskType
from trl import SFTTrainer

# ==========================================
# 1. DETEKSI AKSELERATOR AMD RADEON
# ==========================================
def get_device():
    # Cek ROCm (Di Linux / WSL2 ROCm terdeteksi sebagai CUDA)
    if torch.cuda.is_available():
        device_name = torch.cuda.get_device_name(0)
        print(f"[✓] Menggunakan AMD ROCm GPU: {device_name}")
        return "cuda"
    
    # Cek Microsoft DirectML (Native Windows untuk AMD Radeon)
    try:
        import torch_directml
        if torch_directml.is_available():
            dml_device = torch_directml.device()
            print(f"[✓] Menggunakan AMD DirectML (Native Windows): {torch_directml.device_name(0)}")
            return dml_device
    except ImportError:
        pass

    print("[!] GPU Akselerator tidak ditemukan, menggunakan CPU (Lambat).")
    return "cpu"

DEVICE = get_device()

# ==========================================
# 2. KONFIGURASI MODEL & MEMORI (16GB VRAM)
# ==========================================
# Rekomendasi model yang sangat pas untuk VRAM 16GB:
# Qwen2.5-Coder-7B-Instruct atau Llama-3.2-3B-Instruct
MODEL_ID = "Qwen/Qwen2.5-Coder-7B-Instruct"
OUTPUT_DIR = "widya_model_amd_output"
DATASET_PATH = "widya_ai_dataset.jsonl"
MAX_SEQ_LENGTH = 2048

def main():
    print(f"\n[*] Memuat Tokenizer & Model: {MODEL_ID}...")
    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID, trust_remote_code=True)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    # VRAM 16GB pada AMD Radeon sangat leluasa menjalankan model 7B dengan Float16 / Bfloat16
    torch_dtype = torch.float16
    if isinstance(DEVICE, str) and DEVICE == "cuda" and torch.cuda.is_bf16_supported():
        torch_dtype = torch.bfloat16

    print(f"[*] Memuat bobot model dengan presisi: {torch_dtype}...")
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        torch_dtype=torch_dtype,
        trust_remote_code=True,
        device_map="auto" if DEVICE == "cuda" else None,
        low_cpu_mem_usage=True,
    )

    if DEVICE != "cuda" and DEVICE != "cpu":
        model = model.to(DEVICE)

    # ==========================================
    # 3. KONFIGURASI LORA (PARAMETER EFFICIENT)
    # ==========================================
    print("[*] Memasang modul adapter LoRA...")
    peft_config = LoraConfig(
        task_type=TaskType.CAUSAL_LM,
        r=16,
        lora_alpha=32,
        lora_dropout=0.05,
        target_modules=["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"],
        bias="none",
    )
    model = get_peft_model(model, peft_config)
    model.print_trainable_parameters()

    # ==========================================
    # 4. MEMUAT DATASET WIDYA-LANG
    # ==========================================
    print(f"[*] Membaca berkas dataset: {DATASET_PATH}...")
    def format_chatml(example):
        messages = example["messages"]
        text = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=False)
        return {"text": text}

    dataset = load_dataset("json", data_files=DATASET_PATH, split="train")
    formatted_dataset = dataset.map(format_chatml)

    # ==========================================
    # 5. ARGUMEN TRAINING (OPTIMAL 16GB VRAM)
    # ==========================================
    training_args = TrainingArguments(
        output_dir=OUTPUT_DIR,
        per_device_train_batch_size=1,       # Batch size 1 + Gradient Accumulation 8 = Efektif 8
        gradient_accumulation_steps=8,
        learning_rate=2e-4,
        lr_scheduler_type="cosine",
        warmup_ratio=0.05,
        num_train_epochs=3,
        fp16=(torch_dtype == torch.float16 and DEVICE == "cuda"),
        bf16=(torch_dtype == torch.bfloat16 and DEVICE == "cuda"),
        logging_steps=1,
        save_strategy="epoch",
        optim="adamw_torch",
        weight_decay=0.01,
        report_to="none",
        gradient_checkpointing=True,         # Menghemat VRAM hingga 50%
    )

    print("[*] Memulai pelatihan SFTTrainer pada AMD Radeon...")
    trainer = SFTTrainer(
        model=model,
        tokenizer=tokenizer,
        train_dataset=formatted_dataset,
        dataset_text_field="text",
        max_seq_length=MAX_SEQ_LENGTH,
        args=training_args,
    )

    trainer.train()
    print("\n[✓] Pelatihan SFT selesai dengan sukses!")

    # ==========================================
    # 6. MENYIMPAN ADAPTER LORA HASIL TRAINING
    # ==========================================
    save_path = "widya-coder-lora-amd"
    print(f"[*] Menyimpan bobot adapter ke: {save_path}...")
    model.save_pretrained(save_path)
    tokenizer.save_pretrained(save_path)
    print(f"[✓] Model LoRA tersimpan! Siap dimerge atau diekspor ke format GGUF untuk Ollama.")

if __name__ == "__main__":
    main()
