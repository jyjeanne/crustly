---
type: Rust Function
title: detect_windows_dxgi
resource: src/llm/provider/hardware_detect.rs#L317-L357
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/trim_dxgi_description
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_windows_dxgi() -> Option<GpuInfo>`

# Calls

- [gpu_vendor_from_pci_id](../../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id.md)
- [trim_dxgi_description](../../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description.md)