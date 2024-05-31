use tokio::process;

pub async fn build_client() {
    match process::Command::new("bun")
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
                    "Error building client\nSTDOUT\n{:#?}\nSTDERR\n{:#?}",
                    String::from_utf8(v.stdout).unwrap(),
                    String::from_utf8(v.stderr).unwrap()
                );
            }
        }
        Err(err) => {
            println!("Error running build command\n{:#?}", err);
        }
    }
}
