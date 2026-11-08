# 🚀 Widya-Lang: Panduan Penerapan Produksi (Production Deployment Guide)

Panduan deployment resmi untuk menjalankan aplikasi dan layanan mikro Widya-Lang di lingkungan produksi perusahaan berskala besar (*enterprise-grade*).

---

## 1. Opsi Arsitektur Deployment

Widya-Lang mendukung 4 pola penerapan produksi utama:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        POLA PENERAPAN WIDYA                            │
├───────────────────┬───────────────────┬────────────────┬───────────────┤
│ 1. Docker/K8s     │ 2. MicroVM /      │ 3. Serverless  │ 4. Native     │
│    Container      │    Unikernel      │    Edge WASI   │    Bare-Metal │
├───────────────────┼───────────────────┼────────────────┼───────────────┤
│ Linux Container   │ AWS Firecracker   │ Cloudflare /   │ Bare-Metal    │
│ Kubernetes Pod    │ QEMU direct-boot  │ Fastly Wasm    │ Linux ELF-64  │
│ 15 MB distroless  │ <5 MB memory      │ <2µs snapshot  │ Zero-libc     │
└───────────────────┴───────────────────┴────────────────┴───────────────┘
```

---

## 2. Pola 1: Container Docker & Kubernetes

Karena Widya-Lang dapat mengemisikan biner mandiri statis, ukuran container Docker dapat dipangkas drastis menjadi kurang dari 15 MB menggunakan citra dasar `scratch` atau `distroless`.

### Dockerfile Multi-Stage:
```dockerfile
# Tahap 1: Kompilasi Biner Mandiri
FROM rust:1.80-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin widya

# Tahap 2: Citra Produksi Minimalis
FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /
COPY --from=builder /app/target/release/widya /usr/local/bin/widya
COPY projects/enterprise_microservice/Widya.toml /etc/widya/Widya.toml

USER nonroot:nonroot
EXPOSE 8080 8085

ENTRYPOINT ["/usr/local/bin/widya", "jalankan", "/etc/widya/Widya.toml"]
```

### Kubernetes Deployment & Service Manifest:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: widya-microservice
  labels:
    app: widya-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: widya-service
  template:
    metadata:
      labels:
        app: widya-service
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "8080"
        prometheus.io/path: "/metrics"
    spec:
      containers:
      - name: widya-core
        image: registry.widya-lang.id/prod/enterprise-service:1.0.0
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 8085
          name: mcp
        resources:
          limits:
            cpu: "2"
            memory: 256Mi
          requests:
            cpu: "250m"
            memory: 64Mi
        livenessProbe:
          httpGet:
            path: /kesehatan
            port: 8080
          initialDelaySeconds: 2
          periodSeconds: 5
        readinessProbe:
          httpGet:
            path: /kesehatan
            port: 8080
          initialDelaySeconds: 1
          periodSeconds: 3
```

---

## 3. Pola 2: Unikernel MicroVM Direct-Boot (AWS Firecracker / QEMU)

Widya-Lang dapat dikompilasi langsung menjadi citra unikernel berukuran kecil yang dapat diboot langsung oleh hypervisor tanpa sistem operasi Linux lengkap:

```bash
# 1. Bangun citra unikernel mandiri
widya unikernel bangun proyek.wya -o kernel.img --protocol direct-kernel --arch x86_64

# 2. Boot langsung menggunakan QEMU microvm
qemu-system-x86_64 \
    -M microvm \
    -kernel kernel.img \
    -m 64M \
    -nographic \
    -serial mon:stdio
```

Waktu boot tercapai dalam **< 10 milidetik**, ideal untuk lingkungan multi-tenant dengan isolasi keamanan hardware penuh.

---

## 4. Pola 3: Serverless Edge (WASI 0.2 Component Model)

Untuk deployment edge di WasmEdge, Cloudflare Workers, atau Fastly:

```bash
# 1. Kompilasi ke WASI Preview 2 Component
widya wasm-component bangun layanan.wya -o layanan.wasm

# 2. Eksekusi menggunakan runtime WASI 0.2
wasmtime run --wasi preview2 layanan.wasm
```

---

## 5. Pemantauan & Observabilitas Produksi

1. **Prometheus Metrics:** Layanan mengekspor metrik `/metrics` otomatis yang siap discrape oleh Prometheus atau Grafana Agent.
2. **OpenTelemetry Distributed Tracing:** Kirim trace spans langsung ke Jaeger atau Otel Collector melalui W3C trace context header `traceparent`.
3. **Audit Kepatuhan Lisensi & CVE:** Otomatisasi CI/CD menggunakan `widya sbom audit` untuk memastikan zero-trust supply chain compliance.
