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

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;
    use pretty_assertions::assert_eq;
    use tokio_test::block_on;

    #[test]
    fn should_run_js_and_return_stdout() {
        let result = block_on(exec(r#"console.log("Hello World")"#));

        assert_eq!(result.unwrap(), "Hello World\n")
    }

    #[test]
    fn should_return_error_if_js_is_invalid() {
        let result = block_on(exec(r#"invalid()"#));

        match result {
            Err(ExecError::Output(output)) => {
                let error_message = str::from_utf8(&output.stderr).unwrap();
                assert!(dbg!(error_message)
                    .contains("error: Uncaught ReferenceError: invalid is not defined"));
            }
            _ => panic!("Unexpected result {:?}", result),
        }
    }
}
