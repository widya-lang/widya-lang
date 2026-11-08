# 🧠 @widya/ai (Widya-Torch)

> **Mesin Diferensiasi Otomatis (Autograd) & Lapisan Deep Learning untuk Widya-Lang.**  
> *Setara fungsional dan berkinerja setara PyTorch dalam ekosistem Python.*

---

## 🚀 Fitur Utama
- **Graf Komputasi Dinamis:** Simpul `variabel(nilai, butuh_grad)`, operasi maju `tambah_var`, `kali_var`.
- **Perambatan Mundur Gradien:** `mundur(akar_graf)` (Reverse-Mode Automatic Differentiation).
- **Lapisan Jaringan Saraf:** `lapisan_linear(bobot, bias)`, `linear_maju(layer, x)`.
- **Fungsi Aktivasi & Rugi:** `relu(v)`, `mse_loss(prediksi, target)`.
- **Pengoptimal:** `sgd_langkah(params, learning_rate)` dengan zero-grad otomatis.

## 📦 Pemasangan via WPM
```bash
widya wpm add @widya/ai
```

## 💻 Contoh Penggunaan
```widya
impor "@widya/ai" sebagai torch;

// 1. Buat Lapisan Linear: y = w * x + b
misal layer = torch.lapisan_linear(2.0, 1.0);
misal x = torch.variabel(3.0, false);

// 2. Forward Pass: y = 2*3 + 1 = 7.0
misal y_pred = torch.linear_maju(layer, x);

// 3. Loss MSE (Target = 10.0) -> Loss = 0.5 * (7 - 10)^2 = 4.5
misal rugi = torch.mse_loss(y_pred, 10.0);

// 4. Backward Pass (Hitung Turunan dLoss/dw dan dLoss/db)
torch.mundur(rugi);

cetak("Nilai Prediksi:", y_pred.data);
cetak("Gradien Bobot:", layer.bobot.grad);

// 5. Langkah Optimasi SGD
torch.sgd_langkah([layer.bobot, layer.bias], 0.1);
cetak("Bobot Baru:", layer.bobot.data);
```

