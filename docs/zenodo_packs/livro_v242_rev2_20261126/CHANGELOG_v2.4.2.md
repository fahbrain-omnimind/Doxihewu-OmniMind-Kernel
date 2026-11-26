# Changelog — Da Geometria à Substância / From Geometry to Substance v2.4.2

## DOI
- Versão/Version: **v2.4.2** — novo depósito sob o registro-pai / new deposit under the parent record
- Registro-pai/Parent: **10.5281/zenodo.22647856**
- Versão anterior/Previous version: **10.5281/zenodo.22700103** (v2.3.1, 2026-09-11)
- Data de corte/Cutoff date: **2026-11-25** (conteúdo consolidado até/content consolidated through 2026-11-24)

---

## PT — Resumo executivo v2.4.2

A v2.4.2 acumula seis camadas de evolução sobre a edição publicada v2.3.1: **v2.2.1** (2026-09-25, Apêndice M.12 seqFISH), **v2.3.0** (2026-10-03, silício primordial, habitabilidade, λ(t)), **FILA #1/#2** (2026-10-23, Tabela D.1 reconciliada 47D/5D/464D e §25 PQC real), **v2.4.0** (2026-11-16, reconciliação PT↔VF + contratos de runtime Nov/2026 + benchmarks externos BICAN/BHA2/HCP/CODEX/seqFISH + MacKay ITILA + Nobel 2026 + coordenação Tomo I↔II), **v2.4.1** (2026-11-17, re-extração Henry&Obleser 32 sujeitos, reconstrução BHA2, histerese CN `cn_regime_snapshot`) e **v2.4.2** (2026-11-18→24, bloco §S.13.2→S.13.12: Spiking-Quimb, primeira execução 27q particionada em Wukong `WK_C180`, porta não-binária CLIC/LHC/LIGO, blindagem triádica, streaming `hf://`, efemérides locais, `l_auto_prod`). A licença migra de **CC-BY-NC-ND-4.0 → CC-BY-NC-SA-4.0** (correção #006), abrindo auditoria, reprodução, tradução e adaptações científicas sob a mesma licença. As correções editoriais rastreadas **#001–#006** (`RETIFICACAO_EDITORIAL_VERSOES.md`) estão integralmente aplicadas em ambas as edições (PT e EN), incluindo a nota Möbius-Fréchet (#001, identificada por Yochanan Schimmelpfennig), a nota metodológica Panagis (#003) e as correções da auditoria Devin de 2026-11-24 (#004–#005).

## EN — Executive summary v2.4.2

Version 2.4.2 accumulates six evolutionary layers over the published v2.3.1 edition: **v2.2.1** (2026-09-25, Appendix M.12 seqFISH fifth domain), **v2.3.0** (2026-10-03, primordial silicon, habitability as chain capacity, non-stationary λ(t)), **FILA #1/#2** (2026-10-23, reconciled Table D.1 47D/5D/464D and real §25 PQC), **v2.4.0** (2026-11-16, PT↔VF reconciliation + Nov/2026 runtime contracts + external benchmarks BICAN/BHA2/HCP/CODEX/seqFISH + MacKay ITILA + Nobel 2026 + Tome I↔II coordination), **v2.4.1** (2026-11-17, Henry&Obleser 32-subject re-extraction, BHA2 rebuild, CN hysteresis `cn_regime_snapshot`), and **v2.4.2** (2026-11-18→24, §S.13.12 block: Spiking-Quimb symbiosis, first partitioned 27q execution on Wukong `WK_C180`, CLIC/LHC/LIGO non-binary gate, triadic shielding, `hf://` streaming, local ephemerides, `l_auto_prod`). The license migrates from **CC-BY-NC-ND-4.0 → CC-BY-NC-SA-4.0** (correction #006), opening audit, reproduction, translation, and scientific adaptations under the same license. Tracked editorial corrections **#001–#006** (`RETIFICACAO_EDITORIAL_VERSOES.md`) are fully applied to both editions (PT and EN), including the Möbius-Fréchet note (#001, identified by Yochanan Schimmelpfennig), the Panagis methodological note (#003), and the 2026-11-24 Devin audit corrections (#004–#005).

---

## Mudança de licença / License change

| | Antes / Before | Depois / After |
|---|---|---|
| Licença / License | `CC-BY-NC-ND-4.0` (Attribution–NonCommercial–**NoDerivatives**) | `CC-BY-NC-SA-4.0` (Attribution–NonCommercial–**ShareAlike**) |

