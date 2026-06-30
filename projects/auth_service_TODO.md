# TODO: auth_service (⭐ 6/10)

## Usage

```bash
cargo run --bin auth_service
cargo test --bin auth_service
```

## Milestones

- [ ] Add signup and login endpoints with validated request models.
- [ ] Hash passwords safely and store users in a repository abstraction.
- [ ] Issue and verify session cookies or JWT access tokens.
- [ ] Add refresh, logout, and auth middleware/guards.
- [ ] Add rate limiting and audit logging for auth-sensitive flows.
- [ ] Add tests for invalid credentials, expiry, and protected routes.

## Extra

- [ ] Add MFA challenge flow with TOTP support.

## Tips

- Start with protocol and contract tests first (request, response, error, timeout).
- Build a strict parser before adding convenience behavior; fail closed on malformed input.
- Add structured request logging early so debugging network paths is cheap.
- Keep connection lifecycle explicit: open, active, idle timeout, close.
- Add load and latency checks after correctness is stable.
