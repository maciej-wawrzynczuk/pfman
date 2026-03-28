use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::transaction_log::TransactionLog;

pub type AppState = Arc<RwLock<Option<TransactionLog>>>;

pub fn initial_state() -> AppState {
    Arc::new(RwLock::new(None))
}
