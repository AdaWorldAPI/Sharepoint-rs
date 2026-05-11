//! Delta Cursor Store — Persists delta tokens for incremental sync (ST-10 target)

pub struct DeltaCursor<'a> {
    pub site_id: &'a str,
    pub drive_id: &'a str,
    pub delta_token: &'a str,
    pub last_sync_at_ms: u64,
}

pub trait DeltaCursorStore {
    type Error;

    fn get(&self, site: &str, drive: &str) -> impl std::future::Future<Output = Result<Option<DeltaCursor<'_>>, Self::Error>> + '_;
    fn save(&self, cursor: &DeltaCursor<'_>) -> impl std::future::Future<Output = Result<(), Self::Error>> + '_;
}