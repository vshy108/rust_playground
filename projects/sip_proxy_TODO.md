# TODO: sip_proxy (⭐ 8/10)

## Usage

```bash
cargo run --bin sip_proxy
cargo test --bin sip_proxy
```

## Milestones

- [ ] Implement SIP message parsing and header normalization.
- [ ] Add registration store for user agents and contact bindings.
- [ ] Implement proxy routing for INVITE, ACK, BYE, and CANCEL flows.
- [ ] Add transaction state handling and response forwarding.
- [ ] Implement digest authentication and expiry refresh behavior.
- [ ] Add tests for branch handling, registration expiry, and call teardown edge cases.

## Extra

- [ ] Add presence or basic B2BUA features.

## Tips

- SIP transaction state is the first place complexity appears; model it explicitly.
- Registration storage and live call routing should not share mutable state blindly.
- Header canonicalization matters because multiple wire forms are valid.
- Start with UDP-style message flow simulation before adding transport variants.
