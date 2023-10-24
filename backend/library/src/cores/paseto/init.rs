use anyhow::Result;
use std::sync::{Arc, RwLock};

use crate::DBManager;
use crate::Paseto;

impl Paseto {
    pub fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = Paseto::select(manager) {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(Self::new().upsert(manager)?)))
    }
}