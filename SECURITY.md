# Security Policy

## Scope

This repository (`Doxihewu-OmniMind-Kernel`) contains the public, reproducible kernel crates, procfs/eBPF interfaces, and the `sovereign_module` Linux kernel module (`GPL-2.0`).

- Do **not** deploy `sovereign_module` or eBPF probes onto production hosts without reviewing `SECURITY_AUDIT.md` and `RISK_NOTICE.md`.
- All `/proc/omnimind/hw_triad` hardware telemetry surfaces are strictly read-only (`0444`).

## Reporting a Vulnerability (Private Disclosure Only)

**DO NOT open a public GitHub Issue** for severe, critical, or sensitive security vulnerabilities — especially anything involving **API tokens, keys, credentials, secrets, privilege escalation, or remote execution**.

Please report all security issues privately via:

1. **Private Email:** `psicofabs@gmail.com` / `fabricioslv@gmail.com`
2. **GitHub Private Security Advisory:** `https://github.com/fahbrain-omnimind/Doxihewu-OmniMind-Kernel/security/advisories/new`

Include reproduction steps, affected file paths/commits, and potential impact. We commit to an initial response and triage/remediation timeline within **48 to 72 hours**.
