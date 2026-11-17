//! Helpers shared by the provider tests.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::oneshot;

/// A one-shot HTTP server that sends `first`, then holds the connection open
/// until `release` fires, then sends `rest` and closes.
///
/// A provider that really streams yields the events for `first` while the
/// server is still holding; one that reads the whole body before yielding
/// anything cannot, which is what the streaming regression tests check.
/// Returns the server's base URL, e.g. `http://127.0.0.1:12345`.
pub(crate) async fn held_response_server(
    content_type: &'static str,
    first: String,
    rest: String,
    release: oneshot::Receiver<()>,
) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind mock server");
    let addr = listener.local_addr().expect("mock server addr");

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept request");
        read_request(&mut socket).await;

        let head =
            format!("HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\nconnection: close\r\n\r\n");
        socket.write_all(head.as_bytes()).await.expect("write head");
        socket
            .write_all(first.as_bytes())
            .await
            .expect("write first part");
        socket.flush().await.expect("flush first part");

        // If the test ends without releasing (a failure), just close.
        if release.await.is_ok() {
            let _ = socket.write_all(rest.as_bytes()).await;
        }
        let _ = socket.shutdown().await;
    });

    format!("http://{addr}")
}

/// Reads the request head and its `Content-Length` body, so closing the
/// socket later does not reset a connection that still has unread data.
async fn read_request(socket: &mut tokio::net::TcpStream) {
    let mut request = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = socket.read(&mut buf).await.expect("read request");
        if n == 0 {
            return;
        }
        request.extend_from_slice(&buf[..n]);
        let Some(head_end) = request.windows(4).position(|w| w == b"\r\n\r\n") else {
            continue;
        };
        let head = String::from_utf8_lossy(&request[..head_end]).to_ascii_lowercase();
        let body_len = head
            .lines()
            .find_map(|line| line.strip_prefix("content-length:"))
            .and_then(|value| value.trim().parse::<usize>().ok())
            .unwrap_or(0);
        while request.len() < head_end + 4 + body_len {
            let n = socket.read(&mut buf).await.expect("read request body");
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buf[..n]);
        }
        return;
    }
}
