use crate::load_balancer::{LoadBalancer, RequestCtx};
use bytes::Bytes;
use common::payload::CheckOut;
use pingora::prelude::{Result, Session};

const MAX_LOGGED_RESPONSE_BODY_BYTES: usize = 8 * 1024;

pub fn response_body_filter(
    body: &mut Option<Bytes>,
    ctx: &mut RequestCtx,
) -> Result<Option<std::time::Duration>> {
    if let Some(chunk) = body.as_ref() {
        let remaining = MAX_LOGGED_RESPONSE_BODY_BYTES.saturating_sub(ctx.response_body.len());

        if remaining > 0 {
            let bytes_to_copy = remaining.min(chunk.len());
            ctx.response_body.extend_from_slice(&chunk[..bytes_to_copy]);
        }

        if chunk.len() > remaining {
            ctx.response_body_truncated = true;
        }
    }

    Ok(None)
}

pub fn logging(
    load_balancer: &LoadBalancer,
    session: &mut Session,
    e: Option<&pingora::Error>,
    ctx: &mut RequestCtx,
) {
    // NO CHECKOUT FOR PING AND FRONT
    if !ctx.is_ping && !ctx.is_front {
        // CHECKOUT
        let status = session.response_written().map_or(0, |r| r.status.as_u16());
        let checkout = build_checkout(status, e, ctx);
        load_balancer.writer.send_checkout(checkout);
    }
}

pub fn build_checkout(status: u16, e: Option<&pingora::Error>, ctx: &RequestCtx) -> CheckOut {
    match e {
        Some(err) => CheckOut::new(Some(status), Some(err.to_string()), ctx.x_id.clone()),
        None if status >= 400 => CheckOut::new(
            Some(status),
            response_body_for_logging(ctx).or_else(|| Some("err".to_string())),
            ctx.x_id.clone(),
        ),
        None => CheckOut::new(Some(status), None, ctx.x_id.clone()),
    }
}

fn response_body_for_logging(ctx: &RequestCtx) -> Option<String> {
    if ctx.response_body.is_empty() {
        return None;
    }

    let mut body = String::from_utf8_lossy(&ctx.response_body)
        .trim()
        .to_string();
    if body.is_empty() {
        return None;
    }

    if ctx.response_body_truncated {
        body.push_str(" …[truncated]");
    }

    Some(body)
}

#[cfg(test)]
mod tests {
    use super::{build_checkout, response_body_for_logging};
    use crate::load_balancer::RequestCtx;

    fn ctx(body: &[u8], truncated: bool) -> RequestCtx {
        RequestCtx {
            x_id: "x-test".to_string(),
            response_body: body.to_vec(),
            response_body_truncated: truncated,
            is_ping: false,
            is_front: false,
        }
    }

    #[test]
    fn build_checkout_does_not_mark_redirect_as_error() {
        let checkout = build_checkout(308, None, &ctx(b"", false));
        assert_eq!(checkout.status, Some(308));
        assert_eq!(checkout.error, None);
    }

    #[test]
    fn build_checkout_uses_body_for_client_and_server_errors() {
        let checkout = build_checkout(404, None, &ctx(b"not found", false));
        assert_eq!(checkout.status, Some(404));
        assert_eq!(checkout.error, Some("not found".to_string()));
    }

    #[test]
    fn build_checkout_uses_generic_err_when_error_body_is_missing() {
        let checkout = build_checkout(500, None, &ctx(b"", false));
        assert_eq!(checkout.status, Some(500));
        assert_eq!(checkout.error, Some("err".to_string()));
    }

    #[test]
    fn response_body_for_logging_appends_truncated_marker() {
        let body = response_body_for_logging(&ctx(b"test body", true));
        assert_eq!(body, Some("test body …[truncated]".to_string()));
    }
}
