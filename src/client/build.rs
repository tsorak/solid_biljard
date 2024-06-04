use anyhow::bail;
use tokio::process;

use super::Client;

pub struct Builder {
    package_manager: String,
}

impl Client {
    pub async fn build_client(&self) -> &Self {
        self.builder.build_client();
        self
    }

    pub async fn ensure_node_modules(&self) -> anyhow::Result<()> {
        self.builder.ensure_node_modules().await
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            package_manager: "".into(),
        }
    }

    pub async fn init(&mut self) -> &mut Self {
        let pm = get_package_manager()
            .await
            .expect("No JS package manager found");

        self.package_manager = pm;
        self
    }

    pub async fn build_client(&self) -> &Self {
        match process::Command::new(&self.package_manager)
            .args(["run", "build"])
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
        self
    }

    pub async fn ensure_node_modules(&self) -> anyhow::Result<()> {
        let pm: String = match process::Command::new("which").arg("bun").output().await {
            Ok(output) => {
                if output.status.success() {
                    "bun".into()
                } else {
                    "npm".into()
                }
            }
            Err(_) => "npm".into(),
        };

        let output = match process::Command::new(pm)
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

        Ok(())
    }
}

async fn get_package_manager() -> anyhow::Result<String> {
    if let Some(bun_path) = get_executable_path("bun").await {
        return Ok(bun_path);
    }

    if let Some(npm_path) = get_executable_path("npm").await {
        return Ok(npm_path);
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
