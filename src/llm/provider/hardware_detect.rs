//! Best-effort local hardware detection (GPU vendor/VRAM, system RAM) for
//! Phase M11 of `ccguf-managment-imrpoment-plan.md` - read-only and
//! advisory, same posture as the rest of this module's siblings
//! (`gguf_metadata`, `llama_cpp_models`): a missing tool, a timeout, a
//! non-zero exit, or unparseable output all degrade to "unknown," never an
//! error, and this never writes back into `LlamaCppProviderConfig`.
//!
//! Deliberately subprocess-based for every vendor path except one (rather
//! than SDK/FFI bindings) - consistent with Phase M0's whole point that the
//! management surface shouldn't need a build-time C/C++ toolchain. The one
//! exception the plan calls for, Windows AMD/Intel via the Win32 DXGI API,
//! is a native FFI binding, not a subprocess, and this development
//! environment has no Windows target to build or test it against; writing
//! `unsafe` COM interface calls with zero ability to verify they even
//! compile for their own target is worse than being explicit about the
//! gap. `detect_windows_dxgi` below is a documented stub returning `None`
//! for this phase - it falls through to the CPU-only/system-RAM path,
//! which is this module's own designed-in "clean degradation," not a
//! crash. NVIDIA-on-Windows is unaffected (`nvidia-smi` is a subprocess,
//! fully portable). See `ccguf-managment-imrpoment-plan.md` Phase M11 for
//! the full writeup of this scope decision.
//!
//! Detection runs at most once per process: [`detect_hardware`] caches its
//! result in a `OnceLock`. Callers that don't need hardware data (a plain
//! `crustly llama-cpp list`, or the TUI dialog's steady-state redraws)
//! must never call this - only `doctor`, `list --best-fit`, and the TUI
//! host-info panel's first load do.

use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// How long a single vendor-tool subprocess is allowed to run before it's
/// killed and treated as "unknown" - long enough for a real tool under
/// normal conditions, short enough not to visibly stall `doctor` or the
/// TUI's first paint on a hung driver.
const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

/// How often [`run_with_timeout`] polls the child for completion.
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// The vendor a detected GPU belongs to. `Other` carries whatever name the
/// Vulkan cross-vendor fallback reported, when no vendor-specific path
/// resolved anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Apple,
    Other(String),
}

/// A detected GPU. Every field beyond `vendor` is `Option` - a source that
/// reports a name but not memory (e.g. the Vulkan fallback) still produces
/// a usable, honestly-partial `GpuInfo`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuInfo {
    pub vendor: GpuVendor,
    pub name: Option<String>,
    pub vram_total_bytes: Option<u64>,
    /// Currently-used VRAM, when the source reports it. `nvidia-smi` does
    /// (`memory.used`); `rocm-smi`/`system_profiler`/`vulkaninfo` generally
    /// don't - `None` there, and [`GpuInfo::vram_available_bytes`] then
    /// conservatively falls back to the full total rather than guessing a
    /// used figure (which would risk under-reporting available memory,
    /// the wrong direction to be wrong in for a "will this fit" check).
    pub vram_used_bytes: Option<u64>,
}

impl GpuInfo {
    /// `vram_total_bytes - vram_used_bytes` when both are known, else the
    /// total as-is, else `None`.
    pub fn vram_available_bytes(&self) -> Option<u64> {
        match (self.vram_total_bytes, self.vram_used_bytes) {
            (Some(total), Some(used)) => Some(total.saturating_sub(used)),
            (Some(total), None) => Some(total),
            (None, _) => None,
        }
    }
}

/// The machine's detected hardware, as best `detect_hardware` could
/// determine it. `gpu: None` means no GPU path resolved anything - CPU-only,
/// not an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareInfo {
    pub gpu: Option<GpuInfo>,
    pub system_ram_total_bytes: Option<u64>,
}

