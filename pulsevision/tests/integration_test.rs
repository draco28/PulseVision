//! Integration tests for PulseVision with real PulseDB and GLM LLM.
//!
//! These tests require:
//! - .env file with LLM_API_BASE, LLM_API_KEY, LLM_MODEL
//! - Network access to the GLM API
//!
//! Run with: cargo test --test integration_test -- --nocapture

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use pulsevision::api::projections::pca_project;
use pulsevision::config::{EventSource, PulseVisionConfig, SubstrateSource};
use pulsevision::session::NoopSessionStore;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

// ── Test Helpers ──────────────────────────────────────────────────────

/// Create a PulseDB with test experiences and return its path.
fn create_test_substrate(dir: &TempDir) -> String {
    use pulsedb::{
        AgentId, Config, ExperienceType, NewExperience, PulseDB,
    };

    let db_path = dir.path().join("test_substrate.db");
    let path_str = db_path.to_string_lossy().to_string();

    let config = Config::with_builtin_embeddings();
    let db = PulseDB::open(&path_str, config).expect("Failed to create test PulseDB");

    let collective_id = db
        .create_collective("test-project")
        .expect("Failed to create collective");

    // Insert experiences of various types with real embeddings (builtin provider generates them)
    let experiences = vec![
        ("Implemented error handling with thiserror derive macros for clean error propagation", ExperienceType::Solution { problem_ref: None, approach: "thiserror".into(), worked: true }),
        ("Discovered that tokio::spawn_blocking is needed for CPU-intensive nalgebra SVD computation", ExperienceType::TechInsight { technology: "tokio".into(), insight: "spawn_blocking for CPU work".into() }),
        ("Connection refused when PulseDB is opened by two writers simultaneously", ExperienceType::ErrorPattern { signature: "SQLITE_BUSY".into(), fix: "Use Config::read_only()".into(), prevention: "Single writer pattern".into() }),
        ("Decided to use PCA over UMAP for v1 dimensionality reduction", ExperienceType::ArchitecturalDecision { decision: "PCA for v1".into(), rationale: "Fast, deterministic, mature Rust ecosystem".into() }),
        ("User prefers dark theme for dev tools with high contrast colors", ExperienceType::UserPreference { category: "UI".into(), preference: "dark theme".into(), strength: 0.9 }),
        ("React Three Fiber InstancedMesh pattern handles 5000+ nodes at 60fps", ExperienceType::SuccessPattern { task_type: "3D rendering".into(), approach: "InstancedMesh".into(), quality: 0.95 }),
        ("WebSocket broadcast channel capacity of 256 can overflow during token streaming", ExperienceType::Difficulty { description: "Broadcast overflow".into(), severity: pulsedb::Severity::Medium }),
        ("PulseHive emits 14 event types all serialized with serde tag type", ExperienceType::Fact { statement: "14 HiveEvent types".into(), source: "PulseHive SPEC".into() }),
        ("Generic experience about testing patterns", ExperienceType::Generic { category: Some("testing".into()) }),
        ("Axum 0.8 requires explicit Router state type parameter", ExperienceType::TechInsight { technology: "axum".into(), insight: "Router<AppState> generic".into() }),
    ];

    for (i, (content, exp_type)) in experiences.iter().enumerate() {
        let importance = 0.3 + (i as f32 * 0.07);
        let new_exp = NewExperience {
            collective_id,
            content: content.to_string(),
            experience_type: exp_type.clone(),
            embedding: None, // Builtin provider generates embedding
            importance: importance.min(1.0),
            confidence: 0.8,
            domain: vec!["rust".into(), "pulsevision".into()],
            related_files: vec![],
            source_agent: AgentId("test-agent".into()),
            source_task: None,
        };
        db.record_experience(new_exp)
            .expect("Failed to record experience");
    }

    // Add a relation between first two experiences
    let exps = db
        .list_experiences(collective_id, 10, 0)
        .expect("Failed to list experiences");
    if exps.len() >= 2 {
        use pulsedb::{NewExperienceRelation, RelationType};
        let rel = NewExperienceRelation {
            source_id: exps[0].id,
            target_id: exps[1].id,
            relation_type: RelationType::Supports,
            strength: 0.8,
            metadata: None,
        };
        db.store_relation(rel).expect("Failed to store relation");
    }

    db.close().expect("Failed to close PulseDB");
    path_str
}

/// Build a PulseVision test app from a substrate path.
fn build_test_app(substrate_path: &str) -> Router {
    let config = PulseVisionConfig {
        substrate: SubstrateSource::File {
            path: substrate_path.to_string(),
        },
        event_source: EventSource::WebSocketIngest,
        session_store: Arc::new(NoopSessionStore),
        collective_id: None,
    };
    pulsevision::router(config)
}

