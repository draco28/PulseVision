use std::sync::Arc;

use pulsedb::{CollectiveId, Config, ExperienceId, PulseDB};

use crate::config::SubstrateSource;
use crate::error::{Error, Result};

/// Unified read API over both substrate access modes.
pub struct SubstrateReader {
    db: Arc<PulseDB>,
}

impl SubstrateReader {
    /// Create a SubstrateReader from a SubstrateSource.
    pub fn new(source: SubstrateSource) -> Result<Self> {
        let db = match source {
            SubstrateSource::Shared(db) => db,
            SubstrateSource::File { path } => {
                let config = Config::read_only();
                let db = PulseDB::open(&path, config)
                    .map_err(|e| Error::Substrate(format!("Failed to open PulseDB: {e}")))?;
                Arc::new(db)
            }
        };
        Ok(Self { db })
    }

    /// Get the underlying PulseDB handle.
    pub fn db(&self) -> &PulseDB {
        &self.db
    }

    /// Get the embedding dimension configured in this database.
    pub fn embedding_dimension(&self) -> usize {
        self.db.embedding_dimension()
    }

    /// Check if the database is in read-only mode.
    pub fn is_read_only(&self) -> bool {
        self.db.is_read_only()
    }

    /// List all collectives.
    pub fn list_collectives(&self) -> Result<Vec<pulsedb::Collective>> {
        self.db
            .list_collectives()
            .map_err(|e| Error::Substrate(e.to_string()))
    }

    /// List experiences with pagination.
    pub fn list_experiences(
        &self,
        collective_id: CollectiveId,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<pulsedb::Experience>> {
        self.db
            .list_experiences(collective_id, limit, offset)
            .map_err(|e| Error::Substrate(e.to_string()))
    }

    /// Get a single experience by ID.
    pub fn get_experience(
        &self,
        id: ExperienceId,
    ) -> Result<Option<pulsedb::Experience>> {
        self.db
            .get_experience(id)
            .map_err(|e| Error::Substrate(e.to_string()))
    }

    /// List relations with pagination.
    pub fn list_relations(
        &self,
        collective_id: CollectiveId,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<pulsedb::ExperienceRelation>> {
        self.db
            .list_relations(collective_id, limit, offset)
            .map_err(|e| Error::Substrate(e.to_string()))
    }

    /// List insights with pagination.
    pub fn list_insights(
        &self,
        collective_id: CollectiveId,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<pulsedb::DerivedInsight>> {
        self.db
            .list_insights(collective_id, limit, offset)
            .map_err(|e| Error::Substrate(e.to_string()))
    }

    /// Get collective stats.
    pub fn get_collective_stats(
        &self,
        collective_id: CollectiveId,
    ) -> Result<pulsedb::CollectiveStats> {
        self.db
            .get_collective_stats(collective_id)
            .map_err(|e| Error::Substrate(e.to_string()))
    }
}
