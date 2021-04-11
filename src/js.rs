use std::process;

use thiserror::Error;
use tokio::process::Command;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("{:}", String::from_utf8_lossy(&.0.stderr))]
    Output(process::Output),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ExecError {
    pub fn other<E: Into<anyhow::Error>>(err: E) -> Self {
        Self::Other(err.into())
    }
}

pub async fn exec(js: &str) -> Result<String, ExecError> {
    let output = Command::new("deno")
        .env("NO_COLOR", "true")
        .arg("eval")
        .arg(js)
        .output()
        .await
        .map_err(ExecError::other)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into())
    } else {
        Err(ExecError::Output(output))
    }
}