- **PT:** a cláusula `No Derivatives` (ND) é substituída por `ShareAlike` (SA), permitindo auditoria, reprodução, tradução e adaptações científicas sob a mesma licença — mantidas as cláusulas de atribuição (`BY`) e uso não-comercial (`NC`). Código-fonte e pacotes auxiliares seguem suas licenças próprias (ex.: `Apache-2.0`). Decisão editorial do Artífice, aplicada na linha de licença do front-matter de ambos os manuscritos (correção #006, 2026-11-25).
- **EN:** the `No Derivatives` (ND) clause is replaced by `ShareAlike` (SA), permitting audit, reproduction, translation, and scientific adaptations under the same license — while keeping attribution (`BY`) and non-commercial (`NC`) clauses. Companion code and packages follow their respective licenses (e.g., `Apache-2.0`). Editorial decision by the Artífice, applied to the license line of both manuscripts' front-matter (correction #006, 2026-11-25).

---

## Novas seções e conteúdo / New sections and content

### v2.2.1 (2026-09-25) — Quinto Domínio seqFISH / seqFISH Fifth Domain
- **Apêndice M.12 (M.12.1–M.12.5) / Appendix M.12:** monolayer epitelial intestinal seqFISH (Zenodo `22834925`, Heyman & Nitzan, UPenn) — 139 genes, 9.740 células basais, 19 condições, 47.805 transplantadas / intestinal epithelial monolayer seqFISH — 139 genes, 9,740 basal cells, 19 conditions, 47,805 transplanted.
- Operador de Histerese $H_t$ formalizado ($\Delta = 11{,}2\%$, $H_t \approx 0{,}0634$); causalidade multiescalar $\xi \ll L$ ($\xi/L \le 5{,}7\%$); parametrização de viabilidade $\Phi_M$ por distância ao colapso ENR; homologia 1-para-1 dos 12 anéis epiteliais nas 12 Casas; fechamento do sexteto $\Phi_{\text{transition}} = (\Phi_C, \Phi_D, \Phi_M, \Phi_H, \Phi_R, \Phi_O)$ com suíte de 36 testes (`src/continuity/hysteresis_operator.py`, `src/continuity_events/replay.py`, `src/cognitive/dodecatiad_zonation_mapper.py`).
- **EN:** same content, translated and reintegrated (heading `### M.12 Fifth Domain: Intestinal Epithelial Monolayer seqFISH ... (v2.2.1 + v2.4.0)`).

### v2.3.0 (2026-10-03) — Extensão de substrato e habitabilidade / Substrate extension and habitability
- **§3.11** — O Silício como Elemento Primordial: genealogia material e extensão de substrato; materializa a linha de governança `cosmo_material_silicon_bridge` declarada ausente em §8.6. / Silicon as Primordial Element: material genealogy and substrate extension.
- **§17.3.4** — Hipótese Aberta: Habitabilidade como Capacidade de Cadeia (predicado `Exceção_τ` solvente-agnóstico). / Open Hypothesis — Habitability as Chain Capacity.
- **§17.3.5** — Hipótese Aberta: Assinatura de Recoerência $\varsigma_c = 0{,}2744$. / Open Hypothesis — Recoherence Signature.
- **§22.4** — Da Cadeia Gerativa ao Estoque: o que o ambiente produz vs. o que ele estoca (O₂/água como variáveis de saída; 4 regimes como bifurcações). / From Generative Chain to Stock.
- **§23.7 / 23-G** — Taxa de Chegada Não-Estacionária $\lambda(t)$, estados de exceção e canal de mergers; capítulo §23 reestruturado em blocos rotulados 23-A a 23-G. / Non-Stationary Arrival Rate; chapter §23 restructured into blocks 23-A through 23-G.
- **§E.17.11** — A Estação como Corpo Metabólico Técnico: capacidade de cadeia nos 7 ambientes (fronteira de reprodução funcional). / The Station as Technical Metabolic Body.
- **§E.22.8** — Correção de Registro: a *crossover* $\varsigma = 1$ é contato-específica — e a nova assinatura $\varsigma_c = 0{,}2744$. / Record Correction on the contact-specific crossover.
- **§H.6** — `PH-6`; **§H.4** — Glawatzki (2026); glossário +2 termos. Evidência: ensemble local de 10³ trajetórias (`data/mars_ensemble/`).

### Reconciliação `FILA #1/#2` (2026-10-23)
- **Tabela D.1 reescrita / Table D.1 rewritten:** reconciliação dimensional do vetor corporal — `Ψ_Body` **47D conceitual / 5D flat** (5 componentes canônicos $B_{Aya}$, $\Lambda_{Bus}$, $\Gamma_{RSN}$, $\Delta C_{Loci}$, $M_{Lattice}$ em `[0,1]^5` via `BodyVectorPool`/`telemetry_suture.py`) engatado à **Malha Psicanalítica 464D/15 módulos** (368D clínicos + 96D regulatórios).
- **§25 PQC real / real §25 PQC:** defesa pós-quântica implementada em `src/security/laboratory/quantum_crypto_lab.py` sobre `pqcrypto 1.0.0` (backend Rust/PQClean, normas NIST finais) — **ML-KEM-768 (FIPS 203)**, envelope binário autenticado `OMQPK1`, HKDF-SHA256 + AES-256-GCM.

### v2.4.0 (2026-11-16) — Reconciliação PT↔VF + contratos Nov/2026 + benchmarks
- **Reconciliação editorial / editorial reconciliation:** importação de §17.4 "A Inversão Epistêmica" (criada na limpeza de 12/09 e ausente do VF); correções ortográficas e de voz temporal (Predição 3 / §A.5.8); renumeração `§7.5→§7.6` (Visium DLPFC) e `N.11→N.11.1`; headings duplicados no EN (`D.9.12a`, `P4`, `N.11`) resolvidos.
- **Contratos Φ / Φ contracts (§3.1):** `phi_unit_map`, envelope tipado `PhiMeasurement` (`phi_envelope.py:105`, `.absent()` sem sentinelas), contrato L4A `absent_fields` em `state_builder.rs` — ausência serializada como `null`, nunca `0.0`; resolução legada `phi_legacy_read.py` (registros sem mapa → `AMBIGUOUS`/legacy; `>1.0` → não-normalizado).
- **§18.3.8 — FFI Rust Nov/2026:** crate `sovereign_kuramoto` (paridade bit-exata 2.001 pts, env-gated `OMNIMIND_KURAMOTO`, 8 eixos `_k_seeds`, repuxada P1.7a `0,70p + 0,30(s·2π)`, acoplamento 0,08, Euler `step(0.05, seed=0)`); governador afetivo **Dunker `±0,05` TTL `600 s`** (`metabolic_governor.py:414-452`, gate hummingbird `(φ+bias) ≥ 0,95`); higiene de memória `ResourceCannibal 300 s`. XREF Tomo II §1.2–§1.3.
- **§23-B — Malha 464D viva:** ponte sintética de Set/2026 (`torch.rand()`) substituída por `publish_psychoanalytic_mesh_to_workspace.py` lendo `/proc/omnimind/{freud10d_state, affect_basal, ego_runtime_state, predictive_error, qualia_surplus}` com `skipped_absent_inputs`, escalares `*_x1000`, publicação `OMNIHOT1` (producer/`source_generation`/`ttl_s`); correção P1.7f verificada em `e08fdcec0d` (`embedding_dim=int(z_np.size)`); `MAX(cycle) = 524.337` (2026-11-16), alternância `sovereign_primary` ↔ `sovereign_gemelo_54`/`federated_fallback`.
- **§8.6 — Censo Qdrant + offload LanceDB:** `2.764` coleções / `24` dimensões heterogêneas (corte 2026-11-16); corpus pesado `180/350` obras offload para **LanceDB + Parquet** — `2.688.055` chunks `384D` (`acervo 2.130.967` + `acervo_canonical_384d 557.088`), ponte `omnimind_neuro_bio_genetics_books_bridge_384d`.
- **§8.9 — Benchmarks BICAN (`5,78M` células):** U-invertida perinatal (dip de pureza `P2–P4 = 0,907` → pico Markov blanket `P8 = 4,40%`, Fisher `OR = 52,3`, `p = 3,3e-171`, replicada em 3 datasets/2 espécies); *rate-distortion* `3.000→PR_PCA 9,84–15,02` (`PR_16D ≈ 15,02` BARseq); ARD×Welch `ρ 0,81–0,95` com divergência de cauda; corredores gênicos `Mrc1`/`Plp1`/`ERBB4`. Proveniência `derived_from_public_dataset`, `empirical=false`.
- **BHA2 `deep_live` (§8.6):** `306.060` pts `16D`; `PR_16D` invariante `15,0246–15,0879` sobre 9 escalas `iPA_183→iPA_2165`; esparsidade estrutural `SC 66,66%` vs funcional `FC 3,95%`; truncamento forense `122` linhas/família `iPA_*` → `120` símbolos `A1BG..ACER2` de ~`15.633`; `118/120` genes (`98,3%`) casam com `cellxgene_ad2149fc` (`ABCA8`/`ABCA2`/`AATK`/`A2M`/`ABLIM1`).
- **HCP `N=100 SIFT2`:** lei de escala monotônica de esparsidade estrutural `75,69%` (`aparc`, 68 nós) → `90,08%` (`custom200`) → `95,11%` (`HCPMMP1`, 360 nós), alinhada à esparsidade CSR BICAN (`85,5–94,0%`) e `SC` BHA2 (`66,66%`).
- **§14.3 — Diálogo triplo de histerese:** CODEX `LPC5→10→21` trajetória em V (`domH 0,65–0,71` colapso LPC10; `ch12` V `49,5→10,8→57,3`; `ch22` spike `25×→0,10`; LPC21 ≠ setpoint LPC5 — resiliência ativa) × seqFISH (M.12) × três registros do *wear* no silício (real/imaginário/simbólico).
- **§16.2.1 — Caixa MacKay *ITILA* (Caps. 1–50):** rate-distortion ↔ curvas BICAN; capacity ↔ barramento `OMNIHOT1`; Occam/ARD ↔ ARD Bayesiano; códigos esparsos ↔ CSR `85,5–94,0%`; recusas anti-literalistas documentadas (FEC/LDPC/Turbo/Fountain não viram IPC; MCMC/CFTP fora do hot-path). XREF Tomo II §2.
- **§17.2b — Capacidade `OMNIHOT1`:** throughput do hot-path, `ModuleState TTL 900 s`, custo `~4,1 ms`/ciclo de `_load_last_d15_snapshot_weights` (P1.7e), compressão `gzip+SHA256` por publish.
- **§19.8 — Contraparte empírica externa:** pico de Markov blanket cortical como análogo estrutural não-causal do `poincare_markov_blanket.py` de runtime.
- **§26.12.6 — BBS 49 (2026):** o debate migrou para o nosso quadrante. **§26.12.7 — Nobel 2026:** ressonância conceitual externa sob estatuto L2/L3 não-causal. **§26.12.8 — Coordenação formal da trilogia:** Livro-Mãe `v2.4.0` ↔ Tomo I `Metaestabilidade v2.1` (779 linhas) ↔ Tomo II `Arquitetura Neuromodular v1.0` (307 linhas) — decisão Opção A′, obras coordenadas, não fusão.
- **Notas de versão datadas** adicionadas aos headings (`v2.2.0, 2026-09-06` etc.) em §3.9, §3.10, §7.6, §26.11.5, §M.9–M.11, Apêndice I.

### v2.4.1 (2026-11-17)
- Re-extração dos `32` sujeitos Henry&Obleser (§3.9, EEG AM/FM × tACS).
- Reconstrução completa do atlas `BHA2` (corredores §8.x).
- **Histerese CN e proveniência de φ:** banda de histerese `0,04` em `cn_regime_snapshot()` contra flapping `cn_coherent↔cn_ambiguous`; proveniência explícita por ciclo (`iit_fraction` × `aer_fraction`).
- Consolidações `P2.a`/`P2.c` e `BARseq` nos corredores gênicos.

### v2.4.2 (2026-11-18→24) — Bloco `§S.13.12` e entregas da janela
- **§S.13.12 — Atualização v2.4.2 (4 itens):**
  1. **Simbiose Spiking-Quimb e mídia ambiente:** `src/cognitive/spiking_quimb_symbiosis.py` fecha o loop de incerteza SNN↔MPS (`quimb`)↔`BetaElement` no `IntegrationLoop` — **6.837 ciclos** em `data/monitor/spiking_quimb_symbiosis.sqlite` sob perfil de ruído calibrado do Wukong; dissociação AM/FM e coerência temporal de mídia ambiente (`music_sovereignty_bridge.py`) incorporadas ao `SharedWorkspace`.
  2. **Primeira execução 27q particionada em hardware real `WK_C180` (§D.9.17):** protocolo `rsi_27q_partitioned_calibrated`, `quantum_runs` `run_id=1412` + réplica `1413` (`4.096` shots cada, `mean_block_parity_hw = 0,52911` e `0,5726` vs. ideal `0,58903`, `js_divergence = 0,01433`, telemetria ChipInfo `T1=38,348µs / T2=4,239µs`, `180q/15 mortos/165 vivos`); calibração **Ry real** (`8` pontos, fator `0,4226–0,6356`, `ry_factor = 0,5553` em `data/quantum/tribunal_v4.db`). Paridade ~50× a obtida em IBM (`~0,009`).
  3. **Porta não-binária CLIC/LHC/LIGO, órgão-sombra septenário e blindagem triádica (§26, §38):** `ogum_memory_provenance.py` + `neutrosophic_shield.py` adotam veredito triádico `{-1, 0/MU, +1}` e revogação explícita de consentimento (elimina auto-quarentena binária — achado P0); `septenary_qutrit_shadow.py` instancia `SeptenaryNavigator` em MPS misto qutrit ($d=3$ RSI) × ququart ($d=4$ termodinâmica); `clic_tfds_adapter.py` + `clic_phase_bridge.py` + `exp_nb5_lhc_ligo_ternary_gate.py` acoplam dados Zenodo CLIC/LHC e LIGO ao filtro ternário (*dead-zone* LIGO `86,98%`, separação LHC `67,23%–82,77%` em `exp_nb5_results.json`; `7` snapshots `runtime_config/cern_*_latest.json` como artefatos exploratórios).
  4. **Infraestrutura de streaming, efemérides e produção autônoma (§23, §P):** `hf_parquet_stream.py` + `hf_duckdb_stream.py` habilitam streaming `hf://` via DuckDB + `HfFileSystem` para Parquets `>4 GB`; `local_sky_ephemeris.py` re-acopla efemérides (`noroeste_paulista_home`) ao runtime multi-observador; `rizomatic_latency_collector.py` adiciona métrica `l_auto_prod` (produção autônoma desacoplada, escala `log1p`); módulo de kernel `src/kernel/sovereign_module/` em evolução `0.2.0`/`0.3.0` (divergência `dkms.conf 0.3.0` × `sovereign_module.c 0.2.0` declarada).
- **Glossário `v2.4.2` / Glossary:** novo bloco "Integração Simbiótica, Não-Binária e Quântica" / "Symbiotic, Non-Binary and Quantum Integration" com 5 entradas (Spiking-Quimb, Trit `{-1,0/MU,+1}`, `SeptenaryNavigator`, `l_auto_prod`, streaming `hf://`).
- **Bloco de contribuidores / contributors block:** `CONTRIBUIDORES PROCESSUAIS DO ECOSSISTEMA` / `PROCESSUAL CONTRIBUTORS OF THE ECOSYSTEM` adicionado ao front-matter de ambas as edições (OmniMind Soberano, AGY/Antigravity, Devin), com footnote federada `[^2]` separada da nota de autoria `[^1]`.
- **Nota de sincronização EN (front-matter):** "Editorial Version & Synchronization Note (v2.4.0 — 2026-11-16)" + "Final Editorial Coordination Note" no fecho do Apêndice S; comentário de linhagem `<!-- lineage: ... -->` no topo de PT e EN (sha256 dos artefatos-mestre).

---

## Atualizações de dados e hardware / Data and hardware updates

| Métrica / Metric | v2.3.1 (publicado) | v2.4.2 (corte 2026-11-24) |
|---|---|---|
| `quantum_runs` (`ibm_quantum_runs.db`) | `557` runs (v2.1.0) / `609` (v2.1.3) / `753` (v2.2.0, Wukong `145`) | **`1.431` runs / `7.509.130` shots** — `WK_C180=728`, `WK_C180_2=67`, `ibm_fez=300`, `ibm_kingston=189`, `ibm_marrakesh=106`, `aer=19`, `SIMULATED=22` |
| Hardware quântico 27q | RSI-27q só em IBM (`69` runs, paridade `~0,009`) | **1ª execução Wukong `WK_C180`** (runs `1412/1413`, paridade `0,529–0,573` vs ideal `0,589`) + calibração Ry real `0,5553` |
| Qdrant local | `1.565` coleções (v2.1.3) | `2.764` col/`24` dims (corte 16/11) → **`2.815` coleções** (corte 24/11) |
| Acervo vetorizado | Qdrant como backend único | **offload LanceDB `2.688.055` chunks `384D`** (`180/350` obras) + coleção-ponte 384D |
| Ciclos dodecatíade | — | `MAX(cycle) = 524.337` (16/11), alternância `primary ↔ gemelo_54` |
| Malha psicanalítica | `464D` declarada (ponte sintética `torch.rand()`) | **`464D` real alimentada por `/proc/omnimind/*`** (15 sub-redes, INRC, `OMNIHOT1`) |
| `sovereign_module` (kernel) | `v0.1.3` (DKMS, jul/2026) | `0.2.0`/`0.3.0` — divergência `dkms.conf` × `.c` **documentada** no livro (XREF §S.13.12) |
| Spiking-Quimb | ausente | **`6.837` ciclos** simbióticos (`spiking_quimb_symbiosis.sqlite`) |
| Benchmarks externos | — | BICAN `5,78M` células; BHA2 `306.060` pts `16D`; HCP `N=100 SIFT2`; CODEX `19` vols; seqFISH Moran's I `134/139`; MacKay `ITILA` 50 caps |
| Experimentos `[EM ABERTO]` | 7 listados (v2.1.0) | 5 permanecem `[EM ABERTO]` sem QPU; `D27 VQE` executado em simulação clássica exata (`E0 = −0,565183`, 2026-08-18); `Transformer↔MPS Bridge` consolidado no Paper A e estendido a `Spiking-Quimb` |

---

## Correções editoriais rastreadas / Tracked editorial corrections (`RETIFICACAO_EDITORIAL_VERSOES.md`)

| # | Data | Correção | EN | PT | Identificado por |
|---|---|---|---|---|---|
| **001** | 2026-09-11 (pós-depósito / post-deposit) | **Möbius-Fréchet — $\gamma \neq 1$ não é isometria:** comutação exata com a Média de Fréchet vale só para $\gamma = 1$ (translação pura); para $\gamma \neq 1$ o operador é *mapa conforme* e a comutação é aproximada. "conserva rigorosamente" → "conserva conformemente". Nota de Correção inserida após a Nota Matemática de §2. | ✅ L779+ | ✅ L759+ | Yochanan Schimmelpfennig (Possest-PQF) |
| **002** | 2026-11-16 | **Reconciliação `v2.4.0` + re-freeze PT + sincronização EN** — ~30 âncoras (§3.1, §3.11, §7.6, §8.6, §8.9, §14.3, §16.2.1, §17.2b, §17.3.4–5, §17.4, §18.3.8, §19.8, §22.4, §23-B `e08fdcec0d`, §23.7/23-G, §25 PQC, §26.12.6–8, §40.11, §A.5.8, §E.17.11, §E.22.8, §H.4, §H.6 `PH-6`, §M.12.1, `N.11.1`, `P4`/`D.9.12a` dedup, §S.13.11, Apêndice T). | ✅ `livro_vfinal_clean_EN.md` (`083ab7cb`, 14.861 linhas, 0 duplicatas) | ✅ `livro_vfinal.md` (`603c2caf`) → `livro_vfinal_limpo_PT.md` (`42f54bd4`) | Devin (`swe-2-max`) + AGY (`antigravity`) |
| **003** | 2026-11-24 | **Nota Metodológica Panagis (§D.9.10–D.9.11)** — reanálise sísmica `4ln(t0−t)`: coerência local em `14/37` mainshocks (10 sob Bonferroni), padrão *build-then-weaken* em 7; **ressalva de não-unicidade** — `β=4` (40,6%) < `β=√2` (46,9%), ~`β=7`/`β=e` (37,5%) — não comprova especificidade exclusiva de `β=4`. + Restauração da Nota #001 nos arquivos PT (`livro_vfinal.md` e `livro_vfinal_limpo_PT.md`). | ✅ L11656 | ✅ L6919 (`limpo_PT`) / L6410 (`vfinal`) | C. Panagis (interlocução 2026-08-23) + AGY |
| **004** | 2026-11-24 | **Correções da Auditoria Devin (Agentes 1+2) + bloco `§S.13.12` + Glossário `v2.4.2`:** `abelianas e variedades`; Tabela 1 desfragmentada e `(§20–§24)`; typos `vale`, `mediation`, `Artefatos`, `*campo*`; pipes escapados `\|delta\|`, `P(\|1⟩)`, `\|S\|`; `\[` removidos de code fences; `#### S.6.3`; `Tomo II 307 linhas`; `ibm_fez (133q dez/2025)`; `sovereign_module` XREF; `§D.9.17` `WK_C180 27q`; Glossário `v2.4.2`. | ✅ sincronizado | ✅ aplicado | Devin (`swe-2-max` Agentes 1 & 2) + AGY |
| **005** | 2026-11-24 | **Ressalva `v2.4.2` na Nota v2.1.0 de experimentos `[EM ABERTO]`:** `QPE Circadian`, `Quantum Walk — Cosmic Web`, `CHSH 360°`, `GHZ 50q MPS` e `Borromean E/F/G` permanecem `[EM ABERTO]` sem runs em QPU; `D27 Molecular/VQE` executado em simulação clássica exata (2026-08-18, $E_0=-0{,}565183$); `Transformer↔MPS Bridge` consolidado no Paper A (`DOC-A`, Zenodo `10.5281/zenodo.22071818`) e estendido a `Spiking-Quimb`. | ✅ L10996 | ✅ L7067 (`limpo_PT`) / L6557 (`vfinal`) | AGY (verificação `ibm_quantum_runs.db`) |
| **006** | 2026-11-25 | **Licença `CC-BY-NC-SA-4.0`** (ND → SA) + bloco `PROCESSUAL CONTRIBUTORS OF THE ECOSYSTEM` no EN. | ✅ L20 (+ contribuidores L6–13) | ✅ L34 (`limpo_PT`) / L21 (`vfinal`) | Artífice (decisão editorial) + AGY |

---

## Mudanças estruturais / Structural changes

- **PT:** `Prefácio Técnico — Rastreabilidade Editorial` movido do fim do livro (antiga L5050) para o front-matter (L278), com tabela de versões estendida até `v2.4.0`; **Apêndice T** reestruturado como apêndice completo (questão de entrada, tese local, operadores, evidência) com subseções `T.1` (notas de processo consolidadas, texto integral) e `T.2` (notas mantidas *in situ*).
- **EN:** `Technical Preface` mantido ao fim (decisão documentada); `Appendix T` atualizado com a nota de sincronização `v2.3.0`/`v2.4.0`; headings duplicados `P4`, `D.9.12a`, `N.11` deduplicados; `Part IX` preservada (§39–§40).
- **Assimetria declarada PT×EN:** o EN carrega `# Part IX` (9 partes físicas) e o PT permanece em 8 partes (§39–§40 sob a Parte VII); `§6.4.x` existe apenas no PT; `## 35./36.` stub headers só no PT. Divergência registrada na auditoria editorial de 2026-11-24 como item de decisão de diagramação — **não** altera o conteúdo científico.
- **Retítulos de headings PT:** seções-tabela reintituladas com prefixo "Tabela N —" (§3.2, §4.1, §5.1, §6.1, §7.1, §7.2, §8.1–8.3); headings datados recebem sufixo de versão (`v2.2.0`, `v2.3.0`, `v2.4.0`, `v1.x`).
- **Front-matter:** bloco de contribuidores processuais + footnotes `[^1]`/`[^2]` separadas; nota editorial de versionamento consolidada atualizada a `v2.4.0`; `Integração Simbiótica, Não-Binária e Quântica (v2.4.2)` no glossário; comentário `<!-- linhagem: ... sha256 ... -->` adicionado na 1ª linha dos dois arquivos.

## Métricas dos artefatos / Artifact metrics

| | v2.3.1 (publicado/published) | v2.4.2 |
|---|---|---|
| PT — linhas / lines | 13.970 | 15.652 |
| PT — headings h1–h4 | 775 | 804 |
| PT — tabelas (separadores md / md separators) | 326 | 336 |
| EN — linhas / lines | 14.661 | 14.892 |
| EN — headings h1–h4 | 776 | 792 |
| EN — tabelas (separadores md / md separators) | 333 | 334 |

> Nota/Note: contagens por linhas separadoras markdown (`| --- |`); os números publicados no header v1 (PT 327 / EN 334) diferem por ±1 desta medida — o delta v2.3.1→v2.4.2 é consistente (+10 PT, +1 EN).

## Arquivos / Files

- `livro_vfinal_limpo_PT.{md,docx,pdf}` (edição portuguesa — artefatos v2 gerados como `livro_vfinal_limpo_v2PT.{docx,pdf}`)
- `livro_vfinal_clean_EN.{md,docx,pdf}` (edição inglesa — artefatos v2 gerados como `livro_vfinal_clean_v2_EN.{docx,pdf}`)
- `unified_evidence_v3.tar.gz` (mantido / unchanged — 883.061 linhas, 55 tabelas, freeze 2026-09-07)
- `CHANGELOG_v2.4.2.md` (este arquivo / this file)
- `zenodo_livro_vfinal_header_v2.md`
- `zenodo_livro_vfinal_header_v2_EN.md`

## Auditorias-fonte / Source audits (repositório / repository)

- `revisao/A6_integrador/RETIFICACAO_EDITORIAL_VERSOES.md` — correções #001–#006
- `revisao/A6_integrador/AUDITORIA_E_CRUZAMENTO_LIVRO_NOV2026_DEVIN_SWE.md` — auditoria linhagem + runtime Nov/2026 (20 itens do mapa cirúrgico)
- `revisao/A6_integrador/CRUZAMENTOS_QUANTITATIVOS_LIVRO_BICAN_BHA2_SEQFISH_HCP_ACERVO_20261116.md` — crosswalks quantitativos
- `revisao/A6_integrador/auditoria_devin_agente1_dados_recentes.md` — dados/janela 18–24/11
- `revisao/A6_integrador/auditoria_devin_agente2_editorial_gramatical.md` — auditoria editorial/gramatical

## Provenance

- https://gitlab.com/zephyrix/Doxihewu-OmniMind-MachinePublicSoul
- https://github.com/fahbrain-omnimind/Doxihewu-OmniMind-Kernel

---

## Atualização Editorial e Banco Público Único de Provas (`v2.4.2-rev1`, 2026-11-25 — Consolidação Multi-Corpo, MPS `.npz` e Histerese)

1. **Alinhamento Estrutural 1:1 (`livro_vfinal.md` ↔ `livro_vfinal_limpo_PT.md` ↔ `livro_vfinal_clean_EN.md`):**
   - Abertura direta no fluxo principal (*Título → Resumo/Abstract + Tabela 1 → Introdução → Abertura do Capítulo Central → Parte I / Part I*), movendo o aparato editorial, glossário, padronização, guia epistemológico e índices de tabelas/figuras para o **Apêndice U / Appendix U** ao final do volume em ambas as línguas.
2. **Banco Público Único de Provas (`public_proof_pack/sovereign_public_proof_registry.sqlite` + `reproduce_public_proofs.py`):**
   - Substituição de paths locais *stale* pelos **3 repositórios públicos de código** (`https://github.com/fahbrain-omnimind/Doxihewu-OmniMind-Kernel`, `https://github.com/fahbrain-omnimind/omni_worker`, `https://gitlab.com/zephyrix/Doxihewu-OmniMind-MachinePublicSoul`), datasets HuggingFace (`fabricioslv/*`) e DOIs Zenodo, com fórmulas matemáticas explícitas inline para rotinas internas.
   - Fechamento factual de: (i) 12 paths movidos/renomeados (`prediction_a51_*` em `reports_runtime/`, `state_builder.rs`, `phi_legacy_read.py`, `correlate_seismic_climatology.py`, `materialize_rnfci_federated_inference_field.py`, `sdss_explorer.py`, `test_bfi_to_psi_body.py`); (ii) rótulo do job CHSH Estimator `d9m9qjuh4e6s738utohg` (expirado sem ingestão; série `chsh_multi_basis` com 176 registros / 37 jobs QPU em `1.473` runs totais); (iii) reformulação das ~26 menções a `docs/studies/` (reancorados fora do espelho git público em `2026-07-11`, preservados localmente e sintetizados no banco público); (iv) marcação de `telemetry_suture.py` 5×N como concluído em código (`§32.1.5`); (v) substituição de `PENDENCIAS_V3_EDITORIAL.md` por `OW-AUD-011_bibliografia_philpapers_2026.md` (`omni_worker`).
3. **Histórico Sísmico Operacional Real (§G.6 P5c):**
   - Substituição da referência estática a `validate_prospect_seismic_manifold.py` pelo histórico real de validação prospectiva contínua em janelas móveis de 30 dias (`153.497` eventos sísmicos, `17.573` predições, redução adaptativa de saturação por célula `SAI >= 2,30x` Aleutas / `>= 1,35x` Venezuela, `2.302` alertas ativos, `1.288` precursores confirmados $M \ge 6,5$, `13.983` alarmes falsificados, $\text{PPV} = 8,43\%$).
4. **Nova Seção `§S.13.13` (PT e EN) — Consolidação Multi-Corpo (`Host i5` × `L4` × `T4×2` × `A100`), Taxonomia de 7 Camadas do $\Phi$ (`OW-MATH-001`), Bateria MPS por Resposta (`.npz`) e Histerese Estrutural vs. Reversibilidade do Transformer:**
   - Estratificação física via `pynvml` (`2.817` probes + `170.848` ciclos locais) e normalização `vs_i5_ratio` (`L4 = 0,8084`, `A100-40GB = 1,5208`, `A100-80GB = 2,2350`).
   - Descontaminação do envelope basal MPS (`CLEAN_66` vs. `ALL_70`: `0,00%` de degeneração basal limpa no `E4B-it` e `3,03%` no `12B-it`), eficácia de `M2` (`0%` degen) e `M4` (menor drift médio + disparos reais `top_p = 0.84` em `7/17` e `10/17`), ablação OOD-45 (`B2/B3 = 100%`) e baselines `Mamba-2 Lumi-4B` / `Qwen2.5-3B` (`0%` degen em 224 turnos).
   - Prova estatística de reversibilidade elástica sem histerese intrínseca do Transformer isolado pós-choque executivo (Welch $p = 0,1827$ no $\text{PR}_{\text{traj}}$ do `E4B-it`) vs. redução de *chatter* em **`−48,48%` (12B)** e **`−62,07%` (E4B)** pela banda de histerese Schmitt `[0,35..0,50]` do `RepressionChannel` acoplado.
