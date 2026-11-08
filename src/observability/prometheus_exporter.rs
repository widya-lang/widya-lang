//! Native OpenTelemetry & Prometheus Metrics Exporter
//! 
//! Fitur Skala Industri:
//! - Prometheus Text Exposition Format v0.0.4 / OpenMetrics compliant
//! - Mendukung Counter, Gauge, Histogram (dengan explicit buckets dan +Inf), Summary
//! - Thread-Safe PrometheusRegistry
//! - Bridge Adapter untuk:
//!   - Widya `MetricsCollector`
//!   - Widya APM Sovereign Tracing (`SovereignApmEngine`)
//!   - WidyaBase `EnterpriseConnectionPool` (`PoolMetrics`)
//! - HTTP `/metrics` request handler hook untuk server web / daemon monitoring

use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;

use crate::observability::metrics::{Metric, MetricType, MetricValue, MetricsCollector};
use crate::apm::SovereignApmEngine;
use widyabase::PoolMetrics;

/// Tipe sampel nilai metrik Prometheus
#[derive(Debug, Clone)]
pub struct PrometheusSample {
    pub suffix: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub value: f64,
}

/// Definisi metrik dalam Registry Prometheus
#[derive(Debug, Clone)]
pub struct PrometheusMetric {
    pub name: String,
    pub help: String,
    pub metric_type: MetricType,
    pub samples: Vec<PrometheusSample>,
}

/// Registry metrik Prometheus skala enterprise
pub struct PrometheusRegistry {
    metrics: RwLock<HashMap<String, PrometheusMetric>>,
}

impl PrometheusRegistry {
    pub fn new() -> Self {
        Self {
            metrics: RwLock::new(HashMap::new()),
        }
    }

    /// Menambahkan atau memperbarui counter
    pub fn register_counter(
        &self,
        name: &str,
        help: &str,
        labels: &[(&str, &str)],
        value: f64,
    ) {
        let mut labels_map = BTreeMap::new();
        for (k, v) in labels {
            labels_map.insert(k.to_string(), v.to_string());
        }

        let mut lock = self.metrics.write().unwrap();
        let entry = lock.entry(name.to_string()).or_insert_with(|| PrometheusMetric {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: MetricType::Counter,
            samples: Vec::new(),
        });

        // Cari sampel yang cocok dengan label yang sama
        if let Some(sample) = entry.samples.iter_mut().find(|s| s.labels == labels_map && s.suffix.is_none()) {
            sample.value = value;
        } else {
            entry.samples.push(PrometheusSample {
                suffix: None,
                labels: labels_map,
                value,
            });
        }
    }

    /// Menambahkan atau memperbarui gauge
    pub fn register_gauge(
        &self,
        name: &str,
        help: &str,
        labels: &[(&str, &str)],
        value: f64,
    ) {
        let mut labels_map = BTreeMap::new();
        for (k, v) in labels {
            labels_map.insert(k.to_string(), v.to_string());
        }

        let mut lock = self.metrics.write().unwrap();
        let entry = lock.entry(name.to_string()).or_insert_with(|| PrometheusMetric {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: MetricType::Gauge,
            samples: Vec::new(),
        });

        if let Some(sample) = entry.samples.iter_mut().find(|s| s.labels == labels_map && s.suffix.is_none()) {
            sample.value = value;
        } else {
            entry.samples.push(PrometheusSample {
                suffix: None,
                labels: labels_map,
                value,
            });
        }
    }

