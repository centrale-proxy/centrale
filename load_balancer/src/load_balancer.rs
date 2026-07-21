pub mod request;
pub mod response;

use crate::connect_to_writer::WriterClient;
use async_trait::async_trait;
use bytes::Bytes;
use pingora::prelude::{HttpPeer, ProxyHttp, Result, Session};
use uuid::Uuid;

pub struct LoadBalancer {
    pub centrale_upstream_address: String,
    pub www_upstream_address: Option<String>,
    pub www_host: Option<String>,
    pub force_https_redirect: bool,
    pub writer: WriterClient,
}

pub struct RequestCtx {
    pub x_id: String,
    pub response_body: Vec<u8>,
    pub response_body_truncated: bool,
    pub is_ping: bool,
    pub is_front: bool,
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = RequestCtx;

    fn new_ctx(&self) -> Self::CTX {
        RequestCtx {
            x_id: Uuid::new_v4().to_string(),
            response_body: Vec::new(),
            response_body_truncated: false,
            is_ping: false,
            is_front: false,
        }
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        crate::load_balancer::request::request_filter(self, session, ctx).await
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        crate::load_balancer::request::upstream_peer(self, session).await
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
        crate::load_balancer::response::response_body_filter(body, ctx)
    }

    async fn logging(
        &self,
        session: &mut Session,
        e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        crate::load_balancer::response::logging(self, session, e, ctx);
    }
}