impl HardwareInfo {
    /// The single number Phase M12's fit comparison (and `doctor`'s
    /// summary line) compares a model's estimated memory footprint
    /// against: available VRAM plus system RAM, reflecting that llama.cpp
    /// can split a model across both (offloaded layers in VRAM, the rest -
    /// plus most of the KV cache when not fully offloaded - in system
    /// RAM). Deliberately coarse, per M12's own "three states, not a
    /// false-precision percentage" framing - not a model of partial-offload
    /// performance. `None` only when *nothing* was determined at all (no
    /// GPU reading and no RAM reading) - every other combination returns a
    /// best-effort `Some`.
    pub fn budget_bytes(&self) -> Option<u64> {
        let vram = self.gpu.as_ref().and_then(GpuInfo::vram_available_bytes);
        match (vram, self.system_ram_total_bytes) {
            (None, None) => None,
            (vram, ram) => Some(vram.unwrap_or(0) + ram.unwrap_or(0)),
        }
    }
}

static HARDWARE_INFO: OnceLock<HardwareInfo> = OnceLock::new();

/// Detects (once per process - see the module doc's "when this runs" rule)
/// and returns the machine's hardware info. Blocking: spawns subprocesses
/// and waits on them, so callers on the async runtime must wrap this in
/// `tokio::task::spawn_blocking` (see `tui/llama_cpp_download.rs`'s
/// `detect_hardware_summary`).
pub fn detect_hardware() -> &'static HardwareInfo {
    HARDWARE_INFO.get_or_init(detect_hardware_uncached)
}

fn detect_hardware_uncached() -> HardwareInfo {
    let gpu = detect_nvidia()
        .or_else(detect_amd_rocm)
        .or_else(detect_windows_dxgi)
        .or_else(detect_apple_display)
        .or_else(detect_vulkan_fallback);

    HardwareInfo {
        gpu,
        system_ram_total_bytes: system_ram_total_bytes(),
    }
}

/// Runs `cmd`, killing and returning `None` if it doesn't finish within
/// `PROBE_TIMEOUT` - polls rather than blocking on `wait_with_output`
/// indefinitely, since a hung driver tool must never hang `doctor`/`list`.
/// Also returns `None` on any spawn failure (binary not found - the common
/// case in CI/most dev machines, and the primary path this module's own
/// tests can exercise portably without a real GPU).
fn run_with_timeout(cmd: &mut Command) -> Option<Output> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;

    let deadline = Instant::now() + PROBE_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => return child.wait_with_output().ok(),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
                std::thread::sleep(POLL_INTERVAL);
            }
            Err(_) => return None,
        }
    }
}

