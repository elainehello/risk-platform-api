# Risk Platform API

Enterprise-grade risk platform built with Rust and Axum.

## Services (minimal skeleton)

| Service            | Port | Description            |
|--------------------|------|------------------------|
| api-gateway          | 8080 | API entry point          |
| regime-service        | 8081 | Regime detection API      |
| ingestion-service      | 8082 | Data ingestion endpoint   |
| volatility-service      | 8083 | Volatility calculations    |
| shock-service           | 8084 | Shock modeling service     |
| scoring-service          | 8085 | Scoring and risk APIs       |

Each service currently responds with a minimal `"ok"` response and is ready for feature development.

## Build

```bash
cargo check --workspace

## Run service
```bash
cargo run -p api-gateway