    /// Menambahkan atau memperbarui histogram dengan bucket Prometheus
    pub fn register_histogram(
        &self,
        name: &str,
        help: &str,
        labels: &[(&str, &str)],
        sum: f64,
        count: u64,
        buckets: &[(f64, u64)],
    ) {
        let mut base_labels = BTreeMap::new();
        for (k, v) in labels {
            base_labels.insert(k.to_string(), v.to_string());
        }

        let mut lock = self.metrics.write().unwrap();
        let entry = lock.entry(name.to_string()).or_insert_with(|| PrometheusMetric {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: MetricType::Histogram,
            samples: Vec::new(),
        });

        // Hapus sampel lama dengan base labels yang sama
        entry.samples.retain(|s| {
            let matches_base = base_labels.iter().all(|(k, v)| s.labels.get(k) == Some(v));
            !matches_base
        });

        // 1. Tambahkan bucket samples: name_bucket{le="X"}
        for (upper_bound, cumulative_count) in buckets {
            let mut bucket_labels = base_labels.clone();
            let le_str = if upper_bound.is_infinite() {
                "+Inf".to_string()
            } else {
                format!("{}", upper_bound)
            };
            bucket_labels.insert("le".to_string(), le_str);

            entry.samples.push(PrometheusSample {
                suffix: Some("bucket".to_string()),
                labels: bucket_labels,
                value: *cumulative_count as f64,
            });
        }

        // 2. Tambahkan sum: name_sum
        entry.samples.push(PrometheusSample {
            suffix: Some("sum".to_string()),
            labels: base_labels.clone(),
            value: sum,
        });

        // 3. Tambahkan count: name_count
        entry.samples.push(PrometheusSample {
            suffix: Some("count".to_string()),
            labels: base_labels,
            value: count as f64,
        });
    }

    /// Mengumpulkan metrik dari Widya `MetricsCollector`
    pub fn collect_from_metrics_collector(&self, collector: &MetricsCollector) {
        let metrics = collector.get_all_metrics();
        for m in metrics {
            let labels_vec: Vec<(&str, &str)> = m.labels.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            match m.value {
                MetricValue::Counter(c) => {
                    self.register_counter(&m.name, "Widya collector counter", &labels_vec, c as f64);
                }
                MetricValue::Gauge(g) => {
                    self.register_gauge(&m.name, "Widya collector gauge", &labels_vec, g);
                }
                MetricValue::Histogram { sum, count, buckets } => {
                    self.register_histogram(&m.name, "Widya collector histogram", &labels_vec, sum, count, &buckets);
                }
                MetricValue::Summary { sum, count, quantiles: _ } => {
                    self.register_counter(&format!("{}_sum", m.name), "Widya collector summary sum", &labels_vec, sum);
                    self.register_counter(&format!("{}_count", m.name), "Widya collector summary count", &labels_vec, count as f64);
                }
            }
        }
    }

    /// Mengumpulkan metrik distributed APM tracing dari `SovereignApmEngine`
    pub fn collect_from_apm_engine(&self, apm: &SovereignApmEngine, service_names: &[&str]) {
        for service in service_names {
            let (total_spans, total_duration_us, avg_duration_us, error_rate) = apm.calculate_metrics(service);
            let s_labels = [("service", *service)];

            self.register_counter(
                "widya_apm_spans_total",
                "Total spans recorded by sovereign APM engine",
                &s_labels,
                total_spans as f64,
            );
            self.register_gauge(
                "widya_apm_duration_avg_microseconds",
                "Average span execution duration in microseconds",
                &s_labels,
                avg_duration_us as f64,
            );
            self.register_gauge(
                "widya_apm_error_rate_ratio",
                "Ratio of span execution errors (0.0 to 1.0)",
                &s_labels,
                error_rate,
            );
        }
    }