fn run_and_capture_stdout(cmd: &mut Command) -> Option<String> {
    let output = run_with_timeout(cmd)?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

// ---------------------------------------------------------------------
// NVIDIA (Linux/Windows) - `nvidia-smi`
// ---------------------------------------------------------------------

fn detect_nvidia() -> Option<GpuInfo> {
    let stdout = run_and_capture_stdout(Command::new("nvidia-smi").args([
        "--query-gpu=name,memory.total,memory.used",
        "--format=csv,noheader,nounits",
    ]))?;
    parse_nvidia_smi_csv(&stdout)
}

/// Parses `nvidia-smi --query-gpu=name,memory.total,memory.used
/// --format=csv,noheader,nounits` output, e.g.
/// `"NVIDIA GeForce RTX 4090, 24564, 1203"` (memory figures in MiB with
/// `nounits`). Only the first GPU line is used (multi-GPU budgeting is out
/// of scope - this module reports one representative GPU, matching M11's
/// "name, VRAM total" display framing, not a full topology).
fn parse_nvidia_smi_csv(stdout: &str) -> Option<GpuInfo> {
    let line = stdout.lines().find(|l| !l.trim().is_empty())?;
    let mut parts = line.split(',').map(str::trim);
    let name = parts.next().filter(|s| !s.is_empty()).map(str::to_string);
    let total_mib: Option<u64> = parts.next().and_then(|s| s.parse().ok());
    let used_mib: Option<u64> = parts.next().and_then(|s| s.parse().ok());
    if name.is_none() && total_mib.is_none() {
        return None;
    }
    Some(GpuInfo {
        vendor: GpuVendor::Nvidia,
        name,
        vram_total_bytes: total_mib.map(|mib| mib * 1024 * 1024),
        vram_used_bytes: used_mib.map(|mib| mib * 1024 * 1024),
    })
}

// ---------------------------------------------------------------------
// AMD (Linux) - `rocm-smi`
// ---------------------------------------------------------------------

fn detect_amd_rocm() -> Option<GpuInfo> {
    let stdout =
        run_and_capture_stdout(Command::new("rocm-smi").args(["--showmeminfo", "vram", "--json"]))?;
    parse_rocm_smi_json(&stdout)
}

/// Parses `rocm-smi --showmeminfo vram --json` output, e.g.:
/// ```json
/// {"card0": {"VRAM Total Memory (B)": "25757220864", "VRAM Total Used Memory (B)": "512000000"}}
/// ```
/// Hand-rolled key lookup (not `serde_json::Value` field-by-struct
/// deserialization) since `rocm-smi`'s top-level key is a variable card
/// name (`"card0"`, `"card1"`, ...), not a fixed schema - the first card
/// object found is used, same "one representative GPU" scope as the
/// NVIDIA path above.
fn parse_rocm_smi_json(stdout: &str) -> Option<GpuInfo> {
    let value: serde_json::Value = serde_json::from_str(stdout).ok()?;
    let card = value.as_object()?.values().next()?.as_object()?;
    let total_bytes = card
        .get("VRAM Total Memory (B)")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u64>().ok());
    let used_bytes = card
        .get("VRAM Total Used Memory (B)")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u64>().ok());
    total_bytes.map(|total| GpuInfo {
        vendor: GpuVendor::Amd,
        name: None,
        vram_total_bytes: Some(total),
        vram_used_bytes: used_bytes,
    })
}

// ---------------------------------------------------------------------
// AMD/Intel (Windows) - Win32 DXGI. See the module doc: a documented stub
// for this phase, not a real implementation - no Windows target available
// in this development environment to build or verify FFI bindings against.
// ---------------------------------------------------------------------

fn detect_windows_dxgi() -> Option<GpuInfo> {
    None
}

// ---------------------------------------------------------------------
// Apple (macOS) - `system_profiler`
// ---------------------------------------------------------------------

fn detect_apple_display() -> Option<GpuInfo> {
    let stdout = run_and_capture_stdout(
        Command::new("system_profiler").args(["SPDisplaysDataType", "-json"]),
    )?;
    parse_system_profiler_json(&stdout)
}

