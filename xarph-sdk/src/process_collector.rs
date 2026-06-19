#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_usage: f32,
    pub memory_kb: u64,
    pub status: String,
}

pub struct ProcessCollector;

impl ProcessCollector {
    pub fn fetch_all() -> Vec<ProcessInfo> {
        let mut processes = Vec::new();
        let proc_dir = match std::fs::read_dir("/proc") {
            Ok(d) => d,
            Err(_) => return processes,
        };

        for entry in proc_dir.flatten() {
            let name = entry.file_name();
            let pid_str = match name.to_str() {
                Some(s) => s,
                None => continue,
            };
            let pid: u32 = match pid_str.parse() {
                Ok(p) => p,
                Err(_) => continue,
            };
            if let Some(info) = Self::read_process(pid) {
                processes.push(info);
            }
        }

        processes.sort_by(|a, b| b.memory_kb.cmp(&a.memory_kb));
        processes
    }

    fn read_process(pid: u32) -> Option<ProcessInfo> {
        let stat_path = format!("/proc/{pid}/stat");
        let stat = std::fs::read_to_string(&stat_path).ok()?;

        let comm_end = stat.rfind(')')?;
        let fields: Vec<&str> = stat[comm_end + 2..].split_whitespace().collect();
        if fields.len() < 20 {
            return None;
        }

        let state = fields[0];
        let utime: u64 = fields[11].parse().unwrap_or(0);
        let stime: u64 = fields[12].parse().unwrap_or(0);
        let cpu_ticks = utime + stime;

        let status_path = format!("/proc/{pid}/status");
        let status_text = std::fs::read_to_string(&status_path).unwrap_or_default();

        let vm_rss: u64 = status_text
            .lines()
            .find(|l| l.starts_with("VmRSS:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let uid = status_text
            .lines()
            .find(|l| l.starts_with("Uid:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0);

        let user = Self::resolve_uid(uid);
        let comm = stat[1..comm_end].to_string();

        let clk_tck: u64 = 100;
        let uptime_secs = std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
            .unwrap_or(1.0);
        let cpu_secs = cpu_ticks as f64 / clk_tck as f64;
        let cpu_pct = if uptime_secs > 0.0 {
            (cpu_secs / uptime_secs * 100.0) as f32
        } else {
            0.0
        };

        let status_char = match state {
            "R" => "Running",
            "S" => "Sleeping",
            "D" => "Disk Sleep",
            "Z" => "Zombie",
            "T" => "Stopped",
            "t" => "Tracing",
            "I" => "Idle",
            _ => state,
        };

        Some(ProcessInfo {
            pid,
            name: comm,
            user,
            cpu_usage: cpu_pct,
            memory_kb: vm_rss,
            status: status_char.to_string(),
        })
    }

    fn resolve_uid(uid: u32) -> String {
        if let Ok(passwd) = std::fs::read_to_string("/etc/passwd") {
            for line in passwd.lines() {
                let fields: Vec<&str> = line.split(':').collect();
                if fields.len() >= 3 {
                    if let Ok(fuid) = fields[2].parse::<u32>() {
                        if fuid == uid {
                            return fields[0].to_string();
                        }
                    }
                }
            }
        }
        format!("{uid}")
    }

    pub fn kill(pid: u32, signal: i32) -> bool {
        unsafe { libc::kill(pid as i32, signal) == 0 }
    }

    pub fn terminate(pid: u32) -> bool {
        Self::kill(pid, libc::SIGTERM)
    }

    pub fn kill_force(pid: u32) -> bool {
        Self::kill(pid, libc::SIGKILL)
    }

    pub fn system_stats() -> SystemStats {
        let mut stats = SystemStats::default();

        // Memory
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if let Some(val) = line.strip_prefix("MemTotal:") {
                    stats.total_memory_kb = val.split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0);
                } else if let Some(val) = line.strip_prefix("MemAvailable:") {
                    stats.available_memory_kb = val.split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0);
                }
            }
        }

        // Load average
        if let Ok(loadavg) = std::fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = loadavg.split_whitespace().collect();
            if parts.len() >= 3 {
                stats.load_1m = parts[0].parse().unwrap_or(0.0);
                stats.load_5m = parts[1].parse().unwrap_or(0.0);
                stats.load_15m = parts[2].parse().unwrap_or(0.0);
            }
        }

        stats
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemStats {
    pub total_memory_kb: u64,
    pub available_memory_kb: u64,
    pub load_1m: f64,
    pub load_5m: f64,
    pub load_15m: f64,
}

impl SystemStats {
    pub fn used_memory_kb(&self) -> u64 {
        self.total_memory_kb.saturating_sub(self.available_memory_kb)
    }

    pub fn memory_usage_percent(&self) -> f32 {
        if self.total_memory_kb == 0 {
            return 0.0;
        }
        (self.used_memory_kb() as f32 / self.total_memory_kb as f32) * 100.0
    }
}
