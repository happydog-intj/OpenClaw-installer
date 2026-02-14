use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct CommandExecutor;

impl CommandExecutor {
    /// 执行命令并实时捕获输出
    pub fn execute_with_output<F>(
        program: &str,
        args: &[&str],
        mut on_output: F,
    ) -> Result<bool, Box<dyn std::error::Error>>
    where
        F: FnMut(String),
    {
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // 读取 stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    on_output(line);
                }
            }
        }

        // 等待进程结束
        let status = child.wait()?;
        Ok(status.success())
    }

    /// 简单执行命令
    pub fn execute(program: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new(program)
            .args(args)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string().into())
        }
    }

    /// 检查命令是否存在
    pub fn command_exists(command: &str) -> bool {
        which::which(command).is_ok()
    }
}
