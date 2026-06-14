use actix_governor::{
    GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
    governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
};
use config::CentraleConfig;

pub fn public_rate_limiter_config()
-> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    // REQUESTS PER SEC
    let requests_per_second = CentraleConfig::get("PUBLIC_RATE_LIMITER_REQUESTS_PER_SECOND")
        .parse::<u64>()
        .unwrap();
    // BURST SIZE
    let burst_size = CentraleConfig::get("PUBLIC_RATE_LIMITER_BURST_SIZE")
        .parse::<u32>()
        .unwrap();
    // SETUP
    GovernorConfigBuilder::default()
        .requests_per_second(requests_per_second)
        .burst_size(burst_size)
        .key_extractor(PeerIpKeyExtractor)
        // permissive in tests, enforcing in prod
        .permissive(cfg!(test))
        .finish()
        // UNWRAP CAN ERROR, IF EITHER OF THE CONFIGS IS 0
        .unwrap()
}
