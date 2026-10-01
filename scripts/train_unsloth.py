#!/usr/bin/env python3
"""
Widya-Lang AI Fine-Tuning Script using Unsloth
=============================================
Skrip resmi untuk melatih (fine-tune) model LLM (Qwen2.5-Coder / Llama-3)
menggunakan Unsloth 2x-5x lebih cepat dengan konsumsi VRAM 70% lebih hemat.

Dapat dijalankan di:
1. Google Colab (Gratis GPU T4 16GB)
2. Mesin Lokal Linux / Windows WSL2 dengan GPU NVIDIA RTX (6GB - 24GB VRAM)
3. Cloud GPU (RunPod, Lambda, Vast.ai)
"""

import os
import torch
from unsloth import FastLanguageModel
from unsloth.chat_templates import get_chat_template, standardize_sharegpt
from datasets import load_dataset
from trl import SFTTrainer
from transformers import TrainingArguments

# ==========================================
# 1. KONFIGURASI MODEL & TRAINING
# ==========================================
MAX_SEQ_LENGTH = 2048  # Bisa ditingkatkan ke 4096 jika VRAM mencukupi
DTYPE = None           # None = auto detect (Float16 untuk T4, Bfloat16 untuk Ampere/Ada)
LOAD_IN_4BIT = True    # 4-bit QLoRA untuk hemat memori ekstrem

# Base Model Rekomendasi (Paling unggul untuk coding):
MODEL_NAME = "unsloth/Qwen2.5-Coder-7B-Instruct-bnb-4bit"
# Opsi alternatif jika VRAM < 8GB:
# MODEL_NAME = "unsloth/Llama-3.2-3B-Instruct-bnb-4bit"

OUTPUT_DIR = "widya_model_output"
DATASET_PATH = "widya_ai_dataset.jsonl"

def main():
    print(f"[*] Memuat model dasar: {MODEL_NAME} dengan Unsloth...")
    model, tokenizer = FastLanguageModel.from_pretrained(
        model_name = MODEL_NAME,
        max_seq_length = MAX_SEQ_LENGTH,
        dtype = DTYPE,
        load_in_4bit = LOAD_IN_4BIT,
    )

    # ==========================================
    # 2. KONFIGURASI LORA PEFT
    # ==========================================
    print("[*] Mengonfigurasi modul LoRA...")
    model = FastLanguageModel.get_peft_model(
        model,
        r = 16,                # Rank LoRA yang seimbang (8, 16, 32, 64)
        target_modules = [
            "q_proj", "k_proj", "v_proj", "o_proj",
            "gate_proj", "up_proj", "down_proj",
        ],
        lora_alpha = 16,
        lora_dropout = 0,      # 0 optimal untuk unsloth
        bias = "none",
        use_gradient_checkpointing = "unsloth", # 30% penghematan VRAM tambahan
        random_state = 3407,
        use_rslora = False,
        loftq_config = None,
    )

    # ==========================================
    # 3. CHAT TEMPLATE & DATASET
    # ==========================================
    print(f"[*] Memuat dataset: {DATASET_PATH}...")
    tokenizer = get_chat_template(
        tokenizer,
        chat_template = "chatml",  # Format standar untuk Qwen2.5
    )

    def formatting_prompts_func(examples):
        convos = examples["messages"]
        texts = [tokenizer.apply_chat_template(convo, tokenize = False, add_generation_prompt = False) for convo in convos]
        return { "text" : texts }

    dataset = load_dataset("json", data_files = DATASET_PATH, split = "train")
    dataset = dataset.map(formatting_prompts_func, batched = True)

    # ==========================================
    # 4. TRAINING DENGAN SFTTRAINER
    # ==========================================
    print("[*] Memulai proses training SFTTrainer...")
    trainer = SFTTrainer(
        model = model,
        tokenizer = tokenizer,
        train_dataset = dataset,
        dataset_text_field = "text",
        max_seq_length = MAX_SEQ_LENGTH,
        dataset_num_proc = 2,
        packing = False, # Dapat diset True untuk mempercepat dataset pendek
        args = TrainingArguments(
            per_device_train_batch_size = 2,
            gradient_accumulation_steps = 4,
            warmup_steps = 5,
            num_train_epochs = 3,       # 3 epoch cukup untuk mempelajari sintaks
            learning_rate = 2e-4,
            fp16 = not torch.cuda.is_bf16_supported(),
            bf16 = torch.cuda.is_bf16_supported(),
            logging_steps = 1,
            optim = "adamw_8bit",
            weight_decay = 0.01,
            lr_scheduler_type = "linear",
            seed = 3407,
            output_dir = OUTPUT_DIR,
            report_to = "none",
        ),
    )

    trainer_stats = trainer.train()
    print("\n[✓] Training selesai!")
    print(f"[*] Durasi Training: {trainer_stats.metrics['train_runtime']:.2f} detik")

    # ==========================================
    # 5. EKSPOR KE FORMAT GGUF (UNTUK OLLAMA)
    # ==========================================
    print("\n[*] Mengekspor model hasil training ke format GGUF untuk Ollama...")
    # Simpan adapter LoRA terlebih dahulu
    model.save_pretrained("widya-coder-lora")
    tokenizer.save_pretrained("widya-coder-lora")

    # Ekspor kuantisasi 4-bit (q4_k_m) berkecepatan tinggi untuk mesin lokal
    gguf_path = "widya-coder-7b-q4_k_m.gguf"
    model.save_pretrained_gguf(
        "widya-coder-gguf",
        tokenizer,
        quantization_method = "q4_k_m"
    )
    print(f"[✓] Berkas model GGUF berhasil dibuat di: widya-coder-gguf/{gguf_path}")
    print("[*] Siap diintegrasikan ke Ollama di Windows!")

if __name__ == "__main__":
    main()
