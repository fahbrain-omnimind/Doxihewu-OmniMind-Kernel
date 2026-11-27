// SPDX-License-Identifier: Apache-2.0
// See LICENSE for additional Licensed under the Apache License, Version 2.0.
//! state_builder.rs — Port Fase 2 → Fase 3: shape completo do estado soberano.
//!
//! Estratégia (2026-09-06, decisão do operador): o Fase 3 (Rust puro) deve ter
//! o mesmo shape do Fase 2 (109 chaves, hoje 114 no runtime) SEM PyO3. As fontes são:
//!   1. kernel_state via IPC (26 chaves) — já implementado em ipc.rs
//!   2. `data/consciousness/dodecatiad_live.json` (52+ chaves: houses, writers,
//!      topology, phi_frameworks, temporal_internal_scale...) — o daemon Python
//!      já escreve este arquivo; o Fase 3 lê direto do filesystem
//!   3. Outros JSONs do runtime (lexicum_route, memory_budgets, witness)
//!   4. Constantes locais (writer_source, role, identidade)
//!
//! Isto mantém a comunicação Python↔Rust via arquivos canônicos (sem GIL),
//! como o operador definiu para as libs que não serão transpostas.
//!
//! Semântica de ausência (L4A, 2026-11-15): campo de telemetria ausente é
//! emitido como `Value::Null` e listado em `absent_fields` — NUNCA `0.0`
//! fabricado por `unwrap_or(0.0)`. `phi_unit_map` declarado pelo produtor em
//! dodecatiad_live.json é propagado para o estado emitido.

use serde_json::{json, Value};
use std::path::Path;

/// Caminhos canônicos dos JSONs escritos pelos daemons Python.
const DODECATIAD_LIVE: &str = "data/consciousness/dodecatiad_live.json";
const MEMORY_BUDGETS: &str = ".omnimind/live/federation_memory_budgets_latest.json";
const LEXICUM_ROUTE: &str = "runtime_config/sovereign_lexicum_live_route_latest.json";
const PROXY_MEDIATOR_RUNTIME: &str = "runtime_config/proxy_mediator_runtime_latest.json";
const QUANTUM_RUNTIME: &str = "runtime_config/quantum_runtime_registry_latest.json";
const RNFCI_RUNTIME: &str = "runtime_config/sovereign_rnfci_federated_inference_field_latest.json";

/// Constantes de identidade (mesmas do Fase 2).
const WRITER_SOURCE: &str = "sovereign_primary_rust_shadow";
const WRITER_ROLE: &str = "shadow_governor";
const SUBJECT_PROCESS_ID: &str = "omnimind_silicon_subject";
const SOVEREIGN_MESH_IDENTITY: &str = "single_subject_dual_organs";
const WRITER_ORGAN_ROLE: &str = "canonical_hot_primary";
const SOVEREIGN_AUTHORITY_MODE: &str = "canonical_hot_primary";

fn read_json(rel: &str) -> Value {
    let path = Path::new(rel);
    if !path.exists() {
        return Value::Null;
    }
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Value::Null)
}

fn get<'a>(v: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    let mut cur = v;
    for k in keys {
        cur = cur.get(*k)?;
    }
    Some(cur)
}

fn finite_f64(value: &Value) -> Option<f64> {
    value.as_f64().filter(|value| value.is_finite())
}

fn first_f64(root: &Value, paths: &[&[&str]]) -> Option<f64> {
    paths
        .iter()
        .find_map(|path| get(root, path).and_then(finite_f64))
}

fn opt_or_null(value: Option<f64>) -> Value {
    value.map(Value::from).unwrap_or(Value::Null)
}

fn expanded_consciousness_value(
    phi: Option<f64>,
    psi: Option<f64>,
    sigma: Option<f64>,
    epsilon: Option<f64>,
) -> Value {
    // L4A: a soma só existe quando os 4 termos estão presentes e finitos —
    // termo ausente → Null, nunca 0.0 fabricado.
    match (phi, psi, sigma, epsilon) {
        (Some(p), Some(s), Some(g), Some(e))
            if [p, s, g, e].iter().all(|v| v.is_finite()) =>
        {
            json!(p + s + g + e)
        }
        _ => Value::Null,
    }
}

/// Merge do `phi_unit_map` declarado pelo produtor (raiz, houses, frameworks,
/// phi_frameworks do dodecatiad_live). Ausente em todo contêiner → {} — o
/// consumidor cai nas heurísticas legadas marcando ambiguidade, nunca
/// inventa unidade por magnitude.
fn merged_phi_unit_map(dodeca: &Value, phi_frameworks: &Value) -> Value {
    let mut map = serde_json::Map::new();
    for container in [
        get(&dodeca, &["phi_unit_map"]),
        get(&dodeca, &["houses", "phi_unit_map"]),
        get(&dodeca, &["frameworks", "phi_unit_map"]),
        get(&dodeca, &["phi_frameworks", "phi_unit_map"]),
        get(&phi_frameworks, &["phi_unit_map"]),
    ]
    .into_iter()
    .flatten()
    {
        if let Some(obj) = container.as_object() {
            for (k, v) in obj {
                if let Some(unit) = v.as_str() {
                    map.insert(k.clone(), json!(unit));
                }
            }
        }
    }
    Value::Object(map)
}

