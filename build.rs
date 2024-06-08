use std::process::Command;

fn main() {
    let pm = get_pm();

    match Command::new(&pm)
        .arg("install")
        .current_dir("./client")
        .output()
        .expect("Error running install command")
        .status
        .success()
    {
        true => (),
        false => panic!("Failed to install client dependencies"),
    };

    match Command::new(&pm)
        .args(["run", "build"])
        .current_dir("./client")
        .output()
        .expect("Error running build command")
        .status
        .success()
    {
        true => (),
        false => panic!("Failed to build client!"),
    };
}

fn get_pm() -> String {
    match Command::new("which")
        .arg("bun")
        .output()
        .expect("'which' command failed to run")
        .status
        .success()
    {
        true => "bun".into(),
        false => "npm".into(),
    }
}
