use anyhow::Result;
use std::sync::{Arc, RwLock};

use crate::DBManager;
use crate::Mailer;

impl Mailer {
    pub fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = Mailer::select(manager) {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(Mailer::default())))
    }
}