use chrono::Utc;
use serde_json;
use {{crate_name}}::Object;
use uuid::Uuid;

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl std::fmt::Debug for {{project-name | capitalize}}Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(thiserror::Error)]
pub enum {{project-name | capitalize}}Error {
    #[error("Invalid {{project-name | capitalize}} data")]
    InvalidDataError(#[from] serde_json::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(level = "debug")]
pub fn build_object() -> Result<Object, {{project-name | capitalize}}Error> {
    Ok(Object {
        name: "test".to_string(),
        id: Uuid::new_v4(),
        modified: Utc::now(),
    })
}