/// Parses `system_profiler SPDisplaysDataType -json` output. Apple Silicon
/// reports unified memory as `"spdisplays_vram_shared"` (or similar,
/// varies by macOS version) rather than a fixed hardware chip name - this
/// looks for `sppci_model`/`_name` for the chip name and any
/// `*vram*`-suffixed key with a parseable `"N MB"`/`"N GB"` value under the
/// first display entry, degrading to name-only when no such key is
/// present (unified-memory total is still available separately via
/// `system_ram_total_bytes`, so a missing VRAM figure here isn't fatal to
/// the overall `HardwareInfo`).
fn parse_system_profiler_json(stdout: &str) -> Option<GpuInfo> {
    let value: serde_json::Value = serde_json::from_str(stdout).ok()?;
    let entry = value
        .get("SPDisplaysDataType")?
        .as_array()?
        .first()?
        .as_object()?;

    let name = entry
        .get("sppci_model")
        .or_else(|| entry.get("_name"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let vram_total_bytes = entry
        .iter()
        .find(|(k, _)| k.to_lowercase().contains("vram"))
        .and_then(|(_, v)| v.as_str())
        .and_then(parse_size_string);

    if name.is_none() && vram_total_bytes.is_none() {
        return None;
    }
    Some(GpuInfo {
        vendor: GpuVendor::Apple,
        name,
        vram_total_bytes,
        vram_used_bytes: None,
    })
}

/// Parses a human-readable size string like `"24576 MB"`/`"24 GB"` into
/// bytes. Used by the Apple `system_profiler` path, whose memory figures
/// are unit-suffixed strings rather than a fixed-unit number the way
/// `nvidia-smi`'s CSV output is.
fn parse_size_string(s: &str) -> Option<u64> {
    let s = s.trim();
    let (number, unit) = s.split_once(' ')?;
    let number: f64 = number.parse().ok()?;
    let multiplier: f64 = match unit.to_uppercase().as_str() {
        "MB" | "MIB" => 1024.0 * 1024.0,
        "GB" | "GIB" => 1024.0 * 1024.0 * 1024.0,
        _ => return None,
    };
    Some((number * multiplier) as u64)
}

// ---------------------------------------------------------------------
// Cross-vendor fallback - `vulkaninfo`. No VRAM figure available this way
// (matches llamastash's own documented limitation for this path) - name
// only, so at least *something* better than "CPU-only" shows when a GPU
// exists but none of the vendor-specific tools above are installed.
// ---------------------------------------------------------------------

fn detect_vulkan_fallback() -> Option<GpuInfo> {
    let stdout = run_and_capture_stdout(Command::new("vulkaninfo").arg("--summary"))?;
    parse_vulkaninfo_summary(&stdout)
}

/// Parses `vulkaninfo --summary` output for the first `deviceName` line,
/// e.g. `"deviceName        = NVIDIA GeForce RTX 4090"`. Only reached when
/// no vendor-specific detection above resolved anything, so this is the
/// last-resort "there's *a* GPU, we don't know its VRAM" signal.
fn parse_vulkaninfo_summary(stdout: &str) -> Option<GpuInfo> {
    let name = stdout
        .lines()
        .find_map(|l| l.trim().strip_prefix("deviceName"))
        .and_then(|rest| rest.trim_start_matches([' ', '=']).split('\n').next())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)?;
    Some(GpuInfo {
        vendor: GpuVendor::Other("vulkan".to_string()),
        name: Some(name),
        vram_total_bytes: None,
        vram_used_bytes: None,
    })
}

// ---------------------------------------------------------------------
// System RAM (all platforms) - `sysinfo`
// ---------------------------------------------------------------------

/// Total system RAM in bytes, via `sysinfo` (see this crate's own
/// `"system"` feature, added in `Cargo.toml` specifically for this
/// function - a RAM-total query is commodity functionality a well-
/// maintained pure-Rust crate already solves portably, not worth
/// reimplementing per-OS `/proc/meminfo`/`sysctl`/WMI parsing by hand).
/// `None` only if the underlying platform call itself reports zero, which
/// `sysinfo` treats as "couldn't determine" on every supported target.
fn system_ram_total_bytes() -> Option<u64> {
    let sys = sysinfo::System::new_all();
    let total = sys.total_memory();
    (total > 0).then_some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- nvidia-smi -----------------------------------------------------

    #[test]
    fn parses_a_real_shaped_nvidia_smi_line() {
        let gpu = parse_nvidia_smi_csv("NVIDIA GeForce RTX 4090, 24564, 1203\n").unwrap();
        assert_eq!(gpu.vendor, GpuVendor::Nvidia);
        assert_eq!(gpu.name.as_deref(), Some("NVIDIA GeForce RTX 4090"));
        assert_eq!(gpu.vram_total_bytes, Some(24564 * 1024 * 1024));
        assert_eq!(gpu.vram_used_bytes, Some(1203 * 1024 * 1024));
        assert_eq!(
            gpu.vram_available_bytes(),
            Some((24564 - 1203) * 1024 * 1024)
        );
    }

    #[test]
    fn nvidia_smi_only_reads_the_first_gpu_line() {
        let gpu = parse_nvidia_smi_csv(
            "NVIDIA GeForce RTX 4090, 24564, 1203\nNVIDIA GeForce RTX 3090, 24576, 0\n",
        )
        .unwrap();
        assert_eq!(gpu.name.as_deref(), Some("NVIDIA GeForce RTX 4090"));
    }

    #[test]
    fn nvidia_smi_empty_output_is_none() {
        assert!(parse_nvidia_smi_csv("").is_none());
        assert!(parse_nvidia_smi_csv("\n\n").is_none());
    }

    #[test]
    fn nvidia_smi_line_with_no_usable_fields_is_none() {
        assert!(parse_nvidia_smi_csv(",,\n").is_none());
    }

    #[test]
    fn nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable() {
        // A name-bearing but memory-unparseable line still produces a
        // usable, honestly-partial GpuInfo rather than discarding the name
        // too - same "partial data beats none" posture as the rest of this
        // module's parsers.
        let gpu = parse_nvidia_smi_csv("Some GPU,not-a-number,also-not\n").unwrap();
        assert_eq!(gpu.name.as_deref(), Some("Some GPU"));
        assert_eq!(gpu.vram_total_bytes, None);
        assert_eq!(gpu.vram_used_bytes, None);
    }

    // -- rocm-smi ---------------------------------------------------------

    #[test]
    fn parses_a_real_shaped_rocm_smi_json_blob() {
        let json = r#"{"card0": {"VRAM Total Memory (B)": "25757220864", "VRAM Total Used Memory (B)": "512000000"}}"#;
        let gpu = parse_rocm_smi_json(json).unwrap();
        assert_eq!(gpu.vendor, GpuVendor::Amd);
        assert_eq!(gpu.vram_total_bytes, Some(25_757_220_864));
        assert_eq!(gpu.vram_used_bytes, Some(512_000_000));
    }

    #[test]
    fn rocm_smi_malformed_json_is_none() {
        assert!(parse_rocm_smi_json("not json").is_none());
        assert!(parse_rocm_smi_json("{}").is_none());
        assert!(parse_rocm_smi_json(r#"{"card0": {}}"#).is_none());
    }

    // -- system_profiler --------------------------------------------------

    #[test]
    fn parses_a_real_shaped_system_profiler_json_blob() {
        let json = r#"{
            "SPDisplaysDataType": [
                {
                    "sppci_model": "Apple M3 Max",
                    "spdisplays_vram_shared": "48 GB"
                }
            ]
        }"#;
        let gpu = parse_system_profiler_json(json).unwrap();
        assert_eq!(gpu.vendor, GpuVendor::Apple);
        assert_eq!(gpu.name.as_deref(), Some("Apple M3 Max"));
        assert_eq!(gpu.vram_total_bytes, Some(48 * 1024 * 1024 * 1024));
    }

    #[test]
    fn system_profiler_name_only_when_no_vram_key_present() {
        let json = r#"{"SPDisplaysDataType": [{"sppci_model": "Intel Iris"}]}"#;
        let gpu = parse_system_profiler_json(json).unwrap();
        assert_eq!(gpu.name.as_deref(), Some("Intel Iris"));
        assert_eq!(gpu.vram_total_bytes, None);
    }

    #[test]
    fn system_profiler_malformed_or_empty_is_none() {
        assert!(parse_system_profiler_json("not json").is_none());
        assert!(parse_system_profiler_json(r#"{"SPDisplaysDataType": []}"#).is_none());
        assert!(parse_system_profiler_json("{}").is_none());
    }

    // -- vulkaninfo ---------------------------------------------------------

    #[test]
    fn parses_a_real_shaped_vulkaninfo_summary() {
        let text = "Devices:\n========\nGPU0:\n\tdeviceName        = NVIDIA GeForce RTX 4090\n\tdriverVersion     = 550.54.14\n";
        let gpu = parse_vulkaninfo_summary(text).unwrap();
        assert_eq!(gpu.vendor, GpuVendor::Other("vulkan".to_string()));
        assert_eq!(gpu.name.as_deref(), Some("NVIDIA GeForce RTX 4090"));
        assert_eq!(gpu.vram_total_bytes, None);
    }

    #[test]
    fn vulkaninfo_no_device_name_line_is_none() {
        assert!(parse_vulkaninfo_summary("Devices:\n========\n").is_none());
    }

    // -- parse_size_string --------------------------------------------------

    #[test]
    fn parse_size_string_handles_mb_and_gb() {
        assert_eq!(parse_size_string("1536 MB"), Some(1536 * 1024 * 1024));
        assert_eq!(parse_size_string("48 GB"), Some(48 * 1024 * 1024 * 1024));
    }

    #[test]
    fn parse_size_string_rejects_unknown_units() {
        assert_eq!(parse_size_string("48 furlongs"), None);
        assert_eq!(parse_size_string("garbage"), None);
    }

    // -- GpuInfo::vram_available_bytes / HardwareInfo::budget_bytes -------

    #[test]
    fn vram_available_falls_back_to_total_when_used_is_unknown() {
        let gpu = GpuInfo {
            vendor: GpuVendor::Nvidia,
            name: None,
            vram_total_bytes: Some(1000),
            vram_used_bytes: None,
        };
        assert_eq!(gpu.vram_available_bytes(), Some(1000));
    }

    #[test]
    fn vram_available_is_none_when_total_is_unknown() {
        let gpu = GpuInfo {
            vendor: GpuVendor::Nvidia,
            name: None,
            vram_total_bytes: None,
            vram_used_bytes: Some(500),
        };
        assert_eq!(gpu.vram_available_bytes(), None);
    }

    #[test]
    fn budget_bytes_sums_vram_and_ram_when_both_known() {
        let info = HardwareInfo {
            gpu: Some(GpuInfo {
                vendor: GpuVendor::Nvidia,
                name: None,
                vram_total_bytes: Some(10_000),
                vram_used_bytes: Some(2_000),
            }),
            system_ram_total_bytes: Some(20_000),
        };
        assert_eq!(info.budget_bytes(), Some(8_000 + 20_000));
    }

    #[test]
    fn budget_bytes_is_ram_only_when_no_gpu() {
        let info = HardwareInfo {
            gpu: None,
            system_ram_total_bytes: Some(16_000),
        };
        assert_eq!(info.budget_bytes(), Some(16_000));
    }

    #[test]
    fn budget_bytes_is_none_when_nothing_was_determined() {
        let info = HardwareInfo {
            gpu: None,
            system_ram_total_bytes: None,
        };
        assert_eq!(info.budget_bytes(), None);
    }

    // -- run_with_timeout / spawn-failure degradation ----------------------

    #[test]
    fn run_with_timeout_returns_none_for_a_nonexistent_binary() {
        // Portable across CI/dev machines without depending on wall-clock
        // timeout behavior of a real hung process - exercises the "missing
        // tool" branch every vendor-detection function above degrades
        // through, deterministically.
        let out = run_with_timeout(&mut Command::new(
            "definitely-not-a-real-binary-crustly-hardware-detect-test",
        ));
        assert!(out.is_none());
    }

    #[test]
    fn run_with_timeout_returns_output_for_a_fast_real_command() {
        let mut cmd = Command::new("echo");
        cmd.arg("hello");
        let out = run_with_timeout(&mut cmd);
        // `echo` may not exist on every CI image (e.g. minimal Windows
        // runners); only assert when it actually ran, same "don't assume
        // tool availability" posture as the vendor probes themselves.
        if let Some(out) = out {
            assert!(out.status.success());
            assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "hello");
        }
    }

    // -- detect_hardware caching -------------------------------------------

    #[test]
    fn detect_hardware_is_cached_across_calls() {
        let first = detect_hardware() as *const HardwareInfo;
        let second = detect_hardware() as *const HardwareInfo;
        assert_eq!(first, second, "expected the same cached &'static instance");
    }

    // -- system_ram_total_bytes ----------------------------------------------

    #[test]
    fn system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine() {
        // Best-effort - assert only the shape (a real machine reports a
        // non-zero figure), not a specific number.
        if let Some(bytes) = system_ram_total_bytes() {
            assert!(bytes > 0);
        }
    }
}
