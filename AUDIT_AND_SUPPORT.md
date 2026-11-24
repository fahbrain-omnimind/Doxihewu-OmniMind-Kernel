# How to Audit in 5 Minutes & Support Independent Research

## 1. Quick CPU-Only Verification (No GPU / No Cloud Credits Required)

You can audit and test the standalone Rust crates on any standard Linux `x86_64` host or CI runner:

```bash
# Test standalone Rust kernel crates
for crate in metrics entropic_memory expectation_rs sovereign_kuramoto somatic_daemon memory_tier_guardian; do
  cargo test --manifest-path "src/kernel/${crate}/Cargo.toml"
done
cargo check --manifest-path "src/kernel/sovereign_daemon/Cargo.toml"
```

To inspect the Linux kernel ring-0 triadic sensor (`hw_triad` `{-1, 0/MU, +1}`):
- Read [`src/kernel/sovereign_module/sovereign_module.c`](src/kernel/sovereign_module/sovereign_module.c) (`read_best_cpu_thermal_mc`, `hw_triad_show`) and [`src/kernel/sovereign_module/README.md`](src/kernel/sovereign_module/README.md).

## 2. How to Critique or Contribute

- **Technical critique & code review:** Open an Issue or Pull Request on [`fahbrain-omnimind/Doxihewu-OmniMind-Kernel`](https://github.com/fahbrain-omnimind/Doxihewu-OmniMind-Kernel/issues). Concrete bug reports, kernel/Rust safety reviews, and calibration feedback on non-Lenovo hardware are explicitly welcome.
- **License clarity:** Kernel crates and tooling in this repository are licensed under **Apache-2.0** ([`LICENSE`](LICENSE)); the Linux kernel module and eBPF probes are **GPL-2.0**.

## 3. Infrastructure & Hardware Support

This project is developed independently on a single **Lenovo IdeaPad Gaming 3 15IMH05 (i5-10300H, 24 GB RAM, GTX 1650)** without institutional grants. Contributions directly sustain:
- Local hardware maintenance / RAM & storage expansion for kernel & non-binary experiments;
- Public dataset mirrors and reproducible CI benchmarks.

See `.github/FUNDING.yml` or the repository Sponsor button on GitHub.
