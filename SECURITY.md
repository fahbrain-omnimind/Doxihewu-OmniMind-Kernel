# Security Policy

## Scope

This repository (`Doxihewu-OmniMind-Kernel`) contains the public, reproducible kernel crates, procfs/eBPF interfaces, and the `sovereign_module` Linux kernel module (`GPL-2.0`).

- Do **not** deploy `sovereign_module` or eBPF probes onto production hosts without reviewing `SECURITY_AUDIT.md` and `RISK_NOTICE.md`.
- All `/proc/omnimind/hw_triad` hardware telemetry surfaces are strictly read-only (`0444`).

## Reporting a Vulnerability

If you discover a security issue (e.g., buffer/bounds issue in C/Rust, privilege escalation in procfs/eBPF, or unsafe SQL/serialization handling):

1. **Preferred:** Open a private GitHub Security Advisory at `https://github.com/fahbrain-omnimind/Doxihewu-OmniMind-Kernel/security/advisories/new`.
2. **Alternative:** Open a GitHub Issue marked `[security]` (without posting exploitable secrets/credentials).

We aim to acknowledge reports within 72 hours and ship a verified patch + regression test.