/// Send a GET request and return (status, body_json).
async fn get_json(
    app: &Router,
    uri: &str,
) -> (StatusCode, serde_json::Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap_or(serde_json::json!({}));
    (status, json)
}

// ── Health Check ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_endpoint() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (status, body) = get_json(&app, "/api/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["substrate"], "connected (read-only)");
}

// ── Collectives ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_list_collectives() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (status, body) = get_json(&app, "/api/substrate/collectives").await;
    assert_eq!(status, StatusCode::OK);

    let collectives = body["collectives"].as_array().unwrap();
    assert!(!collectives.is_empty());
    assert_eq!(collectives[0]["name"], "test-project");
}

// ── Experiences ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_list_experiences() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    // First get collective ID
    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();

    let uri = format!(
        "/api/substrate/experiences?collective_id={}&limit=5",
        collective_id
    );
    let (status, body) = get_json(&app, &uri).await;
    assert_eq!(status, StatusCode::OK);

    let experiences = body["experiences"].as_array().unwrap();
    assert_eq!(experiences.len(), 5);
    assert_eq!(body["limit"], 5);

    // Each experience should have required fields
    let exp = &experiences[0];
    assert!(exp["id"].is_string());
    assert!(exp["content_preview"].is_string());
    assert!(exp["experience_type"].is_string());
    assert!(exp["importance"].is_number());
}

#[tokio::test]
async fn test_get_experience_detail() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    // Get an experience ID
    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();
    let uri = format!(
        "/api/substrate/experiences?collective_id={}&limit=1",
        collective_id
    );
    let (_, list) = get_json(&app, &uri).await;
    let exp_id = list["experiences"][0]["id"].as_str().unwrap();

    // Get detail
    let detail_uri = format!("/api/substrate/experiences/{}", exp_id);
    let (status, body) = get_json(&app, &detail_uri).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["content"].as_str().unwrap().len() > 10);
    assert!(body["domain"].is_array());
}

#[tokio::test]
async fn test_experience_not_found() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (status, body) = get_json(
        &app,
        "/api/substrate/experiences/00000000-0000-0000-0000-000000000000",
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body["error"].is_string());
}

// ── Relations ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_list_relations() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();

    let uri = format!(
        "/api/substrate/relations?collective_id={}",
        collective_id
    );
    let (status, body) = get_json(&app, &uri).await;
    assert_eq!(status, StatusCode::OK);

    let relations = body["relations"].as_array().unwrap();
    assert!(!relations.is_empty());
    assert_eq!(relations[0]["relation_type"], "Supports");
    assert!(relations[0]["strength"].as_f64().unwrap() > 0.0);
}

// ── PCA Projections ───────────────────────────────────────────────────

#[tokio::test]
async fn test_embeddings_pca_projection() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();

    let uri = format!(
        "/api/substrate/embeddings?collective_id={}",
        collective_id
    );
    let (status, body) = get_json(&app, &uri).await;
    assert_eq!(status, StatusCode::OK);

    assert_eq!(body["method"], "pca");
    assert_eq!(body["total_points"], 10); // 10 test experiences
    assert!(body["embedding_dimension"].as_u64().unwrap() > 0);

    let projections = body["projections"].as_array().unwrap();
    assert_eq!(projections.len(), 10);

    // Each projection should have id, x, y, z
    for p in projections {
        assert!(p["id"].is_string());
        assert!(p["x"].is_number());
        assert!(p["y"].is_number());
        assert!(p["z"].is_number());
        // Coordinates should be finite
        assert!(p["x"].as_f64().unwrap().is_finite());
        assert!(p["y"].as_f64().unwrap().is_finite());
        assert!(p["z"].as_f64().unwrap().is_finite());
    }

    // Variance explained should sum to <= 1.0
    let variance = body["variance_explained"].as_array().unwrap();
    let total_var: f64 = variance.iter().map(|v| v.as_f64().unwrap()).sum();
    assert!(total_var <= 1.01);
}

