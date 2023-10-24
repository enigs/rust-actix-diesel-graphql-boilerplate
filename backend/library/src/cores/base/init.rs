use anyhow::Result;
use std::sync::{Arc, RwLock};

use crate::DBManager;
use crate::Base;

impl Base {
    pub fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = Base::select(manager) {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(Base::default())))
    }
}