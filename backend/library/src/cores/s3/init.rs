use anyhow::Result;
use std::sync::{Arc, RwLock};

use crate::DBManager;
use crate::S3;

impl S3 {
    pub fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = S3::select(manager) {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(S3::default())))
    }
}