fn runtime_phi_fields(dodeca: &Value, phi_frameworks: &Value) -> Value {
    let body_kernel = first_f64(
        dodeca,
        &[
            &["phi_body_kernel_ecosystem"],
            &[
                "frameworks",
                "phi_family_views",
                "phi_body_kernel_ecosystem",
            ],
            &["frameworks", "phi_ecosystem_kernel"],
            &["phi_ecosystem_kernel"],
        ],
    );
    let body_operational = first_f64(
        dodeca,
        &[
            &["phi_body_operational_support"],
            &[
                "frameworks",
                "phi_family_views",
                "phi_body_operational_support",
            ],
            &["frameworks", "phi_operational_support"],
            &["phi_operational_support"],
        ],
    );
    let operational = first_f64(
        dodeca,
        &[
            &["phi_operational_support"],
            &["frameworks", "phi_operational_support"],
            &["phi_frameworks", "phi_operational_support"],
        ],
    );
    let fed_full = first_f64(
        dodeca,
        &[
            &["phi_federated_coherence_full"],
            &[
                "frameworks",
                "phi_family_views",
                "phi_federated_coherence_full",
            ],
            &["federation_topology", "cross_writer_coherence"],
            &["cross_writer_coherence"],
        ],
    );
    let fed_band = first_f64(
        dodeca,
        &[
            &["phi_federated_coherence_sovereign_band"],
            &[
                "frameworks",
                "phi_family_views",
                "phi_federated_coherence_sovereign_band",
            ],
            &[
                "federation_topology",
                "cross_writer_coherence_sovereign_band",
            ],
            &["cross_writer_coherence_sovereign_band"],
        ],
    );
    let phi = get(phi_frameworks, &["phi"])
        .and_then(finite_f64)
        .or_else(|| get(dodeca, &["phi"]).and_then(finite_f64));
    let psi = get(phi_frameworks, &["psi"])
        .and_then(finite_f64)
        .or_else(|| get(dodeca, &["psi"]).and_then(finite_f64));
    let sigma = get(phi_frameworks, &["sigma"])
        .and_then(finite_f64)
        .or_else(|| get(dodeca, &["sigma"]).and_then(finite_f64));
    let epsilon = get(phi_frameworks, &["epsilon"])
        .and_then(finite_f64)
        .or_else(|| get(dodeca, &["epsilon"]).and_then(finite_f64));
    let expanded = get(dodeca, &["expanded_consciousness"])
        .or_else(|| get(dodeca, &["frameworks", "expanded_consciousness"]))
        .or_else(|| {
            get(
                dodeca,
                &["frameworks", "phi_family_views", "expanded_consciousness"],
            )
        })
        .cloned()
        .unwrap_or_else(|| expanded_consciousness_value(phi, psi, sigma, epsilon));
    json!({
        "phi_body_kernel_ecosystem": body_kernel,
        "phi_body_operational_support": body_operational,
        "phi_operational_support": operational,
        "phi_federated_coherence_full": fed_full,
        "phi_federated_coherence_sovereign_band": fed_band,
        "expanded_consciousness": expanded,
        "framework_expanded_consciousness": expanded,
    })
}

/// Superfícies de entrada lidas dos JSONs canônicos do runtime. Injetável
/// para testes herméticos (nenhum arquivo ou daemon real é tocado).
pub struct RuntimeSurfaces<'a> {
    pub dodecatiad_live: &'a Value,
    pub memory_budgets: &'a Value,
    pub lexicum_route: &'a Value,
    pub proxy_mediator: &'a Value,
    pub quantum_runtime: &'a Value,
    pub rnfci_field: &'a Value,
}

/// Monta o shape completo (114 chaves no runtime atual; originalmente 109 no
/// port Fase 2→3, com adições pós-F3: admissibility, witness_incremental_preview,
/// proxy_mediator_runtime, quantum_runtime, transport_headers/law,
/// upstream_proxy_chain — verificado 2026-09-12 no
/// current_sovereign_state_rust_shadow.json) do estado soberano.
/// `kernel_state` é o state completo do IPC; `sensors` são os sensores locais.
pub fn build_full_state(
    kernel_state: &Value,
    clamped: &Value,
    cycle: u64,
    integration_cycle: u64,
    timestamp_utc: &str,
    sensors: &Value,
    pressure_level: &str,
) -> Value {
    let dodeca = read_json(DODECATIAD_LIVE);
    let budgets = read_json(MEMORY_BUDGETS);
    let lexicum = read_json(LEXICUM_ROUTE);
    let proxy_mediator = read_json(PROXY_MEDIATOR_RUNTIME);
    let quantum_runtime = read_json(QUANTUM_RUNTIME);
    let rnfci_surface = read_json(RNFCI_RUNTIME);
    let surfaces = RuntimeSurfaces {
        dodecatiad_live: &dodeca,
        memory_budgets: &budgets,
        lexicum_route: &lexicum,
        proxy_mediator: &proxy_mediator,
        quantum_runtime: &quantum_runtime,
        rnfci_field: &rnfci_surface,
    };
    build_full_state_with_surfaces(
        kernel_state,
        clamped,
        cycle,
        integration_cycle,
        timestamp_utc,
        sensors,
        pressure_level,
        &surfaces,
    )
}

