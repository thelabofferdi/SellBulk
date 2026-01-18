# Changelog

All notable changes to Sellify Core will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- CI/CD with GitHub Actions
- Automated testing on every commit
- Automated releases with binaries for Linux x86_64 and ARM64
- Code coverage reporting with Codecov

## [0.1.0] - 2026-01-18

### Added
- **11 Deterministic Engines**: Complete core functionality
  - License Engine (HWID validation)
  - Storage Engine (AES-256-GCM encrypted SQLite)
  - Config Engine (global parameters)
  - Knowledge Base Engine (product catalog)
  - Conversation Engine (7-state machine)
  - Quota Engine (anti-ban with auto-reset)
  - Decision Engine (deterministic action selection)
  - Alert Engine (human notifications)
  - IA Gateway (constrained text generation)
  - Anti-Hallucination Engine (double validation)
  - Audit Engine (complete traceability)

- **REST API**: 13 endpoints with Axum framework
  - Health check endpoint
  - Decision making endpoint
  - Text validation endpoint
  - Quota management (check, record, status, reset)
  - Conversation state transitions
  - Knowledge base queries
  - Audit logging
  - Prometheus metrics endpoint

- **Security Features**
  - API key authentication (X-API-Key header)
  - AES-256-GCM encryption for sensitive data
  - Rate limiting (100 req/min per client)
  - HWID-based license validation

- **Automatic Quota Resets**
  - Daily reset at 00:00 UTC (messages, images)
  - Weekly reset on Monday 00:00 UTC (messages, videos)
  - Cron-based scheduler with tokio-cron-scheduler

- **Prometheus Metrics**: 16 production metrics
  - Decision tracking by action type
  - Conversation state distribution
  - Quota usage (messages, images, videos)
  - Validation results (valid/invalid)
  - AI generation duration
  - HTTP request metrics
  - Quota reset events

- **Testing**
  - 65 comprehensive unit tests
  - Test coverage for all engines
  - API integration tests
  - Metrics collection tests

- **Documentation**
  - Complete API documentation (API.md)
  - Architecture documentation (ARCHITECTURE.md)
  - Integration guide for WhatsApp (INTEGRATION_GUIDE.md)
  - Session summaries (SESSION1-5_SUMMARY.md)

- **Deployment**
  - Docker support with docker-compose
  - Multi-stage Dockerfile for optimized builds
  - Environment configuration (.env.example)

### Technical Details
- **Language**: Rust 1.77+
- **Framework**: Axum 0.7 (async HTTP)
- **Database**: SQLite with rusqlite
- **Encryption**: AES-256-GCM with authenticated encryption
- **Metrics**: Prometheus-compatible endpoint
- **Scheduler**: tokio-cron-scheduler for automatic resets

### Test Coverage
- Core engines: 33/33 tests ✅
- Quota engine: 12/12 tests ✅
- Metrics module: 6/6 tests ✅
- Scheduler: 4/4 tests ✅
- API server: 4/4 tests ✅
- Rate limiter: 4/4 tests ✅
- Auth: 2/2 tests ✅
- **Total: 65/65 tests passing** ✅

[Unreleased]: https://github.com/thelabofferdi/SellBulk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/thelabofferdi/SellBulk/releases/tag/v0.1.0