    /// Mengumpulkan metrik koneksi database dari WidyaBase `PoolMetrics`
    pub fn collect_from_db_pool(&self, pool_name: &str, pool_metrics: &PoolMetrics) {
        let p_labels = [("pool", pool_name)];

        self.register_gauge(
            "widyabase_pool_active_connections",
            "Current number of actively acquired database connections",
            &p_labels,
            pool_metrics.active_connections as f64,
        );
        self.register_gauge(
            "widyabase_pool_idle_connections",
            "Current number of idle database connections ready in the pool",
            &p_labels,
            pool_metrics.idle_connections as f64,
        );
        self.register_counter(
            "widyabase_pool_connections_created_total",
            "Total number of database connections created since startup",
            &p_labels,
            pool_metrics.total_created as f64,
        );
        self.register_counter(
            "widyabase_pool_connections_recycled_total",
            "Total number of database connections recycled and reused",
            &p_labels,
            pool_metrics.total_recycled as f64,
        );
        self.register_counter(
            "widyabase_pool_connections_evicted_total",
            "Total number of database connections evicted due to timeout/unhealthiness",
            &p_labels,
            pool_metrics.total_evicted as f64,
        );
        self.register_counter(
            "widyabase_pool_acquisitions_total",
            "Total connection acquisition requests",
            &p_labels,
            pool_metrics.total_acquisitions as f64,
        );
        self.register_counter(
            "widyabase_pool_timeouts_total",
            "Total connection acquisition timeout failures",
            &p_labels,
            pool_metrics.total_timeouts as f64,
        );
        self.register_counter(
            "widyabase_pool_health_check_failures_total",
            "Total health check ping heartbeat failures",
            &p_labels,
            pool_metrics.total_health_check_failures as f64,
        );
        self.register_gauge(
            "widyabase_pool_average_acquire_time_ms",
            "Exponential moving average connection acquisition latency in ms",
            &p_labels,
            pool_metrics.average_acquire_time_ms,
        );

        // Histogram latency koneksi
        if !pool_metrics.latency_histogram_ms.is_empty() {
            let count = pool_metrics.total_queries_executed;
            let sum = pool_metrics.average_acquire_time_ms * (count as f64);
            self.register_histogram(
                "widyabase_pool_query_latency_ms",
                "Database query execution latency histogram in milliseconds",
                &p_labels,
                sum,
                count,
                &pool_metrics.latency_histogram_ms,
            );
        }
    }

    /// Render seluruh metrik dalam format teks Prometheus v0.0.4 (OpenMetrics)
    pub fn render(&self) -> String {
        let lock = self.metrics.read().unwrap();
        let mut sorted_keys: Vec<&String> = lock.keys().collect();
        sorted_keys.sort();

        let mut output = String::new();

        for key in sorted_keys {
            let m = &lock[key];
            let type_str = match m.metric_type {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Summary => "summary",
            };

            // Header HELP dan TYPE
            output.push_str(&format!("# HELP {} {}\n", m.name, m.help));
            output.push_str(&format!("# TYPE {} {}\n", m.name, type_str));

            for sample in &m.samples {
                let sample_name = match &sample.suffix {
                    Some(suffix) => format!("{}_{}", m.name, suffix),
                    None => m.name.clone(),
                };

                if sample.labels.is_empty() {
                    output.push_str(&format!("{} {}\n", sample_name, sample.value));
                } else {
                    let labels_formatted: Vec<String> = sample
                        .labels
                        .iter()
                        .map(|(k, v)| format!("{}=\"{}\"", k, v))
                        .collect();
                    output.push_str(&format!("{}{} {}\n", sample_name, format!("{{{}}}", labels_formatted.join(",")), sample.value));
                }
            }
        }

        output
    }

    /// Handler HTTP router untuk endpoint `/metrics`
    /// Mengembalikan tuple: (Status HTTP, Content-Type, Response Body)
    pub fn handle_http_request(&self, path: &str) -> Option<(u16, &'static str, String)> {
        let clean_path = path.split('?').next().unwrap_or(path);
        if clean_path == "/metrics" || clean_path == "/metrics/" || clean_path == "/v1/metrics" {
            let body = self.render();
            Some((200, "text/plain; version=0.0.4; charset=utf-8", body))
        } else {
            None
        }
    }
}

impl Default for PrometheusRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prometheus_exposition_counter_and_gauge() {
        let registry = PrometheusRegistry::new();
        registry.register_counter(
            "widya_http_requests_total",
            "Total HTTP requests received",
            &[("method", "GET"), ("status", "200")],
            1520.0,
        );

