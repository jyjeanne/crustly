---
type: Rust Function
title: mock_server
resource: src/memory/embed.rs#L327-L346
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
---

# Signature

`async fn mock_server( status: &'static str, body: String, ) -> (String, tokio::sync::oneshot::Receiver<String>)`