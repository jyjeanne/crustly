---
type: Rust Function
title: gpu_vendor_from_pci_id
resource: src/llm/provider/hardware_detect.rs#L298-L305
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_windows_dxgi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_recognizes_the_three_named_vendors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_falls_back_to_other_for_an_unrecognized_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn gpu_vendor_from_pci_id(vendor_id: u32) -> GpuVendor`

# Called by

- [detect_windows_dxgi](../../../../../functions/src/llm/provider/hardware_detect/detect_windows_dxgi.md)
- [gpu_vendor_from_pci_id_recognizes_the_three_named_vendors](../../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_recognizes_the_three_named_vendors.md)
- [gpu_vendor_from_pci_id_falls_back_to_other_for_an_unrecognized_id](../../../../../functions/src/llm/provider/hardware_detect/gpu_vendor_from_pci_id_falls_back_to_other_for_an_unrecognized_id.md)