//! Basic workflow integration tests for chaos engine

use chaos_engine::{ChaosEngine, ChaosEngineConfig};

#[tokio::test]
async fn test_chaos_engine_start_stop_workflow() {
    let config = ChaosEngineConfig::default();
    let engine = ChaosEngine::new(config).expect("Failed to create engine");

    // Start engine
    engine.start().await.expect("Failed to start engine");

    // Perform operations (placeholder)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Stop engine
    engine.stop().await.expect("Failed to stop engine");
}

#[tokio::test]
async fn test_multiple_engines_can_run_concurrently() {
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = tokio::spawn(async {
            let config = ChaosEngineConfig::default();
            let engine = ChaosEngine::new(config).unwrap();
            engine.start().await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            engine.stop().await.unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task failed");
    }
}
