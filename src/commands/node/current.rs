use crate::utils::file_utils::run_command;

pub async fn current() {
    let node_task = tokio::spawn(async {
        let res = run_command("node", vec!["--version"]);
        if res.status.success() {
            println!("node version: {}", String::from_utf8_lossy(&res.stdout));
        } else {
            println!("failed: node not set");
        }
    });
    let npm_task = tokio::spawn(async {
        let res = run_command("npm", vec!["--version"]);
        if res.status.success() {
            println!("npm version: {}", String::from_utf8_lossy(&res.stdout));
        } else {
            println!("failed: npm not set");
        }
    });

    let _ = tokio::join!(node_task, npm_task);
}
