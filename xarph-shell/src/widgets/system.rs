use super::ShellWidget;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, Widget, glib};
use std::fs;
use std::sync::mpsc;
use std::thread;

/// CPU and RAM usage widget.
/// Reads /proc/stat and /proc/meminfo — no external dependencies.
pub struct SystemWidget {
    /// Refresh interval in milliseconds.
    pub interval_ms: u64,
    /// Whether to show CPU percentage.
    pub show_cpu: bool,
    /// Whether to show RAM usage.
    pub show_ram: bool,
}

impl Default for SystemWidget {
    fn default() -> Self {
        Self {
            interval_ms: 2000,
            show_cpu: true,
            show_ram: true,
        }
    }
}

#[derive(Clone, Default, Debug)]
struct SysStats {
    cpu_pct: f32,
    ram_used_mb: u64,
    ram_total_mb: u64,
}

fn read_stats(prev_idle: &mut u64, prev_total: &mut u64) -> SysStats {
    let mut stats = SysStats::default();

    // ── CPU ─────────────────────────────────────────────────────────────────
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        if let Some(line) = content.lines().next() {
            // "cpu  u n s i w x y z"
            let nums: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() >= 4 {
                let idle = nums[3];
                let total: u64 = nums.iter().sum();
                let d_total = total.saturating_sub(*prev_total);
                let d_idle = idle.saturating_sub(*prev_idle);
                if d_total > 0 {
                    stats.cpu_pct = ((d_total - d_idle) as f32 / d_total as f32) * 100.0;
                }
                *prev_idle = idle;
                *prev_total = total;
            }
        }
    }

    // ── RAM ─────────────────────────────────────────────────────────────────
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total_kb: u64 = 0;
        let mut available_kb: u64 = 0;
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("MemTotal:") => {
                    total_kb = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0)
                }
                Some("MemAvailable:") => {
                    available_kb = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0)
                }
                _ => {}
            }
        }
        stats.ram_total_mb = total_kb / 1024;
        stats.ram_used_mb = (total_kb - available_kb) / 1024;
    }

    stats
}

impl ShellWidget for SystemWidget {
    fn build(&self) -> Widget {
        let container = Box::new(Orientation::Horizontal, 6);
        container.add_css_class("system-widget");

        let cpu_label = Label::builder()
            .css_classes(vec!["system-label".to_string()])
            .build();
        let ram_label = Label::builder()
            .css_classes(vec!["system-label".to_string()])
            .build();

        if self.show_cpu {
            container.append(&Label::new(Some("󰍛")));
            container.append(&cpu_label);
        }
        if self.show_ram {
            container.append(&Label::new(Some("󰘚")));
            container.append(&ram_label);
        }

        let show_cpu = self.show_cpu;
        let show_ram = self.show_ram;
        let interval = self.interval_ms;

        let (tx, rx) = mpsc::channel::<SysStats>();

        thread::spawn(move || {
            let mut prev_idle = 0u64;
            let mut prev_total = 0u64;
            loop {
                let s = read_stats(&mut prev_idle, &mut prev_total);
                let _ = tx.send(s);
                thread::sleep(std::time::Duration::from_millis(interval));
            }
        });

        glib::timeout_add_local(std::time::Duration::from_millis(interval), move || {
            while let Ok(s) = rx.try_recv() {
                if show_cpu {
                    cpu_label.set_text(&format!("{:.0}%", s.cpu_pct));
                }
                if show_ram {
                    ram_label.set_text(&format!("{}/{} MB", s.ram_used_mb, s.ram_total_mb));
                }
            }
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}
