#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    #[test]
    fn test_district_processing() {
        // 通过子进程执行并捕获输出
        let output = Command::new("cargo")
            .arg("run")
            .arg("-q")
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), "3,3,3,2,1");
    }
}