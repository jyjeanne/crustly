---
type: Rust Module
title: hardware_detect
resource: src/llm/provider/hardware_detect.rs#L1-L750
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-process-command-output-stdio
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-oncelock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-instant
    resolved_by: tree-sitter
    confidence: exact
  - target: external/windows-win32-graphics-dxgi-createdxgifactory1-idxgifactory1-dxgi-adapter-flag-software
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [GpuVendor](../../../../classes/src/llm/provider/hardware_detect/GpuVendor.md)
- [GpuInfo](../../../../classes/src/llm/provider/hardware_detect/GpuInfo.md)
- [vram_available_bytes](../../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)
- [HardwareInfo](../../../../classes/src/llm/provider/hardware_detect/HardwareInfo.md)
- [budget_bytes](../../../../functions/src/llm/provider/hardware_detect/HardwareInfo/budget_bytes.md)
- [detect_hardware](../../../../functions/src/llm/provider/hardware_detect/detect_hardware.md)
- [detect_hardware_uncached](../../../../functions/src/llm/provider/hardware_detect/detect_hardware_uncached.md)
- [run_with_timeout](../../../../functions/src/llm/provider/hardware_detect/run_with_timeout.md)
- [run_and_capture_stdout](../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [detect_nvidia](../../../../functions/src/llm/provider/hardware_detect/detect_nvidia.md)
- [parse_nvidia_smi_csv](../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)
- [detect_amd_rocm](../../../../functions/src/llm/provider/hardware_detect/detect_amd_rocm.md)
- [parse_rocm_smi_json](../../../../functions/src/llm/provider/hardware_detect/parse_rocm_smi_json.md)
- [gpu_vendor_from_pci_id](../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id.md)
- [trim_dxgi_description](../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description.md)
- [detect_windows_dxgi](../../../../functions/src/llm/provider/hardware_detect/detect_windows_dxgi.md)
- [detect_windows_dxgi](../../../../functions/src/llm/provider/hardware_detect/detect_windows_dxgi-2.md)
- [detect_apple_display](../../../../functions/src/llm/provider/hardware_detect/detect_apple_display.md)
- [parse_system_profiler_json](../../../../functions/src/llm/provider/hardware_detect/parse_system_profiler_json.md)
- [parse_size_string](../../../../functions/src/llm/provider/hardware_detect/parse_size_string.md)
- [detect_vulkan_fallback](../../../../functions/src/llm/provider/hardware_detect/detect_vulkan_fallback.md)
- [parse_vulkaninfo_summary](../../../../functions/src/llm/provider/hardware_detect/parse_vulkaninfo_summary.md)
- [system_ram_total_bytes](../../../../functions/src/llm/provider/hardware_detect/system_ram_total_bytes.md)
- [parses_a_real_shaped_nvidia_smi_line](../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_nvidia_smi_line.md)
- [nvidia_smi_only_reads_the_first_gpu_line](../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_only_reads_the_first_gpu_line.md)
- [nvidia_smi_empty_output_is_none](../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_empty_output_is_none.md)
- [nvidia_smi_line_with_no_usable_fields_is_none](../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_line_with_no_usable_fields_is_none.md)
- [nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable](../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable.md)
- [parses_a_real_shaped_rocm_smi_json_blob](../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_rocm_smi_json_blob.md)
- [rocm_smi_malformed_json_is_none](../../../../functions/src/llm/provider/hardware_detect/rocm_smi_malformed_json_is_none.md)
- [parses_a_real_shaped_system_profiler_json_blob](../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_system_profiler_json_blob.md)
- [system_profiler_name_only_when_no_vram_key_present](../../../../functions/src/llm/provider/hardware_detect/system_profiler_name_only_when_no_vram_key_present.md)
- [system_profiler_malformed_or_empty_is_none](../../../../functions/src/llm/provider/hardware_detect/system_profiler_malformed_or_empty_is_none.md)
- [parses_a_real_shaped_vulkaninfo_summary](../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_vulkaninfo_summary.md)
- [vulkaninfo_no_device_name_line_is_none](../../../../functions/src/llm/provider/hardware_detect/vulkaninfo_no_device_name_line_is_none.md)
- [parse_size_string_handles_mb_and_gb](../../../../functions/src/llm/provider/hardware_detect/parse_size_string_handles_mb_and_gb.md)
- [parse_size_string_rejects_unknown_units](../../../../functions/src/llm/provider/hardware_detect/parse_size_string_rejects_unknown_units.md)
- [vram_available_falls_back_to_total_when_used_is_unknown](../../../../functions/src/llm/provider/hardware_detect/vram_available_falls_back_to_total_when_used_is_unknown.md)
- [vram_available_is_none_when_total_is_unknown](../../../../functions/src/llm/provider/hardware_detect/vram_available_is_none_when_total_is_unknown.md)
- [budget_bytes_sums_vram_and_ram_when_both_known](../../../../functions/src/llm/provider/hardware_detect/budget_bytes_sums_vram_and_ram_when_both_known.md)
- [budget_bytes_is_ram_only_when_no_gpu](../../../../functions/src/llm/provider/hardware_detect/budget_bytes_is_ram_only_when_no_gpu.md)
- [budget_bytes_is_none_when_nothing_was_determined](../../../../functions/src/llm/provider/hardware_detect/budget_bytes_is_none_when_nothing_was_determined.md)
- [run_with_timeout_returns_none_for_a_nonexistent_binary](../../../../functions/src/llm/provider/hardware_detect/run_with_timeout_returns_none_for_a_nonexistent_binary.md)
- [run_with_timeout_returns_output_for_a_fast_real_command](../../../../functions/src/llm/provider/hardware_detect/run_with_timeout_returns_output_for_a_fast_real_command.md)
- [gpu_vendor_from_pci_id_recognizes_the_three_named_vendors](../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_recognizes_the_three_named_vendors.md)
- [gpu_vendor_from_pci_id_falls_back_to_other_for_an_unrecognized_id](../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_falls_back_to_other_for_an_unrecognized_id.md)
- [trim_dxgi_description_stops_at_the_first_nul](../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description_stops_at_the_first_nul.md)
- [trim_dxgi_description_handles_a_buffer_with_no_nul_terminator](../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description_handles_a_buffer_with_no_nul_terminator.md)
- [detect_hardware_is_cached_across_calls](../../../../functions/src/llm/provider/hardware_detect/detect_hardware_is_cached_across_calls.md)
- [system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine](../../../../functions/src/llm/provider/hardware_detect/system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine.md)

# Imports

- `std::process::{Command, Output, Stdio}`
- `std::sync::OnceLock`
- `std::time::{Duration, Instant}`
- `windows::Win32::Graphics::Dxgi::{
        CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE,
    }`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)