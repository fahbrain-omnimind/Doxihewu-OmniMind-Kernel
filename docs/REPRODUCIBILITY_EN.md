# REPRODUCIBILITY

> English. Português: [`docs/REPRODUCIBILITY.md`](REPRODUCIBILITY.md).

This document describes how to **reproduce** what this repository publishes — separating what is verifiable from what is merely asserted.

## Principle

- Everything that pulses here carries **verifiable provenance** (`source`, `path`, `source_present`). If the real source is not available, the field is `null` with `source_present=false` — never a fabricated value (see `SECURITY.md`, "Zero Rule").
- **Simulation is never presented as observed evidence** (see the vocabulary in [`docs/AGENT_CONTRIBUTION_POLICY.md`](AGENT_CONTRIBUTION_POLICY.md)).
- The public extract is reproducible from the canonical databases `/ release / Zenodo`, not from the raw internal runtime data.

## What this repository contains

- **Public pulse**: `data/pulse/current.json` (the most recent beat of the system, with `provenance.sources` pointing to canonical databases).
- **Published papers and evidence databases**: the final book pack (`PDF` + `MD` in PT and EN) lives in [`docs/zenodo_packs/livro_v242_rev2_20261126/`](zenodo_packs/livro_v242_rev2_20261126/), while the compressed unified evidence database (`unified_evidence_v3.tar.gz`, 74,587,004 bytes, SHA256 `c9ec7728b5ebc3eed399c984e59c1ffe56c54b8248464f45588c6fcbddf57c72`, containing `unified_evidence_v3.sqlite` with 62 tables / 883,164 rows, `sovereign_public_proof_registry.sqlite`, `reproduce_public_proofs.py`, `music_neural_dodecatiad_crossing_sanitized.sqlite`, and `dodecatiad_full_period_20260621_20261125_sanitized.parquet`) lives in the **GitHub/GitLab Release `v2.4.2-rev2`** and **Zenodo** (DOI `10.5281/zenodo.24548874` / Concept DOI `10.5281/zenodo.22647856`), plus the HuggingFace mirror (`fabricioslv/omnimind-dodecatiad-v3-evidence`).
- **Reproducible core**: `kernel_base/omnimind_public_base.tar.gz` + `kernel_base/colab_public_base_triad.py` (compiles and tests 5 crates on Colab, no root, no required token).

## Reproduction steps (safe track)

1. **Core** — run the public Colab:
   ```
   kernel_base/colab_public_base_triad.py
   ```
   It downloads the tarball from HF, compiles the user-space crates and runs the tests. No sudo, no system changes.

2. **Simulation** — use the simulator (local), never hardware, for routine reproduction. It requires an optional `HF_TOKEN` (public repo) — never embed a private token.

3. **Evidence** — for the evidence databases and hashes, see the matching GitHub release / Zenodo, not this repo (the git repo keeps the pointer, not the bytes).

## Hash verification

For any release, check the reported checksum against the sum of the downloaded artifact:

```bash
sha256sum <downloaded-artifact>
```

Compare with the value published in the release / Zenodo. If it diverges, **do not** use the artifact and report it (see `SECURITY.md`).

## Limitations

- Reproduction confirms that the **pipeline works and is deterministic under the described conditions** — it does not validate biomedical claims nor the presence of consciousness.
- Environments may diverge (version of `rustc`, libs). Record your environment version when reproducing for comparability.
- Items marked `[SIMULATED]` in papers are **models**, not empirical data.

## Reporting a reproduction

When you reproduce and get a result **different** from the published one, open an Issue labeled `reproducibility`, reporting: environment (OS, rustc, libs), executed commands, obtained vs expected result, and hashes.
