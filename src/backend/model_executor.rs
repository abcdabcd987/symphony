use super::tensorflow;
use super::error::BackendError;
use crate::common::*;

pub struct ModelExecutor {
    model_session: ModelSession,
    tf_session: tensorflow::Session,
}

impl ModelExecutor {
    pub fn new(
        model_session: ModelSession,
        config: tensorflow::SessionConfig,
    ) -> Result<ModelExecutor, BackendError> {
        let tf_session = tensorflow::Session::new(config)?;
        Ok(ModelExecutor {
            model_session,
            tf_session,
        })
    }
}
