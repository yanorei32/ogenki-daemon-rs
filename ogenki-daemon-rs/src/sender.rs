use anyhow::Result;
use twelite_serial::StatusNotify;

pub struct WebBackend {
    client: reqwest::Client,
    backend: crate::cli::Backend,
}

impl WebBackend {
    fn new_from_backend(backend: &crate::cli::Backend) -> Self {
        let client = reqwest::Client::new();
        let backend = backend.clone();

        Self { client, backend }
    }

    async fn send(&self, notify: &StatusNotify) -> Result<()> {
        let url = self.backend.url.as_ref().unwrap();

        let ctx = self.client.post(url.to_string());

        let ctx = match self.backend.username {
            Some(_) => ctx.basic_auth(
                &self.backend.username.as_ref().unwrap(),
                self.backend.password.as_ref(),
            ),
            None => ctx,
        };

        ctx.multipart(
            reqwest::multipart::Form::new()
                .text("wireless", notify.lqi().to_string())
                .text("battery", notify.power_voltage_millis().to_string())
                .text("doorsensor", notify.di_status().to_string())
                .text("status", notify.di1_status().to_string())
                .text("changed", notify.di1_changed().to_string()),
        )
        .send()
        .await?
        .error_for_status()?;

        Ok(())
    }
}

pub enum Sender {
    Web(WebBackend),
    Nothing,
}

impl Sender {
    pub fn new(backend: &crate::cli::Backend) -> Self {
        match backend.url {
            None => {
                println!("Warning: backend is not specified.");
                println!("         entering dry-run mode.");
                Self::Nothing
            }
            Some(_) => Self::Web(WebBackend::new_from_backend(&backend)),
        }
    }

    pub async fn send(&self, notify: &StatusNotify) -> Result<()> {
        match self {
            Self::Web(backend) => backend.send(notify).await,
            Self::Nothing => Ok(()),
        }
    }
}
