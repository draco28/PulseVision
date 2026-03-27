use clap::Parser;
use pulsevision::config::{EventSource, PulseVisionConfig, SubstrateSource};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "pulsevision", about = "PulseVision — Real-time observability for PulseHive multi-agent systems")]
#[command(version)]
struct Cli {
    /// Path to PulseDB substrate file
    #[arg(long)]
    substrate: String,

    /// Port to listen on
    #[arg(long, default_value = "3333")]
    port: u16,

    /// Bind address
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,

    /// Directory containing frontend static files (index.html, assets/)
    #[arg(long, default_value = "frontend/dist")]
    static_dir: String,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&cli.log_level)),
        )
        .init();

    tracing::info!(
        substrate = %cli.substrate,
        port = cli.port,
        bind = %cli.bind,
        static_dir = %cli.static_dir,
        "Starting PulseVision server"
    );

    // SQLite session store for event persistence
    let session_db_path = format!("{}.sessions.db", cli.substrate);
    let session_store = Arc::new(
        pulsevision::session::sqlite::SqliteSessionStore::new(&session_db_path)
            .expect("Failed to open session store"),
    );
    tracing::info!(session_db = %session_db_path, "Session store initialized");

    let config = PulseVisionConfig {
        substrate: SubstrateSource::File {
            path: cli.substrate,
        },
        event_source: EventSource::WebSocketIngest,
        session_store,
        collective_id: None,
    };

    let api_router = pulsevision::router(config);

    // Serve frontend static files as fallback (SPA: all non-API routes → index.html)
    let static_dir = cli.static_dir.clone();
    let app = if std::path::Path::new(&static_dir).join("index.html").exists() {
        tracing::info!(dir = %static_dir, "Serving frontend from static files");
        let serve = ServeDir::new(&static_dir)
            .not_found_service(ServeFile::new(format!("{}/index.html", static_dir)));
        api_router.fallback_service(serve)
    } else {
        tracing::warn!(dir = %static_dir, "Static directory not found, serving API only");
        api_router
    };

    let addr = format!("{}:{}", cli.bind, cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("PulseVision listening on http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.ok();
    tracing::info!("Shutting down PulseVision...");
}
