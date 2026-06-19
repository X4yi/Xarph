use std::process::Command;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub status: String,
    pub sub_status: String,
    pub enabled: bool,
}

pub struct ServiceManager;

impl ServiceManager {
    pub fn list_services() -> Vec<ServiceInfo> {
        let mut services = Vec::new();

        // Get running units
        if let Ok(out) = Command::new("systemctl")
            .args([
                "--user",
                "list-units",
                "--type=service",
                "--all",
                "--no-legend",
                "--no-pager",
                "--plain",
                "--no-recursive",
            ])
            .output()
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    continue;
                }
                let name = parts[0]
                    .strip_suffix(".service")
                    .unwrap_or(parts[0])
                    .to_string();
                let status = parts[2].to_string();
                let sub_status = parts[3].to_string();

                services.push(ServiceInfo {
                    name,
                    description: String::new(),
                    status,
                    sub_status,
                    enabled: false,
                });
            }
        }

        // Check enabled status
        if let Ok(out) = Command::new("systemctl")
            .args([
                "--user",
                "list-unit-files",
                "--type=service",
                "--no-legend",
                "--no-pager",
                "--plain",
            ])
            .output()
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let unit_name = parts[0]
                        .strip_suffix(".service")
                        .unwrap_or(parts[0]);
                    let enabled = parts[1] == "enabled";
                    if let Some(svc) = services.iter_mut().find(|s| s.name == unit_name) {
                        svc.enabled = enabled;
                    }
                }
            }
        }

        // Get descriptions
        for svc in &mut services {
            if let Ok(out) = Command::new("systemctl")
                .args(["--user", "show", &svc.name, "--property=Description"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    if let Some(desc) = line.strip_prefix("Description=") {
                        svc.description = desc.to_string();
                    }
                }
            }
        }

        services
    }

    pub fn start(service: &str) -> bool {
        Command::new("systemctl")
            .args(["--user", "start", service])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn stop(service: &str) -> bool {
        Command::new("systemctl")
            .args(["--user", "stop", service])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn restart(service: &str) -> bool {
        Command::new("systemctl")
            .args(["--user", "restart", service])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn enable(service: &str) -> bool {
        Command::new("systemctl")
            .args(["--user", "enable", service])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn disable(service: &str) -> bool {
        Command::new("systemctl")
            .args(["--user", "disable", service])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn status(service: &str) -> Option<String> {
        Command::new("systemctl")
            .args(["--user", "status", service])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
    }
}
