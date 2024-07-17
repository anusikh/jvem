use crate::utils::file_utils::run_command;

#[cfg(target_os = "windows")]
pub async fn current_util() {
    let node_task = tokio::spawn(async {
        let res = run_command("powershell", vec!["-C", "node --version"]);
        if res.status.success() {
            println!("node version: {}", String::from_utf8_lossy(&res.stdout));
        } else {
            println!("failed: node not set");
        }
    });
    let npm_task = tokio::spawn(async {
        let res = run_command("powershell", vec!["-C", "npm --version"]);
        if res.status.success() {
            println!("npm version: {}", String::from_utf8_lossy(&res.stdout));
        } else {
            println!("failed: node not set");
        }
    });

    let _ = tokio::join!(node_task, npm_task);
}

#[cfg(unix)]
pub async fn current_util() {
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

pub async fn current() {
    let _ = current_util().await;
}
