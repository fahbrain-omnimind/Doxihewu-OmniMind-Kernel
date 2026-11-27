// SPDX-License-Identifier: Apache-2.0
// See LICENSE for additional Licensed under the Apache License, Version 2.0.
//! Port Rust da função hot-path `_score_live_writer_snapshot` do IntegrationLoop.
//!
//! Esta função é chamada para cada writer snapshot (potencialmente múltiplas
//! vezes por ciclo) para pontuar a qualidade do snapshot e selecionar o
//! writer canônico. É compute puro: sem I/O, sem estado, apenas aritmética.
//!
//! Design: a função aceita valores primitivos (f64, bool, &str) extraídos
//! pelo wrapper Python. Isso evita o overhead de 30+ chamadas dict.get()
//! via PyO3, mantendo o dict access em Python (rápido) e a aritmética
//! em Rust (f64 nativo).
//!
//! Paridade f64 esperada: ≤ 1e-15
//!
//! Referência Python: `src/consciousness/integration_loop.py:6213-6318`

use pyo3::prelude::*;

/// Pontua a qualidade de um writer snapshot.
///
/// Python ref: `IntegrationLoop._score_live_writer_snapshot` (l.6213-6318)
///
/// Args (todos extraídos do dict pelo wrapper Python):
///   sigma, psi, omega, gamma, zeta, maat: valores das casas
///   phi_iit: phi_iit_normalized
///   phi_iit_nats: phi_iit_nats (para log10)
///   phi_trans: phi_transcendent (para log10)
///   snapshot_ts: timestamp do snapshot
///   age_s: idade em segundos (time.time() - snapshot_ts)
///   recovery: recovery_capacity_index
///   crisis: crisis_pressure_index
///   phi_operational_support: valor pré-computado ou 0.0
///   has_topology, has_phi_frameworks, has_temporal_scale, has_sigma_source: flags
///   writer_role: string do role
///
/// Returns:
///   Score float (quanto maior, melhor o snapshot)
#[pyfunction]
pub fn score_live_writer_snapshot(
    sigma: f64,
    psi: f64,
    omega: f64,
    gamma: f64,
    zeta: f64,
    maat: f64,
    phi_iit: f64,
    phi_iit_nats: f64,
    phi_trans: f64,
    age_s: f64,
    recovery: f64,
    crisis: f64,
    phi_operational_support_input: f64,
    has_topology: bool,
    has_phi_frameworks: bool,
    has_temporal_scale: bool,
    has_sigma_source: bool,
    writer_role: &str,
) -> f64 {
    // === phi_operational_support (symmetric MaxEnt domain-normalized fallback) ===
    let phi_operational_support: f64 = if phi_operational_support_input > 0.0 {
        phi_operational_support_input
    } else {
        let log10_ceil = crate::phi_constants::phi_control_log10_ceiling();
        let mut active: Vec<f64> = Vec::with_capacity(3);
        if phi_iit.is_finite() && phi_iit > 0.0 {
            active.push(phi_iit.clamp(0.0, 1.0));
        }
        if phi_iit_nats.is_finite() && phi_iit_nats > 0.0 {
            active.push(((1.0 + phi_iit_nats).log10() / log10_ceil).clamp(0.0, 1.0));
        }
        if phi_trans.is_finite() && phi_trans > 0.0 {
            active.push(((1.0 + phi_trans).log10() / log10_ceil).clamp(0.0, 1.0));
        }
        if active.is_empty() {
            0.0
        } else {
            (active.iter().sum::<f64>() / (active.len() as f64)).clamp(0.0, 1.0)
        }
    };

    // === Richness ===
    let mut richness: f64 = 0.0;
    if has_topology {
        richness += 0.12;
    }
    if has_phi_frameworks {
        richness += 0.12;
    }
    if has_temporal_scale {
        richness += 0.12;
    }
    if has_sigma_source {
        richness += 0.08;
    }

    // === Default axes detection ===
    let mut default_axes: i32 = 0;
    if (psi - 0.5).abs() <= 0.02 {
        default_axes += 1;
    }
    if (omega - 0.5).abs() <= 0.02 {
        default_axes += 1;
    }
    if (gamma - 0.6).abs() <= 0.02 {
        default_axes += 1;
    }
    if (zeta - 0.1).abs() <= 0.02 {
        default_axes += 1;
    }

    // === Role bias ===
    let role_bias: f64 = match writer_role {
        "federated_fallback" => -0.04,
        "autonomous_executor" => 0.04,
        "sovereign_primary" => 0.08,
        _ => 0.0,
    };

    // === Maat penalty ===
    let maat_penalty: f64 = if maat > 0.05 { 0.0 } else { 0.14 };

    // === Freshness bonus ===
    let freshness_bonus: f64 = if age_s <= 300.0 {
        0.14
    } else if age_s <= 900.0 {
        0.08
    } else if age_s <= 3600.0 {
        0.02
    } else {
        -0.22
    };

    // === Freshness penalty ===
    let freshness_penalty: f64 = if age_s > 480.0 {
        (((age_s - 480.0) / 480.0) * 0.18).min(0.35)
    } else {
        0.0
    };

    // === Final score ===
    // Onda 3A (R1-A11): clamp recovery to [0, 1] consistently and normalize the 5 base
    // health weights (sum = 0.96 -> 1.0) while preserving relative weight ratios.
    const BASE_HEALTH_WEIGHT_SUM: f64 = 0.96;
    let base_health = (phi_operational_support.clamp(0.0, 1.0) * 0.18
        + sigma.clamp(0.0, 1.0) * 0.22
        + maat.clamp(0.0, 1.0) * 0.16
        + recovery.clamp(0.0, 1.0) * 0.26
        + (1.0 - crisis.clamp(0.0, 1.0)) * 0.14)
        / BASE_HEALTH_WEIGHT_SUM;

    base_health
        + richness
        - (default_axes as f64) * 0.03
        + role_bias
        + freshness_bonus
        - maat_penalty
        - freshness_penalty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_score() {
        let score = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.5, 0.0, 0.0,
            100.0, 0.8, 0.2,
            0.5,
            true, true, true, true,
            "sovereign_primary",
        );
        // Should be a positive score
        assert!(score > 0.0);
    }

    #[test]
    fn test_fresh_snapshot_higher() {
        let fresh = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.5, 0.0, 0.0,
            50.0, 0.8, 0.2, 0.5,
            true, true, true, true, "sovereign_primary",
        );
        let stale = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.5, 0.0, 0.0,
            5000.0, 0.8, 0.2, 0.5,
            true, true, true, true, "sovereign_primary",
        );
        assert!(fresh > stale);
    }

    #[test]
    fn test_sovereign_primary_bonus() {
        let sovereign = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.5, 0.0, 0.0,
            100.0, 0.8, 0.2, 0.5,
            true, true, true, true, "sovereign_primary",
        );
        let fallback = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.5, 0.0, 0.0,
            100.0, 0.8, 0.2, 0.5,
            true, true, true, true, "federated_fallback",
        );
        // sovereign_primary gets +0.08, federated_fallback gets -0.04 → diff 0.12
        assert!((sovereign - fallback - 0.12).abs() < 1e-15);
    }

    #[test]
    fn test_maat_penalty() {
        let good_maat = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.5,
            0.5, 0.0, 0.0,
            100.0, 0.8, 0.2, 0.5,
            true, true, true, true, "auxiliary_writer",
        );
        let bad_maat = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.01,
            0.5, 0.0, 0.0,
            100.0, 0.8, 0.2, 0.5,
            true, true, true, true, "auxiliary_writer",
        );
        // Diferença = (0.5-0.01)*(0.16/0.96) (maat contribution) + 0.14 (penalty)
        let expected_diff = (0.5_f64 - 0.01) * (0.16 / 0.96) + 0.14;
        assert!((good_maat - bad_maat - expected_diff).abs() < 1e-15);
    }

    #[test]
    fn test_recovery_clamped_and_base_weights_sum_to_one() {
        let clamped = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1,
            1.0,  // maat
            0.0, 0.0, 0.0,
            100.0, // age_s -> +0.14 freshness
            1.5,   // recovery > 1.0 should clamp to 1.0
            0.0,   // crisis = 0.0 -> inv_crisis = 1.0
            1.0,   // phi_operational_support = 1.0
            true, true, true, true, // richness = 0.44
            "auxiliary_writer",
        );
        let at_one = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1,
            1.0,
            0.0, 0.0, 0.0,
            100.0,
            1.0,
            0.0,
            1.0,
            true, true, true, true,
            "auxiliary_writer",
        );
        assert!((clamped - at_one).abs() < 1e-15);

        let negative_clamped = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1,
            0.0, 0.0, 0.0, 0.0,
            100.0,
            -0.5, // recovery < 0.0 should clamp to 0.0
            1.0,
            0.0,
            false, false, false, false,
            "auxiliary_writer",
        );
        let at_zero = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1,
            0.0, 0.0, 0.0, 0.0,
            100.0,
            0.0,
            1.0,
            0.0,
            false, false, false, false,
            "auxiliary_writer",
        );
        assert!((negative_clamped - at_zero).abs() < 1e-15);
        // If sigma = 1.0 and phi = 1.0 and maat = 1.0 and recovery = 1.0 and crisis = 0.0:
        // base_health == 1.0 exactly (weights sum to 1.0 after / 0.96).
        let max_base = score_live_writer_snapshot(
            1.0, 0.5, 0.5, 0.6, 0.1,
            1.0, 0.0, 0.0, 0.0,
            2000.0, // age_s in (900, 3600] -> +0.02 freshness, -0.35 freshness_penalty
            1.0, 0.0, 1.0,
            false, false, false, false, "auxiliary_writer",
        );
        // base_health = 1.0, default_axes = 4 -> -0.12, freshness = +0.02 - 0.35
        let expected = 1.0 - 4.0 * 0.03 + 0.02 - min_penalty_check(2000.0);
        assert!((max_base - expected).abs() < 1e-14);
    }

    fn min_penalty_check(age_s: f64) -> f64 {
        if age_s > 480.0 {
            (((age_s - 480.0) / 480.0) * 0.18).min(0.35)
        } else {
            0.0
        }
    }

    #[test]
    fn test_phi_silicon_fallback() {
        // phi_operational_support_input = 0.0 → symmetric MaxEnt domain-normalized fallback
        let score = score_live_writer_snapshot(
            0.5, 0.5, 0.5, 0.6, 0.1, 0.3,
            0.0, 1e6, 0.0,  // phi_iit=0, phi_iit_nats=1e6
            100.0, 0.8, 0.2,
            0.0,  // phi_operational_support = 0 → fallback
            false, false, false, false, "auxiliary_writer",
        );
        assert!(score > 0.0);
    }
}
