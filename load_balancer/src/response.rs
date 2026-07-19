use crate::load_balancer::RequestCtx;
use common::payload::CheckOut;

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
