use actix_governor::{
    GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor,
    governor::{clock::QuantaInstant, middleware::NoOpMiddleware},
};
use config::CentraleConfig;

pub fn get_rate_limiter_config() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>
{
    //
    GovernorConfigBuilder::default()
        .requests_per_second(CentraleConfig::RATE_LIMITER_REQUESTS_PER_SECOND)
        .burst_size(CentraleConfig::RATE_LIMITER_BURST_SIZE)
        .permissive(false)
        .finish()
        // IT USES UNWRAP, BUT I'M NOT REALLY SURE, WHEN THIS CAN FAIL, AND WOULD LIKE TO AVOID matchING IT
        .unwrap()
}
