use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    pub cpu_id: String,
    pub disk_id: String,
    pub memory_id: String,
    pub motherboard_id: String,
}

pub struct SystemInfoService;

impl SystemInfoService {
    pub fn new() -> Self {
        SystemInfoService
    }

    pub fn get_system_info(&self) -> SystemInfoResponse {
        info!("SystemInfoService: Generating system info");
        SystemInfoResponse {
            cpu_id: self.get_cpu_id(),
            disk_id: self.get_disk_id(),
            memory_id: self.get_memory_id(),
            motherboard_id: self.get_motherboard_id(),
        }
    }

    pub fn get_cpu_id(&self) -> String {
        if cfg!(target_os = "windows") {
            // Windows系统获取CPU ID
            match Command::new("wmic")
                .args(["cpu", "get", "ProcessorId"])
                .output()
            {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() >= 2 {
                        lines[1].trim().to_string()
                    } else {
                        "无法获取CPU ID".to_string()
                    }
                }
                Err(_) => "无法执行wmic命令".to_string(),
            }
        } else {
            // Linux系统获取CPU信息
            match Command::new("cat").arg("/proc/cpuinfo").output() {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.starts_with("serial") || line.starts_with("Processor") {
                            if let Some((_, value)) = line.split_once(':') {
                                return value.trim().to_string();
                            }
                        }
                    }
                    "无法获取CPU序列号".to_string()
                }
                Err(_) => "无法读取cpuinfo".to_string(),
            }
        }
    }

    pub fn get_disk_id(&self) -> String {
        if cfg!(target_os = "windows") {
            // Windows系统获取磁盘序列号
            match Command::new("wmic")
                .args(["diskdrive", "get", "SerialNumber"])
                .output()
            {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() >= 2 {
                        lines[1].trim().to_string()
                    } else {
                        "无法获取磁盘序列号".to_string()
                    }
                }
                Err(_) => "无法执行wmic命令".to_string(),
            }
        } else {
            // Linux系统获取磁盘ID
            match Command::new("lsblk").args(["-d", "-o", "SERIAL"]).output() {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() >= 2 {
                        lines[1].trim().to_string()
                    } else {
                        "无法获取磁盘序列号".to_string()
                    }
                }
                Err(_) => "无法执行lsblk命令".to_string(),
            }
        }
    }

    pub fn get_memory_id(&self) -> String {
        if cfg!(target_os = "windows") {
            // Windows系统获取内存信息
            match Command::new("wmic")
                .args(["memorychip", "get", "SerialNumber"])
                .output()
            {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() >= 2 {
                        lines[1].trim().to_string()
                    } else {
                        "无法获取内存序列号".to_string()
                    }
                }
                Err(_) => "无法执行wmic命令".to_string(),
            }
        } else {
            // Linux系统获取内存信息
            match Command::new("dmidecode").args(["-t", "17"]).output() {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.contains("Serial Number:") {
                            if let Some((_, value)) = line.split_once(':') {
                                return value.trim().to_string();
                            }
                        }
                    }
                    "无法获取内存序列号".to_string()
                }
                Err(_) => "无法执行dmidecode命令".to_string(),
            }
        }
    }

    pub fn get_motherboard_id(&self) -> String {
        if cfg!(target_os = "windows") {
            // Windows系统获取主板序列号
            match Command::new("wmic")
                .args(["baseboard", "get", "SerialNumber"])
                .output()
            {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() >= 2 {
                        lines[1].trim().to_string()
                    } else {
                        "无法获取主板序列号".to_string()
                    }
                }
                Err(_) => "无法执行wmic命令".to_string(),
            }
        } else {
            // Linux系统获取主板信息
            match Command::new("dmidecode").args(["-t", "2"]).output() {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.contains("Serial Number:") {
                            if let Some((_, value)) = line.split_once(':') {
                                return value.trim().to_string();
                            }
                        }
                    }
                    "无法获取主板序列号".to_string()
                }
                Err(_) => "无法执行dmidecode命令".to_string(),
            }
        }
    }
}
