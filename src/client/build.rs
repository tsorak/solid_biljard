use anyhow::bail;
use tokio::{process, task::JoinHandle};

use crate::state::{ClientChannel, ClientChannelMessage, State};

use super::Client;

pub struct Builder {
    package_manager: String,
    client_channel: Option<ClientChannel>,
    build_loop_handle: Option<JoinHandle<()>>,
}

impl Client {
    pub async fn build_client(&self) -> &Self {
        self.builder.build_client().await;
        self
    }

    pub async fn ensure_node_modules(&self) -> anyhow::Result<()> {
        self.builder.ensure_node_modules().await
    }
}

impl Builder {
    pub async fn new(state: State) -> Self {
        let package_manager = get_package_manager()
            .await
            .expect("No JS package manager found");

        Self {
            package_manager,
            client_channel: Some(state.client_channel.clone()),
            build_loop_handle: None,
        }
    }

    pub fn init(&mut self) -> &mut Self {
        self.build_loop_handle = Some(self.start_build_event_loop());
        self
    }

    pub async fn build_client(&self) -> &Self {
        build_client(&self.package_manager).await;
        self
    }

    pub async fn ensure_node_modules(&self) -> anyhow::Result<()> {
        let output = match process::Command::new(&self.package_manager)
            .arg("install")
            .current_dir("./client")
            .output()
            .await
        {
            Ok(o) => o,
            Err(_) => bail!("Failed to install client dependencies"),
        };

        if !output.status.success() {
            bail!("Failed to install client dependencies");
        }

        println!("node_modules up to date");
        Ok(())
    }

    fn start_build_event_loop(&mut self) -> JoinHandle<()> {
        // at the moment only this method uses the client_channel. Expect it to only be used once.
        let mut ch = self
            .client_channel
            .take()
            .expect("Only take client_channel once");
        let pm = self.package_manager.clone();

        tokio::spawn(async move {
            loop {
                if let Some(ClientChannelMessage::Build) = ch.recv().await {
                    build_client(&pm).await;
                    ch.send_refresh();
                }
            }
        })
    }
}

async fn build_client(pm: &str) {
    println!("Building client...");

    match process::Command::new(pm)
        .args(["run", "build-dev"])
        .current_dir("./client")
        .output()
        .await
    {
        Ok(v) => {
            if v.status.success() {
                println!("{}", String::from_utf8_lossy(v.stdout.as_ref()));
            } else {
                println!(
                    "Error building client\nSTDOUT\n{}\nSTDERR\n{}",
                    String::from_utf8_lossy(v.stdout.as_ref()),
                    String::from_utf8_lossy(v.stderr.as_ref())
                );
            }
        }
        Err(err) => {
            println!("Error running build command\n{:#?}", err);
        }
    };
}

async fn get_package_manager() -> anyhow::Result<String> {
    if let Some(_bun_path) = get_executable_path("bun").await {
        return Ok("bun".into());
    }

    if let Some(_npm_path) = get_executable_path("npm").await {
        return Ok("npm".into());
    }

    bail!("No JS package manager found")
}

async fn get_executable_path(s: &str) -> Option<String> {
    match process::Command::new("which").arg(s).output().await {
        Ok(output) => {
            if output.status.success() {
                let path = String::from_utf8(output.stdout)
                    .expect("Executable path contains invalid utf-8");
                Some(path)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
