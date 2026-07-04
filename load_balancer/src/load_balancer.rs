use crate::{
    connect_to_writer::WriterClient, request::client_for_logging, response::build_checkout,
};
use async_trait::async_trait;
use bytes::Bytes;
use common::payload::CheckIn;
use pingora::prelude::{HttpPeer, ProxyHttp, Result, Session};
use uuid::Uuid;

pub struct LoadBalancer {
    pub centrale_upstream_address: String,
    pub writer: WriterClient,
}

const MAX_LOGGED_RESPONSE_BODY_BYTES: usize = 8 * 1024;

pub struct RequestCtx {
    pub x_id: String,
    pub response_body: Vec<u8>,
    pub response_body_truncated: bool,
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = RequestCtx;

    fn new_ctx(&self) -> Self::CTX {
        RequestCtx {
            x_id: Uuid::new_v4().to_string(),
            response_body: Vec::new(),
            response_body_truncated: false,
        }
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let request_bytes = session.downstream_session.to_h1_raw().to_vec();
        let ip = client_for_logging(session);
        let checkin = CheckIn::new(Some(ip), request_bytes, ctx.x_id.clone());
        self.writer.send_checkin(checkin);
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = HttpPeer::new(
            self.centrale_upstream_address.as_str(),
            false,
            "localhost".to_string(),
        );
        Ok(Box::new(peer))
    }

    fn response_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        _end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<Option<std::time::Duration>>
    where
        Self::CTX: Send + Sync,
    {
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

    async fn logging(
        &self,
        session: &mut Session,
        e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let status = session.response_written().map_or(0, |r| r.status.as_u16());

        let checkout = build_checkout(status, e, ctx);
        self.writer.send_checkout(checkout);
    }
}