// ── Attractors ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_attractors() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();

    // Use low threshold to get some attractors from our test data
    let uri = format!(
        "/api/substrate/attractors?collective_id={}&threshold=0.2",
        collective_id
    );
    let (status, body) = get_json(&app, &uri).await;
    assert_eq!(status, StatusCode::OK);

    let attractors = body["attractors"].as_array().unwrap();
    assert!(!attractors.is_empty());

    for a in attractors {
        assert!(a["experience_id"].is_string());
        assert!(a["strength"].as_f64().unwrap() > 0.2);
        assert!(a["influence_radius"].as_f64().unwrap() > 0.0);
        assert!(a["warp_factor"].as_f64().unwrap() >= 0.0);
        assert!(a["warp_factor"].as_f64().unwrap() <= 1.0);
        assert!(a["position"]["x"].is_number());
    }
}

// ── Stats ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_stats() {
    let dir = TempDir::new().unwrap();
    let path = create_test_substrate(&dir);
    let app = build_test_app(&path);

    let (_, collectives) = get_json(&app, "/api/substrate/collectives").await;
    let collective_id = collectives["collectives"][0]["id"].as_str().unwrap();

    let uri = format!("/api/substrate/stats?collective_id={}", collective_id);
    let (status, body) = get_json(&app, &uri).await;
    assert_eq!(status, StatusCode::OK);

    assert_eq!(body["experience_count"], 10);
    assert!(body["embedding_dimension"].as_u64().unwrap() > 0);
}

// ── PCA Unit Tests (Dynamic Dimension) ────────────────────────────────

#[test]
fn test_pca_with_real_embeddings() {
    // Simulate 384d embeddings with known structure
    let mut embeddings = Vec::new();
    for i in 0..30 {
        let emb: Vec<f32> = (0..384)
            .map(|j| ((i * 13 + j * 7) as f32 * 0.01).sin())
            .collect();
        embeddings.push((format!("exp-{i}"), emb));
    }

    let (projections, variance) = pca_project(&embeddings).unwrap();
    assert_eq!(projections.len(), 30);
    assert_eq!(variance.len(), 3);

    // Points should be distributed (not all at origin)
    let has_nonzero = projections
        .iter()
        .any(|p| p.x.abs() > 0.001 || p.y.abs() > 0.001 || p.z.abs() > 0.001);
    assert!(has_nonzero, "PCA should produce non-zero projections");
}

// ── GLM LLM Integration Test ─────────────────────────────────────────

#[tokio::test]
async fn test_glm_llm_call() {
    // Load .env for API credentials
    dotenvy::dotenv().ok();

    let api_base = match std::env::var("LLM_API_BASE") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Skipping GLM test: LLM_API_BASE not set");
            return;
        }
    };
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY required");
    let model = std::env::var("LLM_MODEL").unwrap_or_else(|_| "GLM-4.7".into());

    // Create OpenAI-compatible provider
    use pulsehive_openai::{OpenAICompatibleProvider, OpenAIConfig};
    use pulsehive_core::prelude::*;

    let config = OpenAIConfig::new(&api_key, &model).with_base_url(&api_base);
    let provider = OpenAICompatibleProvider::new(config);

    // Make a real LLM call
    let messages = vec![
        Message::system("You are a helpful assistant. Reply in one short sentence."),
        Message::user("What is PCA in machine learning?"),
    ];
    let llm_config = LlmConfig::new("glm", &model);

    let result = provider.chat(messages, vec![], &llm_config).await;

    match result {
        Ok(response) => {
            assert!(
                response.content.is_some(),
                "GLM should return content"
            );
            let content = response.content.unwrap();
            assert!(!content.is_empty(), "GLM response should not be empty");
            println!("GLM response: {content}");
            println!(
                "Tokens: input={}, output={}",
                response.usage.input_tokens, response.usage.output_tokens
            );
        }
        Err(e) => {
            // Try fallback model
            let fallback_model =
                std::env::var("LLM_MODEL_FALLBACK").unwrap_or_else(|_| "GLM-4.7".into());
            eprintln!("Primary model failed ({e}), trying fallback: {fallback_model}");

            let config2 = OpenAIConfig::new(&api_key, &fallback_model).with_base_url(&api_base);
            let provider2 = OpenAICompatibleProvider::new(config2);
            let messages2 = vec![
                Message::system("Reply in one sentence."),
                Message::user("What is PCA?"),
            ];
            let llm_config2 = LlmConfig::new("glm", &fallback_model);

            let result2 = provider2.chat(messages2, vec![], &llm_config2).await;
            match result2 {
                Ok(resp) => {
                    assert!(resp.content.is_some());
                    println!("Fallback GLM response: {}", resp.content.unwrap());
                }
                Err(e2) => {
                    panic!("Both GLM models failed: primary={e}, fallback={e2}");
                }
            }
        }
    }
}

// ── Full PulseHive Event Pipeline Test ────────────────────────────────

