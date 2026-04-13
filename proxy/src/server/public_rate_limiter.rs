use actix_governor::{
    GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
    governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
};
use config::CentraleConfig;

pub fn public_rate_limiter_config()
-> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    GovernorConfigBuilder::default()
        .requests_per_second(CentraleConfig::PUBLIC_RATE_LIMITER_REQUESTS_PER_SECOND)
        .burst_size(CentraleConfig::PUBLIC_RATE_LIMITER_BURST_SIZE)
        .key_extractor(PeerIpKeyExtractor)
        // permissive in tests, enforcing in prod
        .permissive(cfg!(test))
        .finish()
        // UNWRAP CAN ERROR, IF EITHER OF THE CONFIGS IS 0
        .unwrap()
}
