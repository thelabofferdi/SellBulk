//! API HTTP Module (optional)
//! 
//! Expose les moteurs Sellify Core via une API REST

#[cfg(feature = "http-server")]
pub mod routes;

#[cfg(feature = "http-server")]
pub mod handlers;

#[cfg(feature = "http-server")]
pub mod server;

#[cfg(feature = "http-server")]
pub mod auth;

#[cfg(feature = "http-server")]
pub mod rate_limit;

#[cfg(feature = "http-server")]
pub mod scheduler;

#[cfg(feature = "http-server")]
pub mod metrics;

#[cfg(feature = "http-server")]
pub use server::create_app;

#[cfg(feature = "http-server")]
pub use scheduler::{QuotaScheduler, setup_auto_reset, SharedQuotaEngine};