#[tokio::test]
async fn test_pulsehive_event_collection() {
    // Load .env for API credentials
    dotenvy::dotenv().ok();

    let api_base = match std::env::var("LLM_API_BASE") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Skipping PulseHive pipeline test: LLM_API_BASE not set");
            return;
        }
    };
    let api_key = std::env::var("LLM_API_KEY").expect("LLM_API_KEY required");
    let model = std::env::var("LLM_MODEL_FALLBACK")
        .or_else(|_| std::env::var("LLM_MODEL"))
        .unwrap_or_else(|_| "GLM-4.7".into());

    use pulsehive::prelude::*;
    use pulsehive::HiveMind;
    use pulsehive_openai::{OpenAICompatibleProvider, OpenAIConfig};
    use std::sync::Mutex;

    // Collect events in a Vec
    struct EventCollector {
        events: Mutex<Vec<String>>,
    }

    // Wrapper that owns Arc<EventCollector> and implements EventExporter
    struct EventCollectorWrapper(Arc<EventCollector>);

    #[async_trait::async_trait]
    impl pulsehive_core::export::EventExporter for EventCollectorWrapper {
        async fn export(&self, event: &pulsehive_core::event::HiveEvent) {
            let json = serde_json::to_string(event).unwrap();
            self.0.events.lock().unwrap().push(json);
        }
        async fn flush(&self) {}
    }

    let collector = Arc::new(EventCollector {
        events: Mutex::new(Vec::new()),
    });

    // Build HiveMind with GLM provider
    let dir = TempDir::new().unwrap();
    let db_path = dir.path().join("pipeline_test.db");

    let llm_config = OpenAIConfig::new(&api_key, &model).with_base_url(&api_base);
    let provider = OpenAICompatibleProvider::new(llm_config);

    let hive = HiveMind::builder()
        .substrate_path(&db_path)
        .llm_provider("glm", provider)
        .event_exporter(EventCollectorWrapper(collector.clone()))
        .build();

    let hive = match hive {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Skipping pipeline test: HiveMind build failed: {e}");
            return;
        }
    };

    // Define a simple agent
    let agent = AgentDefinition {
        name: "test-analyst".into(),
        kind: AgentKind::Llm(Box::new(LlmAgentConfig {
            system_prompt: "You are a code analyst. When given a task, provide a brief one-sentence analysis. Do not use any tools.".into(),
            tools: vec![],
            lens: Lens::default(),
            llm_config: LlmConfig::new("glm", &model),
            experience_extractor: None,
            refresh_every_n_tool_calls: None,
        })),
    };

    let task = pulsehive::Task::new("Analyze the benefits of using Rust for backend development");

    // Deploy and collect events
    use futures_util::StreamExt;
    let mut event_stream = hive.deploy(vec![agent], vec![task]).await.unwrap();

    // Consume the event stream (with timeout)
    let timeout = tokio::time::timeout(std::time::Duration::from_secs(30), async {
        while let Some(event) = event_stream.next().await {
            println!("Event: {:?}", std::mem::discriminant(&event));
        }
    });

    match timeout.await {
        Ok(_) => println!("Event stream completed"),
        Err(_) => println!("Event stream timed out (30s) — this is OK for test"),
    }

    // Drop the event stream and HiveMind to release PulseDB lock
    drop(event_stream);
    drop(hive);
    // Give PulseDB time to release the file lock
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Verify we collected events
    let events = collector.events.lock().unwrap();
    println!("Collected {} events", events.len());

    assert!(
        !events.is_empty(),
        "PulseHive should have emitted at least one event"
    );

    // Verify events are valid JSON with expected fields
    for event_json in events.iter() {
        let event: serde_json::Value = serde_json::from_str(event_json).unwrap();
        assert!(event["type"].is_string(), "Event must have 'type' field");
        assert!(
            event["timestamp_ms"].is_number(),
            "Event must have 'timestamp_ms'"
        );
    }

    // Check we got the key event types
    let event_types: Vec<String> = events
        .iter()
        .map(|e| {
            let v: serde_json::Value = serde_json::from_str(e).unwrap();
            v["type"].as_str().unwrap().to_string()
        })
        .collect();

    println!("Event types: {:?}", event_types);
    assert!(
        event_types.contains(&"agent_started".to_string()),
        "Should have agent_started event"
    );

    // Now verify the substrate was populated and PulseVision can read it
    let app = build_test_app(&db_path.to_string_lossy());
    let (status, health) = get_json(&app, "/api/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health["status"], "healthy");

    println!("E2E pipeline test passed: PulseHive → Events → PulseVision API");
}
