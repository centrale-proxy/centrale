use common::payload::CheckOut;

use crate::RequestCtx;

pub fn build_checkout(status: u16, e: Option<&pingora::Error>, ctx: &RequestCtx) -> CheckOut {
    match e {
        Some(err) => CheckOut::new(Some(status), Some(err.to_string()), ctx.x_id.clone()),
        None if status != 200 => CheckOut::new(
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