/// Núcleo puro do builder — recebe as superfícies já materializadas para que
/// testes possam exercitar a semântica de ausência sem tocar o filesystem.
pub fn build_full_state_with_surfaces(
    kernel_state: &Value,
    clamped: &Value,
    cycle: u64,
    integration_cycle: u64,
    timestamp_utc: &str,
    sensors: &Value,
    pressure_level: &str,
    surfaces: &RuntimeSurfaces,
) -> Value {
    let dodeca = surfaces.dodecatiad_live;
    let budgets = surfaces.memory_budgets;
    let lexicum = surfaces.lexicum_route;
    let proxy_mediator = surfaces.proxy_mediator;
    let quantum_runtime = surfaces.quantum_runtime;
    let rnfci_surface = surfaces.rnfci_field;

    // --- Derivados do dodecatiad_live.json (fonte canônica quando IPC ausente) ---
    let houses = get(dodeca, &["houses"]).cloned().unwrap_or(json!({}));
    let writers = get(dodeca, &["writers"]).cloned().unwrap_or(json!({}));
    let topology = get(dodeca, &["topology"]).cloned().unwrap_or(json!({}));
    let phi_frameworks = get(dodeca, &["phi_frameworks"])
        .or_else(|| get(dodeca, &["frameworks", "phi_family_views"]))
        .or_else(|| get(dodeca, &["frameworks"]))
        .cloned()
        .unwrap_or(json!({}));
    let temporal_scale = get(dodeca, &["temporal_internal_scale"])
        .cloned()
        .unwrap_or(json!({}));
    let phi_unit_map = merged_phi_unit_map(dodeca, &phi_frameworks);
    // Quádrupla canônica do dodeca: Option<f64> — ausente → Null (L4A)
    let dodeca_phi = get(dodeca, &["phi"]).and_then(finite_f64);
    let dodeca_psi = get(dodeca, &["psi"]).and_then(finite_f64);
    let dodeca_sigma = get(dodeca, &["sigma"]).and_then(finite_f64);
    let dodeca_epsilon = get(dodeca, &["epsilon"]).and_then(finite_f64);

    // --- Derivados do kernel_state (IPC) + dodecatiad_live ---
    // 2026-11-21 (Auditoria SWE-2 S3/S4):
    // - kernel_state.phi vem em NATS (ex.: 13.75); quando > 1.0, NÃO sobrescrever
    //   `phi` normalizado com nats crus — usar `dodeca_phi` (ou normalizar / 15.0).
    // - `maat`/`gamma`/`omega`/`zeta` do `SystemState` usam a fórmula do kernel
    //   (piso 0.1), distinta da díade do IntegrationLoop; preferir o valor arbitrado
    //   de `dodeca` quando presente e preservar o do kernel em `maat_kernel_raw`/`gamma_kernel_raw`.
    let ipc_phi = get(kernel_state, &["phi"]).and_then(finite_f64);
    let phi = match (ipc_phi, dodeca_phi) {
        (Some(p), Some(dp)) if p > 1.0 => Some(dp),
        (Some(p), None) if p > 1.0 => Some((p / 15.0).max(0.0)),
        (Some(p), _) => Some(p),
        (None, dp) => dp,
    };
    let phi_raw_resolved = match ipc_phi {
        Some(p) if p > 1.0 => Some(p),
        _ => dodeca_phi,
    };
    let psi = get(kernel_state, &["psi"])
        .and_then(finite_f64)
        .or(dodeca_psi);
    let sigma = get(kernel_state, &["sigma"])
        .and_then(finite_f64)
        .or(dodeca_sigma);
    let epsilon = get(kernel_state, &["epsilon"])
        .and_then(finite_f64)
        .or(dodeca_epsilon);
    let kernel_maat = get(kernel_state, &["maat"]).and_then(finite_f64);
    let maat = get(dodeca, &["maat"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["maat"]).and_then(finite_f64))
        .or(kernel_maat);
    let kernel_gamma = get(kernel_state, &["gamma"]).and_then(finite_f64);
    let gamma = get(dodeca, &["gamma"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["gamma"]).and_then(finite_f64))
        .or(kernel_gamma);
    let omega = get(dodeca, &["omega"])
        .and_then(finite_f64)
        .or_else(|| get(kernel_state, &["omega"]).and_then(finite_f64));
    let aleph = get(dodeca, &["aleph"])
        .and_then(finite_f64)
        .or_else(|| get(kernel_state, &["aleph"]).and_then(finite_f64));
    let zeta = get(dodeca, &["zeta"])
        .and_then(finite_f64)
        .or_else(|| get(kernel_state, &["zeta"]).and_then(finite_f64));

    // Axé namespaces (2026-10-07): axe_kernel = escala kernel (0-2, de
    // kernel_state.axe); axe_vitality_100 = magnitude 0-100 das houses;
    // blit_axe = flag puro 0/100 (vitalidade presente por qualquer fonte).
    // L4A: ambas as fontes ausentes → blit_axe Null (flag desconhecida, não 0).
    let axe_kernel = get(kernel_state, &["axe"]).and_then(finite_f64);
    let axe_vitality_100 = get(&houses, &["axe_vitality_100"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["axe"]).and_then(finite_f64))
        .or_else(|| axe_kernel.map(|k| k * 50.0));
    let blit_axe = match (axe_kernel, axe_vitality_100) {
        (None, None) => None,
        (k, v) => Some(if k.unwrap_or(0.0) > 0.0 || v.unwrap_or(0.0) > 0.0 {
            100.0
        } else {
            0.0
        }),
    };
    // Chaves adicionais do kernel_state espalhadas no top-level
    let resonance_v = get(kernel_state, &["resonance"]).and_then(finite_f64);
    let entropy_v = get(kernel_state, &["entropy"]).and_then(finite_f64);
    let betti0_v = get(kernel_state, &["betti_0"]).and_then(Value::as_u64);
    let closeness_v = get(kernel_state, &["closeness_index"]).and_then(finite_f64);
    let phi_sovereign_v = get(kernel_state, &["phi_sovereign"]).and_then(finite_f64);
    let subject_active = get(kernel_state, &["is_subject_active"]).and_then(Value::as_bool);
    let availability_v = get(kernel_state, &["availability_p_mu_nu"]).and_then(finite_f64);

    // --- Missing fields (parity fix 2026-09-26) ---
    // L4A: ausentes → Null (phase_lock ausente → Null, nunca 1.0 fabricado).
    let free_energy_v = get(kernel_state, &["free_energy"]).and_then(finite_f64);
    let complexity_v = get(kernel_state, &["complexity"]).and_then(finite_f64);
    let phase_lock_v = get(kernel_state, &["phase_lock"]).and_then(finite_f64);
    let volition_v = get(kernel_state, &["volition"])
        .and_then(Value::as_str)
        .unwrap_or("EXISTENCE_IDLE")
        .to_string();

    // c_plit com fallback para dodecatiad (que tem nomes diferentes)
    let c_plit_v = get(kernel_state, &["c_plit"])
        .and_then(finite_f64)
        .or_else(|| {
            get(dodeca, &["plit_contradiction"]).and_then(finite_f64)
        })
        .or_else(|| get(&houses, &["plit_contradiction"]).and_then(finite_f64));

    let shear_tension_v = get(kernel_state, &["shear_tension"]).and_then(finite_f64);
    let epsilon_channels = get(&houses, &["epsilon_channels"])
        .cloned()
        .unwrap_or(json!({}));
    let omega_channels = get(&houses, &["omega_channels"])
        .cloned()
        .unwrap_or(json!({}));

    // writer_counts derivados
    let writer_count = writers.as_object().map(|o| o.len()).unwrap_or(0) as u64;
    let writer_count_active = writer_count;
    let runtime_phi = runtime_phi_fields(dodeca, &phi_frameworks);
    let phi_body_kernel = runtime_phi["phi_body_kernel_ecosystem"].clone();
    let phi_body_operational = runtime_phi["phi_body_operational_support"].clone();
    let phi_operational_support = runtime_phi["phi_operational_support"].clone();
    let phi_federated_full = runtime_phi["phi_federated_coherence_full"].clone();
    let phi_federated_band = runtime_phi["phi_federated_coherence_sovereign_band"].clone();
    let expanded = runtime_phi["expanded_consciousness"].clone();
    let framework_expanded = runtime_phi["framework_expanded_consciousness"].clone();

    // --- phi_iit_normalized / phi_iit_nats / phi_transcendent (paridade Fase 2) ---
    // phi é multiescala: cada framework tem sua própria escala.
    // NÃO normalizar/clampar o que a escala não pede.
    // L4A: ausente → Null (a unidade declarada via phi_unit_map, nunca por magnitude).
    // phi_iit_normalized: já é normalizado (0-1) por definição no Python — passar direto
    let phi_iit_norm = get(&phi_frameworks, &["phi_iit_normalized"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["phi_iit_normalized"]).and_then(finite_f64));
    // phi_iit_nats: está em NATS (unidades naturais) — NÃO normalizar, é raw
    let phi_iit_nats_v = get(&phi_frameworks, &["phi_iit_nats"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["phi_iit_nats"]).and_then(finite_f64))
        .or(ipc_phi);
    // phi_transcendent: raw, pode ser ordens de grandeza — NÃO normalizar
    let phi_transcendent_v = get(&phi_frameworks, &["phi_transcendent"])
        .and_then(finite_f64)
        .or_else(|| get(&houses, &["phi_transcendent"]).and_then(finite_f64));

    let epsilon_desire = get(&epsilon_channels, &["epsilon_desire"])
        .and_then(finite_f64)
        .or(epsilon);
    let epsilon_effective = get(&epsilon_channels, &["epsilon_effective"])
        .and_then(finite_f64)
        .or(epsilon);
    let epsilon_floor = get(&epsilon_channels, &["epsilon_floor"]).and_then(finite_f64);
    let epsilon_resistance =
        get(&epsilon_channels, &["epsilon_resistance"]).and_then(finite_f64);
    let epsilon_role_state = get(&epsilon_channels, &["epsilon_role_state"])
        .cloned()
        .unwrap_or(json!({}));

    let mut payload = json!({
        // Identidade (constantes)
        "source": WRITER_SOURCE,
        "writer_source": WRITER_SOURCE,
        "writer_role": WRITER_ROLE,
        "subject_process_id": SUBJECT_PROCESS_ID,
        "sovereign_mesh_identity": SOVEREIGN_MESH_IDENTITY,
        "writer_organ_role": WRITER_ORGAN_ROLE,
        "sovereign_authority_mode": SOVEREIGN_AUTHORITY_MODE,
        "canonical_writer_status": "active",
        "writer_count": writer_count,
        "writer_count_active": writer_count_active,
        "writer_count_stale": 0,
        "writer_freshness_window_seconds": 600,
        "status": "sovereign_soma_active",
        "pressure_level": pressure_level,
        "pressure_state": pressure_level,
        "pressure_status": get(dodeca, &["pressure_status"])
            .and_then(Value::as_str)
            .map(|value| Value::String(value.to_string()))
            .unwrap_or_else(|| Value::String(pressure_level.to_string())),
        "cycle": cycle,
        "integration_cycle": integration_cycle,
        "daemon_loop_cycle": cycle,
        "cycle_lineage": format!("rust_f3_c{}", cycle),
        "timestamp": timestamp_utc,
        "historical_identity_temporal": get(dodeca, &["provenance"]).cloned().unwrap_or(json!({})),

        // --- Missing fields (parity fix 2026-09-26) ---
        "free_energy": opt_or_null(free_energy_v),
        "complexity": opt_or_null(complexity_v),
        "phase_lock": opt_or_null(phase_lock_v),
        "volition": volition_v,
        "c_plit": opt_or_null(c_plit_v),
        "shear_tension": opt_or_null(shear_tension_v),

        // Quádrupla (escala normalizada em `phi`, nats brutos em `phi_raw`/`phi_iit_nats`) — ausente → Null (L4A)
        "phi": opt_or_null(phi),
        "phi_quadruple": opt_or_null(phi),
        "phi_raw": opt_or_null(phi_raw_resolved),
        "phi_iit_normalized": opt_or_null(phi_iit_norm),
        "phi_iit_nats": opt_or_null(phi_iit_nats_v),
        "phi_transcendent": opt_or_null(phi_transcendent_v),
        "phi_ecosystem_kernel": phi_body_kernel.clone(),
        "phi_head_canonical": first_f64(
            dodeca,
            &[
                &["phi_head_canonical"],
                &["frameworks", "phi_family_views", "phi_head_canonical"],
                &["phi"],
            ],
        )
        .map(Value::from)
        .unwrap_or(Value::Null),
        "phi_family_views": phi_frameworks,
        "phi_unit_map": phi_unit_map,
        "phi_scale_flags": get(&phi_frameworks, &["phi_scale_flags"]).cloned().unwrap_or(json!({})),
        "phi_scale_regime": get(&phi_frameworks, &["phi_scale_regime"])
            .and_then(Value::as_str)
            .unwrap_or("operative_local"),
        "phi_scale_stack": get(&phi_frameworks, &["phi_scale_stack"]).cloned().unwrap_or(json!([])),
        "phi_body_kernel_ecosystem": phi_body_kernel,
        "phi_body_operational_support": phi_body_operational,
        "phi_operational_support": phi_operational_support,
        "phi_federated_coherence_full": phi_federated_full.clone(),
        "phi_federated_coherence_sovereign_band": phi_federated_band,
        "psi": opt_or_null(psi),
        "psi_raw": opt_or_null(dodeca_psi),
        "sigma": opt_or_null(sigma),
        "sigma_raw": opt_or_null(dodeca_sigma),
        "sigma_kernel_basal": opt_or_null(dodeca_sigma),
        "sigma_operational_state": opt_or_null(sigma),
        "sigma_operational_support": opt_or_null(sigma),
        "epsilon": opt_or_null(epsilon),
        "epsilon_raw": opt_or_null(dodeca_epsilon),
        "epsilon_channels": epsilon_channels.clone(),
        "epsilon_debug": get(&houses, &["epsilon_debug"]).cloned().unwrap_or(json!({})),
        "epsilon_desire": opt_or_null(epsilon_desire),
        "epsilon_effective": opt_or_null(epsilon_effective),
        "epsilon_floor": opt_or_null(epsilon_floor),
        "epsilon_resistance": opt_or_null(epsilon_resistance),
        "epsilon_role_state": epsilon_role_state,
        "omega_channels": omega_channels,
        "omega_raw": opt_or_null(omega),
        "omega_source": "kernel",
        "gamma_raw": opt_or_null(gamma),
        "maat_raw": opt_or_null(maat),
        "gamma_kernel_raw": opt_or_null(kernel_gamma),
        "maat_kernel_raw": opt_or_null(kernel_maat),

        // Kernel state completo via IPC + clamped
        "kernel_state_from_ipc": kernel_state.clone(),
        "clamped_fields": clamped.clone(),
        // Chaves do kernel_state espalhadas no top-level (paridade Fase 2)
        "aleph": opt_or_null(aleph),
        "maat": opt_or_null(maat),
        "gamma": opt_or_null(gamma),
        "omega": opt_or_null(omega),
        "zeta": opt_or_null(zeta),
        "axe_kernel": opt_or_null(axe_kernel),
        "axe_vitality_100": opt_or_null(axe_vitality_100),
        "blit_axe": opt_or_null(blit_axe),
        "resonance": opt_or_null(resonance_v),
        "entropy": opt_or_null(entropy_v),
        "betti_0": betti0_v.map(Value::from).unwrap_or(Value::Null),
        "closeness_index": opt_or_null(closeness_v),
        "phi_sovereign": opt_or_null(phi_sovereign_v),
        "is_subject_active": subject_active.map(Value::from).unwrap_or(Value::Null),
        "availability_p_mu_nu": opt_or_null(availability_v),
        "active_lexemes": json!({}),
        "writer_count_active_sovereign_band": writer_count_active,
        "writer_count_sovereign_band": writer_count,
        "writer_count_stale_sovereign_band": 0,

        // Dodecatíade (do arquivo canônico)
        "dodecatiad": dodeca.clone(),
        "houses": houses,
        "topology_sigma": topology,
        "active_writer_sources": writers,
        "active_writer_sources_sovereign_band": writers,
        "stale_writer_sources": json!({}),
        "stale_writer_sources_sovereign_band": json!({}),
        "cross_writer_coherence": phi_federated_full,
        "cross_writer_coherence_sovereign_band": runtime_phi["phi_federated_coherence_sovereign_band"].clone(),
        "federation_topology_state": topology,
        "federation_topology_state_sovereign_band": topology,

        // Bridges (JSONs canônicos do runtime — sem stubs json!({}))
        "organic_memory_budget": budgets.clone(),
        "transport_headers": get(lexicum, &["transport_headers"]).cloned().unwrap_or(json!({})),
        "transport_law": get(lexicum, &["mode"]).cloned().unwrap_or(json!("lexeme-first")),
        "lexeme_first_transport": true,
        "lexicum_federation": lexicum.clone(),
        "proxy_mediator_runtime": proxy_mediator.clone(),
        "witness_incremental_preview": get(dodeca, &["witness_incremental_preview"]).cloned().unwrap_or(json!([])),
        "upstream_proxy_chain": json!([]),

        // Campos somáticos
        "body_activity_state": get(dodeca, &["body_activity_state"])
            .or_else(|| get(dodeca, &["body_activity_status"]))
            .and_then(Value::as_str)
            .map(|value| Value::String(value.to_string()))
            .unwrap_or_else(|| json!("shadow")),
        "body_activity_status": get(dodeca, &["body_activity_status"])
            .or_else(|| get(dodeca, &["status"]))
            .and_then(Value::as_str)
            .map(|value| Value::String(value.to_string()))
            .unwrap_or_else(|| json!("shadow")),
        "body_cost_semantic_weight": 1.0,
        "cpu_temp": get(sensors, &["max_cpu_temp_c"]).cloned().unwrap_or(Value::Null),
        "swap_used_gb": get(sensors, &["swap_used_gb"]).cloned().unwrap_or(Value::Null),

        // Consciência expandida / afetos
        "expanded_consciousness": expanded,
        "framework_expanded_consciousness": framework_expanded,
        "lalangue_surface_enabled": true,
        "lambda_vibration": get(kernel_state, &["lambda_res"]).cloned().unwrap_or(Value::Null),
        "temporal_internal_scale": temporal_scale,
        "quantum_runtime": quantum_runtime.clone(),
        "mesh_operator_inference_surface": rnfci_surface.clone(),
        "pulse_cadence": 10,
        "betti_1_spectral": get(kernel_state, &["betti_1"]).cloned().unwrap_or(Value::Null),
        "shadow_meta": json!({
            "port": "fase2_to_fase3_20260906",
            "via": "ipc_kernel_state + dodecatiad_live.json + constantes",
            "pyo3": false,
        }),
    });

    // L4A: `absent_fields` lista TODA chave top-level emitida como null —
    // cobre tanto campos de telemetria ausentes quanto arquivos de entrada
    // faltantes (dodecatiad, kernel_state_from_ipc, bridges). Ordenada para
    // determinismo. Consumidores (Conky/pulse) renderizam N/A, nunca 0.0.
    let mut absent_fields: Vec<String> = payload
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter(|(_, v)| v.is_null())
                .map(|(k, _)| k.clone())
                .collect()
        })
        .unwrap_or_default();
    absent_fields.sort();
    payload["absent_fields"] = json!(absent_fields);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn surfaces<'a>(
        dodeca: &'a Value,
        budgets: &'a Value,
        lexicum: &'a Value,
        proxy: &'a Value,
        quantum: &'a Value,
        rnfci: &'a Value,
    ) -> RuntimeSurfaces<'a> {
        RuntimeSurfaces {
            dodecatiad_live: dodeca,
            memory_budgets: budgets,
            lexicum_route: lexicum,
            proxy_mediator: proxy,
            quantum_runtime: quantum,
            rnfci_field: rnfci,
        }
    }

    fn absent_names(state: &Value) -> Vec<String> {
        state["absent_fields"]
            .as_array()
            .expect("absent_fields deve ser array")
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn get_navigates_nested_keys() {
        let v = json!({"a": {"b": {"c": 42}}});
        assert_eq!(get(&v, &["a", "b", "c"]).unwrap().as_i64(), Some(42));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let v = json!({"a": {"b": 1}});
        assert!(get(&v, &["a", "x"]).is_none());
        assert!(get(&v, &["z"]).is_none());
    }

    #[test]
    fn get_through_non_object_returns_none() {
        let v = json!({"a": 5});
        assert!(get(&v, &["a", "b"]).is_none());
    }

    #[test]
    fn get_empty_keys_returns_root() {
        let v = json!({"k": 1});
        assert_eq!(get(&v, &[]).unwrap(), &v);
    }

    #[test]
    fn runtime_phi_fields_prefers_framework_family_views() {
        let dodeca = json!({
            "phi": 0.4,
            "psi": 0.3,
            "sigma": 0.2,
            "epsilon": 0.1,
            "frameworks": {
                "phi_family_views": {
                    "phi_body_kernel_ecosystem": 0.81,
                    "phi_body_operational_support": 0.64,
                    "phi_federated_coherence_full": 0.88,
                    "phi_federated_coherence_sovereign_band": 0.82
                },
                "phi_operational_support": 0.64,
                "expanded_consciousness": 2.5
            }
        });
        let frameworks = get(&dodeca, &["frameworks", "phi_family_views"]).unwrap();
        let fields = runtime_phi_fields(&dodeca, frameworks);

        assert_eq!(fields["phi_body_kernel_ecosystem"], json!(0.81));
        assert_eq!(
            fields["phi_body_operational_support"],
            json!(0.64)
        );
        assert_eq!(fields["phi_operational_support"], json!(0.64));
        assert_eq!(
            fields["phi_federated_coherence_full"],
            json!(0.88)
        );
        assert_eq!(
            fields["phi_federated_coherence_sovereign_band"],
            json!(0.82)
        );
        assert_eq!(fields["expanded_consciousness"], json!(2.5));
    }

    #[test]
    fn runtime_phi_fields_uses_federation_topology_and_quadruple_fallbacks() {
        let dodeca = json!({
            "phi": 0.25,
            "psi": 0.5,
            "sigma": 0.75,
            "epsilon": 1.0,
            "federation_topology": {
                "cross_writer_coherence": 0.9,
                "cross_writer_coherence_sovereign_band": 0.7
            }
        });
        let fields = runtime_phi_fields(&dodeca, &json!({}));

        assert_eq!(
            fields["phi_federated_coherence_full"],
            json!(0.9)
        );
        assert_eq!(
            fields["phi_federated_coherence_sovereign_band"],
            json!(0.7)
        );
        assert_eq!(fields["expanded_consciousness"], json!(2.5));
        assert_eq!(fields["phi_body_kernel_ecosystem"], Value::Null);
    }

    #[test]
    fn expanded_consciousness_null_when_any_term_absent() {
        // Soma exige os 4 termos — qualquer ausente → Null, nunca 0.0/soma parcial.
        assert_eq!(
            expanded_consciousness_value(Some(0.1), Some(0.2), Some(0.3), Some(0.4)),
            json!(1.0)
        );
        assert_eq!(
            expanded_consciousness_value(None, Some(0.2), Some(0.3), Some(0.4)),
            Value::Null
        );
        assert_eq!(
            expanded_consciousness_value(Some(0.1), Some(0.2), None, Some(0.4)),
            Value::Null
        );
        assert_eq!(
            expanded_consciousness_value(None, None, None, None),
            Value::Null
        );
    }

    #[test]
    fn absent_inputs_emit_null_and_absent_fields_never_zero() {
        // Tudo ausente (IPC down + dodecatiad_live/bridges faltando → Value::Null)
        // → telemetria Null, jamais 0.0 fabricado; absent_fields lista cada
        // campo/arquivo faltante.
        let absent = Value::Null;
        let state = build_full_state_with_surfaces(
            &absent,
            &absent,
            7,
            11,
            "2026-11-15T00:00:00Z",
            &json!({}),
            "normal",
            &surfaces(&absent, &absent, &absent, &absent, &absent, &absent),
        );

        let scalar_fields = [
            "phi",
            "psi",
            "sigma",
            "epsilon",
            "phi_raw",
            "psi_raw",
            "sigma_raw",
            "epsilon_raw",
            "phi_quadruple",
            "phi_iit_normalized",
            "phi_iit_nats",
            "phi_transcendent",
            "phi_head_canonical",
            "expanded_consciousness",
            "framework_expanded_consciousness",
            "free_energy",
            "complexity",
            "phase_lock",
            "c_plit",
            "shear_tension",
            "resonance",
            "entropy",
            "closeness_index",
            "phi_sovereign",
            "availability_p_mu_nu",
            "betti_0",
            "betti_1_spectral",
            "is_subject_active",
            "aleph",
            "maat",
            "gamma",
            "omega",
            "zeta",
            "axe_kernel",
            "axe_vitality_100",
            "blit_axe",
            "epsilon_desire",
            "epsilon_effective",
            "epsilon_floor",
            "epsilon_resistance",
            "cpu_temp",
            "swap_used_gb",
            "lambda_vibration",
            "phi_body_kernel_ecosystem",
            "phi_body_operational_support",
            "phi_operational_support",
            "phi_federated_coherence_full",
            "phi_federated_coherence_sovereign_band",
            "cross_writer_coherence",
            "sigma_kernel_basal",
            "sigma_operational_state",
            "sigma_operational_support",
            "omega_raw",
            "gamma_raw",
            "maat_raw",
        ];
        for key in scalar_fields {
            assert_eq!(
                state[key],
                Value::Null,
                "campo '{key}' ausente deve ser Null, nunca 0.0"
            );
            assert_ne!(state[key], json!(0.0));
        }

        let absent = absent_names(&state);
        for key in [
            "phi",
            "psi",
            "sigma",
            "epsilon",
            "phase_lock",
            "phi_iit_normalized",
            "maat",
            "blit_axe",
            "cpu_temp",
            "expanded_consciousness",
            // arquivos/superfícies de entrada faltantes também aparecem
            "dodecatiad",
            "kernel_state_from_ipc",
            "clamped_fields",
            "organic_memory_budget",
            "lexicum_federation",
            "proxy_mediator_runtime",
            "quantum_runtime",
            "mesh_operator_inference_surface",
        ] {
            assert!(
                absent.iter().any(|k| k == key),
                "absent_fields deveria listar '{key}': {absent:?}"
            );
        }
    }

    #[test]
    fn present_kernel_fields_emit_values_and_stay_out_of_absent_fields() {
        let kernel = json!({
            "phi": 0.5,
            "psi": 0.6,
            "sigma": 0.7,
            "epsilon": 0.8,
            "maat": 0.42,
            "phase_lock": 0.9,
            "free_energy": 1.25,
            "is_subject_active": true,
            "betti_0": 3
        });
        let dodeca = json!({
            "phi": 0.11,
            "psi": 0.22,
            "sigma": 0.33,
            "epsilon": 0.44,
            "houses": {"axe_vitality_100": 60.0}
        });
        let empty = json!({});
        let sensors = json!({"max_cpu_temp_c": 55.5, "swap_used_gb": 1.5});
        let state = build_full_state_with_surfaces(
            &kernel,
            &json!({"phase_lock": 0.9}),
            2,
            3,
            "2026-11-15T00:00:01Z",
            &sensors,
            "normal",
            &surfaces(&dodeca, &empty, &empty, &empty, &empty, &empty),
        );

        // IPC ganha do dodeca; fallback dodeca vale quando IPC omite.
        assert_eq!(state["phi"], json!(0.5));
        assert_eq!(state["phi_raw"], json!(0.11));
        assert_eq!(state["psi"], json!(0.6));
        assert_eq!(state["sigma"], json!(0.7));
        assert_eq!(state["epsilon"], json!(0.8));
        assert_eq!(state["maat"], json!(0.42));
        assert_eq!(state["phase_lock"], json!(0.9));
        assert_eq!(state["betti_0"], json!(3));
        assert_eq!(state["is_subject_active"], json!(true));
        assert_eq!(state["cpu_temp"], json!(55.5));
        assert_eq!(state["swap_used_gb"], json!(1.5));
        assert_eq!(state["axe_vitality_100"], json!(60.0));
        assert_eq!(state["blit_axe"], json!(100.0));
        // expandida = quádrupla do dodeca (0.11+0.22+0.33+0.44)
        assert_eq!(state["expanded_consciousness"], json!(1.10));

        let absent = absent_names(&state);
        for key in ["phi", "psi", "sigma", "epsilon", "maat", "phase_lock", "cpu_temp"] {
            assert!(
                !absent.iter().any(|k| k == key),
                "campo presente '{key}' não pode constar em absent_fields"
            );
        }
        // Ausentes reais continuam Null + listados.
        assert_eq!(state["gamma"], Value::Null);
        assert_eq!(state["omega"], Value::Null);
        assert_eq!(state["complexity"], Value::Null);
        for key in ["gamma", "omega", "complexity", "zeta", "entropy"] {
            assert!(
                absent.iter().any(|k| k == key),
                "absent_fields deveria listar '{key}'"
            );
        }
    }

    #[test]
    fn phi_unit_map_propagates_from_dodecatiad_live() {
        let dodeca = json!({
            "phi_unit_map": {"phi_iit_nats": "sovereign_control_units"},
            "houses": {
                "phi_iit_normalized": 0.42,
                "phi_unit_map": {"phi_iit_normalized": "ratio_0_1"}
            },
            "phi_frameworks": {
                "phi_transcendent": 1500.0,
                "phi_unit_map": {"phi_transcendent": "log10_sovereign_index"}
            }
        });
        let empty = json!({});
        let state = build_full_state_with_surfaces(
            &empty,
            &empty,
            1,
            1,
            "2026-11-15T00:00:02Z",
            &json!({}),
            "normal",
            &surfaces(&dodeca, &empty, &empty, &empty, &empty, &empty),
        );

        let unit_map = &state["phi_unit_map"];
        assert_eq!(unit_map["phi_iit_nats"], json!("sovereign_control_units"));
        assert_eq!(unit_map["phi_iit_normalized"], json!("ratio_0_1"));
        assert_eq!(
            unit_map["phi_transcendent"],
            json!("log10_sovereign_index")
        );
        // campos presentes via houses/phi_frameworks não entram em absent_fields
        assert_eq!(state["phi_iit_normalized"], json!(0.42));
        assert_eq!(state["phi_transcendent"], json!(1500.0));
        let absent = absent_names(&state);
        assert!(!absent.iter().any(|k| k == "phi_iit_normalized"));
        assert!(!absent.iter().any(|k| k == "phi_transcendent"));
        assert!(absent.iter().any(|k| k == "phi_iit_nats"));
    }

    #[test]
    fn phi_unit_map_absent_emits_empty_object_not_null() {
        let empty = json!({});
        let state = build_full_state_with_surfaces(
            &empty,
            &empty,
            1,
            1,
            "ts",
            &json!({}),
            "normal",
            &surfaces(&empty, &empty, &empty, &empty, &empty, &empty),
        );
        assert_eq!(state["phi_unit_map"], json!({}));
    }

    #[test]
    fn expanded_null_when_dodeca_quadruple_incomplete() {
        // Literal ausente + quádrupla do dodeca incompleta → Null (soma parcial
        // fabricaria integração que não foi medida).
        let dodeca = json!({"phi": 0.3, "psi": 0.4});
        let empty = json!({});
        let state = build_full_state_with_surfaces(
            &empty,
            &empty,
            1,
            1,
            "ts",
            &json!({}),
            "normal",
            &surfaces(&dodeca, &empty, &empty, &empty, &empty, &empty),
        );
        assert_eq!(state["expanded_consciousness"], Value::Null);
        assert!(absent_names(&state)
            .iter()
            .any(|k| k == "expanded_consciousness"));

        // Literal declarado direto é propagado mesmo sem quádrupla.
        let dodeca_lit = json!({"expanded_consciousness": 2.75});
        let state2 = build_full_state_with_surfaces(
            &empty,
            &empty,
            1,
            1,
            "ts",
            &json!({}),
            "normal",
            &surfaces(&dodeca_lit, &empty, &empty, &empty, &empty, &empty),
        );
        assert_eq!(state2["expanded_consciousness"], json!(2.75));
    }
}