        registry.register_gauge(
            "widya_memory_usage_bytes",
            "Memory allocated in bytes",
            &[("zone", "heap")],
            1048576.0,
        );

        let rendered = registry.render();
        assert!(rendered.contains("# HELP widya_http_requests_total Total HTTP requests received"));
        assert!(rendered.contains("# TYPE widya_http_requests_total counter"));
        assert!(rendered.contains("widya_http_requests_total{method=\"GET\",status=\"200\"} 1520"));

        assert!(rendered.contains("# HELP widya_memory_usage_bytes Memory allocated in bytes"));
        assert!(rendered.contains("# TYPE widya_memory_usage_bytes gauge"));
        assert!(rendered.contains("widya_memory_usage_bytes{zone=\"heap\"} 1048576"));
    }

    #[test]
    fn test_prometheus_exposition_histogram() {
        let registry = PrometheusRegistry::new();
        let buckets = vec![
            (0.005, 50),
            (0.01, 120),
            (0.025, 200),
            (f64::INFINITY, 250),
        ];

        registry.register_histogram(
            "widya_http_duration_seconds",
            "HTTP request duration in seconds",
            &[("handler", "/api/v1/query")],
            2.45,
            250,
            &buckets,
        );

        let rendered = registry.render();
        assert!(rendered.contains("# HELP widya_http_duration_seconds HTTP request duration in seconds"));
        assert!(rendered.contains("# TYPE widya_http_duration_seconds histogram"));
        assert!(rendered.contains("widya_http_duration_seconds_bucket{handler=\"/api/v1/query\",le=\"0.005\"} 50"));
        assert!(rendered.contains("widya_http_duration_seconds_bucket{handler=\"/api/v1/query\",le=\"0.01\"} 120"));
        assert!(rendered.contains("widya_http_duration_seconds_bucket{handler=\"/api/v1/query\",le=\"+Inf\"} 250"));
        assert!(rendered.contains("widya_http_duration_seconds_sum{handler=\"/api/v1/query\"} 2.45"));
        assert!(rendered.contains("widya_http_duration_seconds_count{handler=\"/api/v1/query\"} 250"));
    }

    #[test]
    fn test_prometheus_collect_from_db_pool() {
        let registry = PrometheusRegistry::new();
        let pool_metrics = PoolMetrics {
            active_connections: 4,
            idle_connections: 6,
            total_created: 10,
            total_recycled: 42,
            total_evicted: 1,
            total_acquisitions: 50,
            total_timeouts: 0,
            total_health_check_failures: 0,
            total_queries_executed: 100,
            average_acquire_time_ms: 1.25,
            latency_histogram_ms: vec![(5.0, 95), (f64::INFINITY, 100)],
        };

        registry.collect_from_db_pool("primary_postgres", &pool_metrics);
        let rendered = registry.render();

        assert!(rendered.contains("widyabase_pool_active_connections{pool=\"primary_postgres\"} 4"));
        assert!(rendered.contains("widyabase_pool_idle_connections{pool=\"primary_postgres\"} 6"));
        assert!(rendered.contains("widyabase_pool_connections_recycled_total{pool=\"primary_postgres\"} 42"));
        assert!(rendered.contains("widyabase_pool_query_latency_ms_bucket{le=\"5\",pool=\"primary_postgres\"} 95"));
    }

    #[test]
    fn test_prometheus_http_handler() {
        let registry = PrometheusRegistry::new();
        registry.register_counter("uptime_seconds", "Process uptime", &[], 3600.0);

        let res = registry.handle_http_request("/metrics");
        assert!(res.is_some());
        let (status, content_type, body) = res.unwrap();
        assert_eq!(status, 200);
        assert_eq!(content_type, "text/plain; version=0.0.4; charset=utf-8");
        assert!(body.contains("uptime_seconds 3600"));

        let non_matching = registry.handle_http_request("/api/unknown");
        assert!(non_matching.is_none());
    }
}
