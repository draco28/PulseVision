///! Run with: cargo test --test create_fixture -- --nocapture
///! Creates a test PulseDB substrate at tests/fixtures/test_substrate.db

use pulsedb::{AgentId, Config, ExperienceType, NewExperience, PulseDB};

#[test]
fn create_test_substrate() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let base = std::path::Path::new(manifest_dir).parent().unwrap();
    let fixtures_dir = base.join("tests").join("fixtures");
    std::fs::create_dir_all(&fixtures_dir).ok();
    let path_buf = fixtures_dir.join("test_substrate.db");
    let _ = std::fs::remove_file(&path_buf);
    let path = path_buf.to_str().unwrap();

    let config = Config::with_builtin_embeddings();
    let db = PulseDB::open(path, config).expect("Failed to create PulseDB");

    let collective_id = db.create_collective("pulsevision-demo").expect("Failed to create collective");

    let experiences = vec![
        ("Implemented error handling with thiserror derive macros", ExperienceType::Solution { problem_ref: None, approach: "thiserror".into(), worked: true }),
        ("tokio::spawn_blocking needed for CPU-intensive nalgebra SVD", ExperienceType::TechInsight { technology: "tokio".into(), insight: "spawn_blocking for CPU work".into() }),
        ("Connection refused when PulseDB opened by two writers", ExperienceType::ErrorPattern { signature: "SQLITE_BUSY".into(), fix: "Config::read_only()".into(), prevention: "Single writer".into() }),
        ("Decided PCA over UMAP for v1 dimensionality reduction", ExperienceType::ArchitecturalDecision { decision: "PCA for v1".into(), rationale: "Fast, deterministic".into() }),
        ("User prefers dark theme for dev tools", ExperienceType::UserPreference { category: "UI".into(), preference: "dark theme".into(), strength: 0.9 }),
        ("InstancedMesh handles 5000+ nodes at 60fps", ExperienceType::SuccessPattern { task_type: "3D rendering".into(), approach: "InstancedMesh".into(), quality: 0.95 }),
        ("Broadcast channel capacity 256 can overflow", ExperienceType::Difficulty { description: "Broadcast overflow".into(), severity: pulsedb::Severity::Medium }),
        ("PulseHive emits 14 event types with serde tag", ExperienceType::Fact { statement: "14 HiveEvent types".into(), source: "SPEC".into() }),
        ("Generic testing pattern observation", ExperienceType::Generic { category: Some("testing".into()) }),
        ("Axum 0.8 requires explicit Router state type", ExperienceType::TechInsight { technology: "axum".into(), insight: "Router<AppState> generic".into() }),
    ];

    for (i, (content, exp_type)) in experiences.iter().enumerate() {
        let importance = 0.3 + (i as f32 * 0.07);
        db.record_experience(NewExperience {
            collective_id,
            content: content.to_string(),
            experience_type: exp_type.clone(),
            embedding: None,
            importance: importance.min(1.0),
            confidence: 0.8,
            domain: vec!["rust".into(), "pulsevision".into()],
            related_files: vec![],
            source_agent: AgentId("fixture-gen".into()),
            source_task: None,
        }).unwrap();
    }

    db.close().expect("Failed to close");
    println!("Created test substrate at {path} with 10 experiences");
}
