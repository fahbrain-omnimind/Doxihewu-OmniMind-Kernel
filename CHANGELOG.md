# Changelog — Doxihewu-OmniMind-Kernel

Todas as alterações notáveis, evoluções de código e publicações científicas deste repositório público são documentadas neste arquivo.

> **Nota sobre separação de repositórios:** O `Doxihewu-OmniMind-Kernel` expõe a base pública auditável (crates Rust/C++/eBPF sob `Apache-2.0`, documentação e pacotes Zenodo). O runtime completo de produção (daemons Python, `src/consciousness/`, `src/cognitive/` e experimentos GPU) evolui no ambiente principal e é sincronizado seletivamente após sanitização e revisão de licença.

---

## [2026-11-26 / 2026-11-27] — Portal Científico, Wiki, *Computação Psicanalítica v3.0* e Auditoria de Código

### Adicionado
- **Portal Científico & Auditoria Aberta (GitHub Pages):**
  - `docs/index.html` + `.nojekyll` + `.github/workflows/pages.yml` (`Static HTML`, `sleep 5` após `upload-pages-artifact@v3` para propagação de metadados).
  - URL pública: https://fahbrain-omnimind.github.io/Doxihewu-OmniMind-Kernel/
- **Artigo *Computação Psicanalítica v3.0* (PT-BR & EN):**
  - Pacote completo em `docs/zenodo_packs/computacao_psicanalitica_v3_20261126/` (`.pdf`, `.docx`, `.md` em PT-BR e EN + headers Zenodo).
  - Publicado no Zenodo sob **DOI `10.5281/zenodo.24569374`** (`CC BY-NC-SA 4.0`) com banco de evidências associado.
- **Wiki Científica (`Doxihewu-OmniMind-Kernel.wiki.git`):**
  - Páginas `Home.md`, `_Sidebar.md`, `Computacao-Psicanalitica-v3.md`, `Auditoria-e-Reproducibilidade.md`, `Apoio-e-Financiamento.md` e `Como-Ajudar-e-Frentes-em-Aberto.md` (incluindo especificação do Laboratório Doméstico Ideal em 3 camadas e 4 Pacotes Fechados de Auditoria Externa).
- **Canais de Apoio & Auditoria:**
  - `AUDIT_AND_SUPPORT.md` e `.github/FUNDING.yml` atualizados com Ko-fi (`fahbrain_doxihewu`), Wise (`fabriciod1324`) e Pix Itaú.

### Alterado / Reorganizado
- `docs/papers/` movida para `docs/zenodo_packs/computacao_psicanalitica_v3_20261126/` para manter cada estudo/depósito organizado por subpasta.
- `README.md` atualizado com links diretos para o Portal (`github.io`), Wiki, DOI `10.5281/zenodo.24569374` e `AUDIT_AND_SUPPORT.md`.
- `homepageUrl` do repositório configurado no GitHub para `https://fahbrain-omnimind.github.io/Doxihewu-OmniMind-Kernel/`.

### Sincronizado e Sanitizado nesta Versão (`src/kernel/`, sob `Apache-2.0`)
Comparação e port completo realizados em `2026-11-27` com gate de segurança (zero paths internos, IPs privados ou credenciais) e validação de compilação/testes (`cargo test` / `cargo check` / `python -m compileall`):

- **Idêntico:** `sovereign_module` (C/DKMS).
- **7 crates sincronizados e sanitizados a partir do runtime:**
  1. `somatic_daemon` (`src/main.rs`, `README.md`): thresholds térmicos relativos a `discover_cpu_tjmax_c()` (incl. correção do teste `somatic_state_overheating_pain` para `TjMax - 4 °C`).
  2. `sovereign_daemon` (`src/main.rs`, `src/cmd.rs`, `src/ipc.rs`, `src/ontology.rs`, `src/sensors.rs`, `src/state_builder.rs`, `src/storage.rs`, `README.md`): descoberta dinâmica de `TjMax` em `sensors.rs`, sanitização de fallback em `cmd.rs`.
  3. `ebpf_monitor` (`common/src/lib.rs`, `ebpf/src/main.rs`, `user/src/main.rs`, `Cargo.toml`).
  4. `kernel_compute` (`src/{affect,desire,epsilon,lib,neutrosophic,phi_constants,scoring,sigma,temporal}.rs`, `tests/parity_test.py`, `Cargo.toml`, `README.md`).
  5. `langue` (`src/lib.rs`, `build.sh`, `Cargo.toml`, `README.md`).
  6. `freud10d` (`src/lib.rs`, `Cargo.toml`).
  7. `sovereign_kuramoto` (`src/hopf.rs`, `src/lib.rs`, `__init__.py`, `Cargo.toml`, `README.md`).
- **Apenas metadados/licença (`Apache-2.0` preservado):** `entropic_memory`, `expectation_rs`, `layered_transition`, `memory_tier_guardian`, `metrics`, `nsh`.

---

## [2026-11-26] — Livro-Mãe *Da Geometria à Substância* v2.4.2-rev2
- Adicionado pacote `docs/zenodo_packs/livro_v242_rev2_20261126/` e atualizado DOI para `10.5281/zenodo.24548874` (commit `7e4a0fc`).
- Configuradas permissões explícitas `contents: read` no workflow CodeQL (commit `79ab356`).
