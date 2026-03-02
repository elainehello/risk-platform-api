use metrics_exporter_prometheus::PrometheusBuilder;
use once_cell::sync::OnceCell;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

static PROM_HANDLE: OnceCell<metrics_exporter_prometheus::PrometheusHandle> = OnceCell::new();

pub fn init(service_name: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().json())
        .init();

    tracing::info!("Observability initialized for {}", service_name);

    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().expect("metrics recorder");
    PROM_HANDLE.set(handle).ok();
}

pub fn prometheus_handle() -> &'static metrics_exporter_prometheus::PrometheusHandle {
    PROM_HANDLE.get().expect("Prometheus not initialized")
}
