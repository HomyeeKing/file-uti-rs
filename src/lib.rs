use std::process::Command;
use tokio::process::Command as AsyncCommand;

pub fn file_uti_sync(path: String) -> String {
    let output = Command::new("mdls")
        .arg("-raw")
        .arg("-name")
        .arg("kMDItemContentType")
        .arg(path)
        .output()
        .expect("failed to execute process");
    let mut result = String::new();
    if output.status.success() {
        result = String::from_utf8(output.stdout).unwrap();
    } else {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("Command failed with error: {}", stderr);
    }
    result
}

pub async fn file_uti(path: String) -> String {
    let output = AsyncCommand::new("mdls")
        .arg("-raw")
        .arg("-name")
        .arg("kMDItemContentType")
        .arg(path)
        .output()
        .await
        .expect("Failed to execute command");
    let mut result = String::new();
    if output.status.success() {
        result = String::from_utf8(output.stdout).unwrap();
    } else {
        // 处理错误情况
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Command failed with error: {}", stderr);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_sync() {
        let result = file_uti_sync(file!().to_string());
        assert_eq!(result, "dyn.ah62d4rv4ge81e62");
    }
    #[tokio::test]
    async fn it_works() {
        let result = file_uti(file!().to_string()).await;
        assert_eq!(result, "dyn.ah62d4rv4ge81e62");
    }
}
