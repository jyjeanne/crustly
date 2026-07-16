//! SSRF protection for network-fetching tools (`web_fetch`, `http`).
//!
//! Neither tool checked the target host against loopback/link-local/private/
//! cloud-metadata address ranges. `web_fetch` in particular requires no
//! approval ("read-only fetch"), so a model - steered by content it already
//! fetched or read - could autonomously issue a request to
//! `http://169.254.169.254/latest/meta-data/iam/security-credentials/<role>`
//! (the AWS/GCP/Azure metadata endpoint) or an internal-only admin endpoint,
//! with the response fed straight back into its own context.
//!
//! Two layers are needed, not one:
//! - A URL whose host is already an IP literal (`http://169.254.169.254/...`)
//!   never goes through DNS resolution at all, so it must be checked before
//!   the request is ever built.
//! - A URL whose host is a domain name is only actually dangerous once
//!   resolved - and a plain "resolve once to check, then let the HTTP client
//!   resolve again to connect" is a TOCTOU gap (DNS rebinding: the second
//!   resolution can return a different, internal address). A custom
//!   [`reqwest::dns::Resolve`] closes this because it is the *same*
//!   resolution reqwest actually connects with.

use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;

/// Whether `ip` falls in a loopback/link-local/private/documentation/
/// multicast/unspecified range - i.e. anywhere outside the public internet
/// that a network-fetching tool has no legitimate reason to reach.
pub fn is_blocked_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_blocked_ipv4(v4),
        IpAddr::V6(v6) => is_blocked_ipv6(v6),
    }
}

fn is_blocked_ipv4(ip: &Ipv4Addr) -> bool {
    if ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local() // covers the 169.254.169.254 cloud metadata endpoint
        || ip.is_multicast()
        || ip.is_broadcast()
        || ip.is_unspecified()
        || ip.is_documentation()
    {
        return true;
    }
    // 100.64.0.0/10: carrier-grade NAT / shared address space, also used for
    // some cloud-internal networking - not covered by any `is_*` helper.
    let o = ip.octets();
    if o[0] == 100 && (64..=127).contains(&o[1]) {
        return true;
    }
    false
}

fn is_blocked_ipv6(ip: &Ipv6Addr) -> bool {
    if ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() {
        return true;
    }
    // An IPv4-mapped/compatible address must be checked against the IPv4
    // rules too, or `::ffff:169.254.169.254` sails through.
    if let Some(v4) = ip.to_ipv4_mapped() {
        if is_blocked_ipv4(&v4) {
            return true;
        }
    }
    let seg = ip.segments();
    // fc00::/7 - unique local addresses.
    if (seg[0] & 0xfe00) == 0xfc00 {
        return true;
    }
    // fe80::/10 - link-local.
    if (seg[0] & 0xffc0) == 0xfe80 {
        return true;
    }
    false
}

/// A [`reqwest::dns::Resolve`] that filters out any resolved address in a
/// blocked range, so a domain name that resolves (now or on a later,
/// rebound lookup) to an internal/metadata address is rejected at the exact
/// point reqwest would otherwise connect to it.
#[derive(Clone, Default)]
pub struct SsrfSafeResolver;

impl Resolve for SsrfSafeResolver {
    fn resolve(&self, name: Name) -> Resolving {
        Box::pin(async move {
            let host = name.as_str().to_string();
            // `lookup_host` needs a "host:port" pair; the port is discarded
            // by reqwest's connector (it overrides port from the URL/scheme
            // regardless), so any placeholder works.
            let resolved = tokio::net::lookup_host((host.as_str(), 0))
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

            let allowed: Vec<SocketAddr> = resolved
                .filter(|addr| {
                    let blocked = is_blocked_ip(&addr.ip());
                    if blocked {
                        tracing::warn!(
                            "web_fetch/http: blocking resolved address {} for host '{}' \
                             (loopback/link-local/private/metadata range)",
                            addr.ip(),
                            host
                        );
                    }
                    !blocked
                })
                .collect();

            if allowed.is_empty() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "all addresses for '{}' are in a blocked range \
                         (loopback/link-local/private/metadata)",
                        host
                    ),
                ))
                    as Box<dyn std::error::Error + Send + Sync>);
            }

            Ok(Box::new(allowed.into_iter()) as Addrs)
        })
    }
}

/// Reject a URL up front if its host is *already* an IP literal in a
/// blocked range (`http://169.254.169.254/...`) - such a URL never goes
/// through DNS resolution, so [`SsrfSafeResolver`] alone would never see it.
pub fn check_url_not_blocked(url: &str) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| format!("invalid URL: {}", e))?;
    let Some(host) = parsed.host_str() else {
        return Err("URL has no host".to_string());
    };
    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_blocked_ip(&ip) {
            return Err(format!(
                "URL host '{}' is in a blocked range (loopback/link-local/private/metadata) - \
                 fetching internal or cloud-metadata addresses is not allowed",
                host
            ));
        }
    }
    Ok(())
}

/// Install the SSRF-safe resolver on a [`reqwest::ClientBuilder`].
pub fn guard(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
    builder.dns_resolver(Arc::new(SsrfSafeResolver))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_loopback() {
        assert!(is_blocked_ip(&"127.0.0.1".parse().unwrap()));
        assert!(is_blocked_ip(&"::1".parse().unwrap()));
    }

    #[test]
    fn blocks_cloud_metadata_link_local() {
        assert!(is_blocked_ip(&"169.254.169.254".parse().unwrap()));
    }

    #[test]
    fn blocks_rfc1918_private_ranges() {
        assert!(is_blocked_ip(&"10.0.0.5".parse().unwrap()));
        assert!(is_blocked_ip(&"172.16.0.1".parse().unwrap()));
        assert!(is_blocked_ip(&"192.168.1.1".parse().unwrap()));
    }

    #[test]
    fn blocks_carrier_grade_nat_range() {
        assert!(is_blocked_ip(&"100.64.0.1".parse().unwrap()));
        assert!(!is_blocked_ip(&"100.63.255.255".parse().unwrap()));
        assert!(!is_blocked_ip(&"100.128.0.0".parse().unwrap()));
    }

    #[test]
    fn blocks_ipv6_unique_local_and_link_local() {
        assert!(is_blocked_ip(&"fc00::1".parse().unwrap()));
        assert!(is_blocked_ip(&"fe80::1".parse().unwrap()));
    }

    #[test]
    fn blocks_ipv4_mapped_blocked_address() {
        assert!(is_blocked_ip(&"::ffff:169.254.169.254".parse().unwrap()));
    }

    #[test]
    fn allows_public_addresses() {
        assert!(!is_blocked_ip(&"8.8.8.8".parse().unwrap()));
        assert!(!is_blocked_ip(&"1.1.1.1".parse().unwrap()));
        assert!(!is_blocked_ip(
            &"2606:4700:4700::1111".parse().unwrap() // Cloudflare public v6
        ));
    }

    #[test]
    fn check_url_not_blocked_rejects_ip_literal_metadata_url() {
        assert!(check_url_not_blocked("http://169.254.169.254/latest/meta-data/").is_err());
        assert!(check_url_not_blocked("http://127.0.0.1:6379/").is_err());
    }

    #[test]
    fn check_url_not_blocked_allows_normal_domain() {
        assert!(check_url_not_blocked("https://example.com/page").is_ok());
    }
}
