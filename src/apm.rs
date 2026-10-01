//! WidyaTrace — Sovereign APM & Distributed Tracing Engine (OpenTelemetry / Grafana / Datadog Equivalent)
//! Provides W3C TraceContext distributed tracing, span latency tracking, dependency graph extraction,
//! and real-time visual waterfall flamegraph generation.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanStatus {
    Ok,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub service_name: String,
    pub start_time_us: u64,
    pub duration_us: u64,
    pub status: SpanStatus,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub trace_id: String,
    pub spans: Vec<Span>,
}

pub struct SovereignApmEngine {
    traces: Arc<RwLock<HashMap<String, Vec<Span>>>>,
    max_stored_traces: usize,
}

impl SovereignApmEngine {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(RwLock::new(HashMap::new())),
            max_stored_traces: 10_000,
        }
    }

    pub fn generate_trace_id() -> String {
        format!("{:032x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())
    }

    pub fn generate_span_id() -> String {
        format!("{:016x}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 0xFFFFFFFFFFFFFFFF)
    }

    pub fn record_span(&self, span: Span) {
        let mut traces = self.traces.write().unwrap();
        let entry = traces.entry(span.trace_id.clone()).or_insert_with(Vec::new);
        entry.push(span);
    }

    pub fn get_trace(&self, trace_id: &str) -> Option<Trace> {
        let traces = self.traces.read().unwrap();
        traces.get(trace_id).map(|spans| Trace {
            trace_id: trace_id.to_string(),
            spans: spans.clone(),
        })
    }

    pub fn calculate_metrics(&self, service_name: &str) -> (usize, u64, u64, f64) {
        let traces = self.traces.read().unwrap();
        let mut durations: Vec<u64> = Vec::new();
        let mut errors = 0;

        for spans in traces.values() {
            for span in spans {
                if span.service_name == service_name {
                    durations.push(span.duration_us);
                    if let SpanStatus::Error(_) = &span.status {
                        errors += 1;
                    }
                }
            }
        }

        if durations.is_empty() {
            return (0, 0, 0, 0.0);
        }

        durations.sort_unstable();
        let count = durations.len();
        let p50 = durations[count * 50 / 100];
        let p95 = durations[count * 95 / 100];
        let error_rate = (errors as f64 / count as f64) * 100.0;

        (count, p50, p95, error_rate)
    }

    pub fn generate_flamegraph_html(&self, trace_id: &str) -> Result<String, String> {
        let trace = self.get_trace(trace_id).ok_or_else(|| format!("Trace ID '{}' tidak ditemukan", trace_id))?;
        
        let mut rows = String::new();
        let min_start = trace.spans.iter().map(|s| s.start_time_us).min().unwrap_or(0);
        let max_end = trace.spans.iter().map(|s| s.start_time_us + s.duration_us).max().unwrap_or(1);
        let total_duration = (max_end - min_start).max(1);

        for span in &trace.spans {
            let left_percent = ((span.start_time_us - min_start) as f64 / total_duration as f64) * 100.0;
            let width_percent = ((span.duration_us as f64 / total_duration as f64) * 100.0).max(1.0);
            let color = match span.status {
                SpanStatus::Ok => "#38bdf8",
                SpanStatus::Error(_) => "#ef4444",
            };

            rows.push_str(&format!(
                r#"<div style="margin-bottom: 8px;">
                    <div style="font-size: 12px; color: #94a3b8; display: flex; justify-content: space-between;">
                        <span><strong>[{service}]</strong> {name}</span>
                        <span>{dur} ms</span>
                    </div>
                    <div style="background: #1e293b; height: 18px; border-radius: 4px; position: relative; overflow: hidden; margin-top: 2px;">
                        <div style="position: absolute; left: {left:.2}%; width: {width:.2}%; height: 100%; background: {color}; border-radius: 4px;"></div>
                    </div>
                </div>"#,
                service = span.service_name,
                name = span.name,
                dur = span.duration_us as f64 / 1000.0,
                left = left_percent,
                width = width_percent,
                color = color
            ));
        }

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>WidyaTrace Flamegraph — {trace_id}</title>
    <style>
        body {{ background: #0b0f19; color: #f8fafc; font-family: ui-monospace, monospace; padding: 24px; margin: 0; }}
        .card {{ background: #0f172a; border: 1px solid #1e293b; border-radius: 12px; padding: 20px; }}
    </style>
</head>
<body>
    <div class="card">
        <h2 style="margin-top: 0; color: #38bdf8;">⚡ WidyaTrace Distributed APM</h2>
        <p style="color: #64748b; font-size: 13px;">Trace ID: <code>{trace_id}</code> | Total Waktu: {total_dur:.2} ms</p>
        <hr style="border-color: #1e293b; margin: 16px 0;">
        {rows}
    </div>
</body>
</html>"#,
            trace_id = trace_id,
            total_dur = total_duration as f64 / 1000.0,
            rows = rows
        ))
    }
}

/// OpenTelemetry W3C TraceContext Propagator
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct W3cTraceContext {
    pub version: String,
    pub trace_id: String,
    pub parent_id: String,
    pub trace_flags: String,
}

impl W3cTraceContext {
    pub fn new(trace_id: &str, span_id: &str) -> Self {
        Self {
            version: "00".to_string(),
            trace_id: trace_id.to_string(),
            parent_id: span_id.to_string(),
            trace_flags: "01".to_string(), // Sampled
        }
    }

    /// Format ke header HTTP `traceparent: 00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01`
    pub fn to_traceparent_header(&self) -> String {
        format!("{}-{}-{}-{}", self.version, self.trace_id, self.parent_id, self.trace_flags)
    }

    /// Parsing dari header HTTP `traceparent`
    pub fn from_traceparent_header(header_val: &str) -> Result<Self, String> {
        let parts: Vec<&str> = header_val.split('-').collect();
        if parts.len() != 4 {
            return Err("Format header traceparent tidak valid (harus 4 segmen terpisah '-')".to_string());
        }

        Ok(Self {
            version: parts[0].to_string(),
            trace_id: parts[1].to_string(),
            parent_id: parts[2].to_string(),
            trace_flags: parts[3].to_string(),
        })
    }
}

/// Prometheus Metric Exporter
pub struct PrometheusExporter {
    counters: HashMap<String, (f64, String)>, // nama -> (nilai, deskripsi)
    gauges: HashMap<String, (f64, String)>,
}

impl PrometheusExporter {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            gauges: HashMap::new(),
        }
    }

    pub fn inc_counter(&mut self, name: &str, delta: f64, help: &str) {
        let entry = self.counters.entry(name.to_string()).or_insert((0.0, help.to_string()));
        entry.0 += delta;
    }

    pub fn set_gauge(&mut self, name: &str, value: f64, help: &str) {
        self.gauges.insert(name.to_string(), (value, help.to_string()));
    }

    /// Generate format eksposisi standar Prometheus `/metrics`
    pub fn render_metrics(&self) -> String {
        let mut out = String::new();

        for (name, (val, help)) in &self.counters {
            out.push_str(&format!("# HELP {} {}\n", name, help));
            out.push_str(&format!("# TYPE {} counter\n", name));
            out.push_str(&format!("{} {}\n\n", name, val));
        }

        for (name, (val, help)) in &self.gauges {
            out.push_str(&format!("# HELP {} {}\n", name, help));
            out.push_str(&format!("# TYPE {} gauge\n", name));
            out.push_str(&format!("{} {}\n\n", name, val));
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_w3c_traceparent_header_roundtrip() {
        let ctx = W3cTraceContext::new("4bf92f3577b34da6a3ce929d0e0e4736", "00f067aa0ba902b7");
        let header = ctx.to_traceparent_header();
        assert_eq!(header, "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01");

        let parsed = W3cTraceContext::from_traceparent_header(&header).unwrap();
        assert_eq!(parsed.trace_id, "4bf92f3577b34da6a3ce929d0e0e4736");
        assert_eq!(parsed.parent_id, "00f067aa0ba902b7");
    }

    #[test]
    fn test_prometheus_metrics_export() {
        let mut prom = PrometheusExporter::new();
        prom.inc_counter("widya_http_requests_total", 42.0, "Total transaksi HTTP yang diproses");
        prom.set_gauge("widya_active_connections", 15.0, "Jumlah koneksi aktif saat ini");

        let rendered = prom.render_metrics();
        assert!(rendered.contains("# TYPE widya_http_requests_total counter"));
        assert!(rendered.contains("widya_http_requests_total 42"));
        assert!(rendered.contains("# TYPE widya_active_connections gauge"));
        assert!(rendered.contains("widya_active_connections 15"));
    }
}

