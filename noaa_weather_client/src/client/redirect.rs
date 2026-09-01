//! Manual redirect following with credential and downgrade policy.
//!
//! The reqwest client is built with `redirect::Policy::none()` so that this
//! module decides which hops to follow, which headers survive a change of
//! origin, and when to stop.

use reqwest::{
    StatusCode,
    header::{ACCEPT, HeaderValue, LOCATION},
};
use url::Url;

use crate::apis::{ProtocolError, RedirectReason};

const API_KEY_HEADER: &str = "X-Api-Key";
const FEATURE_FLAGS_HEADER: &str = "Feature-Flags";

/// The most redirects one attempt will follow before failing.
pub(crate) const MAX_REDIRECTS: u8 = 5;

/// Request headers that are re-sent on every hop of one attempt.
#[derive(Clone, Copy)]
pub(crate) struct HopHeaders<'a> {
    pub(crate) accept: &'static str,
    pub(crate) feature_flags: Option<&'a HeaderValue>,
    /// Sent only while the hop stays on the origin the caller configured.
    pub(crate) api_key: Option<&'a str>,
}

/// Why a single attempt ended before a final response was available.
pub(crate) enum HopError {
    Transport(reqwest::Error),
    Redirect(Box<ProtocolError>),
}

/// Sends `GET origin` and follows redirects until a non-redirect response.
///
/// The returned response's `url()` is the final URL after redirects.
pub(crate) async fn follow(
    http: &reqwest::Client,
    origin: Url,
    headers: HopHeaders<'_>,
) -> Result<reqwest::Response, HopError> {
    let mut url = origin.clone();
    let mut hops: u8 = 0;
    loop {
        let response = hop_request(http, url.clone(), &headers, &origin)
            .send()
            .await
            .map_err(HopError::Transport)?;
        let status = response.status();
        if !is_redirect(status) {
            return Ok(response);
        }
        if hops == MAX_REDIRECTS {
            return Err(HopError::Redirect(Box::new(ProtocolError::Redirect {
                url,
                reason: RedirectReason::TooManyRedirects {
                    limit: MAX_REDIRECTS,
                },
            })));
        }
        let next = resolve_location(&url, response.headers().get(LOCATION)).map_err(|reason| {
            HopError::Redirect(Box::new(ProtocolError::Redirect { url, reason }))
        })?;
        tracing::debug!(
            status = status.as_u16(),
            to = %next,
            hop = hops + 1,
            "following redirect"
        );
        hops += 1;
        url = next;
    }
}

/// Builds one hop of a request, dropping the API key off-origin.
pub(crate) fn hop_request(
    http: &reqwest::Client,
    url: Url,
    headers: &HopHeaders<'_>,
    origin: &Url,
) -> reqwest::RequestBuilder {
    let send_api_key = same_origin(&url, origin);
    let mut request = http.get(url).header(ACCEPT, headers.accept);
    if let Some(feature_flags) = headers.feature_flags {
        request = request.header(FEATURE_FLAGS_HEADER, feature_flags.clone());
    }
    if let Some(api_key) = headers.api_key.filter(|_| send_api_key) {
        request = request.header(API_KEY_HEADER, api_key);
    }
    request
}

const fn is_redirect(status: StatusCode) -> bool {
    matches!(status.as_u16(), 301 | 302 | 303 | 307 | 308)
}

/// Resolves a `Location` header against the current URL.
///
/// Rejects missing or unparsable locations, non-HTTP targets, and any hop
/// that would downgrade an `https` request to `http`.
pub(crate) fn resolve_location(
    current: &Url,
    location: Option<&HeaderValue>,
) -> Result<Url, RedirectReason> {
    let Some(location) = location else {
        return Err(RedirectReason::MissingLocation);
    };
    let invalid = || RedirectReason::InvalidLocation {
        location: String::from_utf8_lossy(location.as_bytes()).into_owned(),
    };
    let text = location.to_str().map_err(|_| invalid())?;
    let next = current.join(text).map_err(|_| invalid())?;
    if !matches!(next.scheme(), "http" | "https") || next.cannot_be_a_base() {
        return Err(invalid());
    }
    if current.scheme() == "https" && next.scheme() != "https" {
        return Err(RedirectReason::InsecureDowngrade { target: next });
    }
    Ok(next)
}

/// Returns whether two URLs share scheme, host, and effective port.
fn same_origin(a: &Url, b: &Url) -> bool {
    a.scheme() == b.scheme()
        && a.host() == b.host()
        && a.port_or_known_default() == b.port_or_known_default()
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;
    use url::Url;

    use super::{resolve_location, same_origin};
    use crate::apis::RedirectReason;

    fn url(text: &str) -> Url {
        Url::parse(text).unwrap()
    }

    #[test]
    fn relative_and_absolute_locations_resolve_against_the_current_url() {
        let current = url("https://api.weather.gov/offices/PSR/briefing/download/latest");
        let relative = resolve_location(&current, Some(&HeaderValue::from_static("/files/a.pdf")));
        assert_eq!(
            relative.unwrap().as_str(),
            "https://api.weather.gov/files/a.pdf"
        );
        let sibling = resolve_location(&current, Some(&HeaderValue::from_static("../latest.pdf")));
        assert_eq!(
            sibling.unwrap().as_str(),
            "https://api.weather.gov/offices/PSR/briefing/latest.pdf"
        );
        let absolute = resolve_location(
            &current,
            Some(&HeaderValue::from_static("https://cdn.weather.gov/x.pdf")),
        );
        assert_eq!(absolute.unwrap().as_str(), "https://cdn.weather.gov/x.pdf");
    }

    #[test]
    fn https_to_http_downgrade_is_refused() {
        let current = url("https://api.weather.gov/document");
        let result = resolve_location(
            &current,
            Some(&HeaderValue::from_static("http://api.weather.gov/document")),
        );
        assert!(
            matches!(result, Err(RedirectReason::InsecureDowngrade { ref target }) if target.scheme() == "http"),
            "{result:?}"
        );
        // Upgrades and same-scheme hops are fine.
        let plain = url("http://127.0.0.1:8080/document");
        assert!(
            resolve_location(
                &plain,
                Some(&HeaderValue::from_static("https://example.test/"))
            )
            .is_ok()
        );
        assert!(
            resolve_location(
                &plain,
                Some(&HeaderValue::from_static("http://example.test/"))
            )
            .is_ok()
        );
    }

    #[test]
    fn missing_and_invalid_locations_are_rejected() {
        let current = url("https://api.weather.gov/document");
        assert_eq!(
            resolve_location(&current, None),
            Err(RedirectReason::MissingLocation)
        );
        assert!(matches!(
            resolve_location(&current, Some(&HeaderValue::from_static("mailto:someone"))),
            Err(RedirectReason::InvalidLocation { .. })
        ));
        assert!(matches!(
            resolve_location(&current, Some(&HeaderValue::from_bytes(b"\xff").unwrap())),
            Err(RedirectReason::InvalidLocation { .. })
        ));
    }

    #[test]
    fn origin_comparison_uses_scheme_host_and_effective_port() {
        assert!(same_origin(
            &url("https://api.weather.gov/a"),
            &url("https://api.weather.gov:443/b")
        ));
        assert!(!same_origin(
            &url("http://127.0.0.1:8001/a"),
            &url("http://127.0.0.1:8002/a")
        ));
        assert!(!same_origin(
            &url("https://api.weather.gov/a"),
            &url("http://api.weather.gov/a")
        ));
        assert!(!same_origin(
            &url("https://api.weather.gov/a"),
            &url("https://cdn.weather.gov/a")
        ));
    }
}
