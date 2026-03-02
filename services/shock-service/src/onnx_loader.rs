//! This can integrate ONNX model inference

use onnxruntime::{environment::Environment};

pub fn load_model(path: &str) {
    let environment = Environment::builder()
        .with_name("shock-model")
        .build()
        .unwrap();

    // Load session here
}