use wasm_bindgen::prelude::*;


#[derive(Debug)]

pub enum Error {
    Invalid(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;


#[wasm_bindgen]
pub fn heavy_calc(x: f64) -> f64 {
    let mut sum = 0.0f64;
    for i in 0..1_000_000 {
        let a = 2.0 * (x + i as f64);
        sum += 0.5 * a.sin();
    }
    sum
}

#[wasm_bindgen]
pub fn local_time(cycles: f64, freq_hz: f64) -> f64 {
    cycles / freq_hz
}

#[wasm_bindgen]
pub fn local_energy(cycles: f64, freq_hz: f64, kappa: f64) -> f64 {
    kappa * freq_hz.powi(2) * cycles
}

#[wasm_bindgen]
pub fn optimal_frequency(alpha: f64, beta: f64, e_local_j: f64, kappa: f64, t_ref_s: f64) -> f64 {
    let denom = 2.0 * beta * kappa * t_ref_s;
    (alpha * e_local_j / denom).cbrt()
}

#[wasm_bindgen]
pub fn snr_linear(p_watt: f64, channel_gain: f64, noise_watt: f64) -> f64 {
    p_watt * channel_gain / noise_watt
}

#[wasm_bindgen]
pub fn link_rate_bps(bandwidth_hz: f64, snr_linear_val: f64, subchannels_v: f64) -> f64 {
    (bandwidth_hz / subchannels_v) * (1.0 + snr_linear_val).log2()
}

#[wasm_bindgen]
pub fn link_rate_simplified_bps(bandwidth_hz: f64, spectral_eff_bps_per_hz: f64) -> f64 {
    bandwidth_hz * spectral_eff_bps_per_hz
}

#[wasm_bindgen]
pub fn tx_time_s(data_bits: f64, rate_bps: f64) -> f64 {
    data_bits / rate_bps
}

#[wasm_bindgen]
pub fn tx_energy_j(p_watt: f64, t_tx_s: f64) -> f64 {
    p_watt * t_tx_s
}

#[wasm_bindgen]
pub fn remote_compute_time_s(cycles: f64, dest_freq_hz: f64) -> f64 {
    cycles / dest_freq_hz
}

#[wasm_bindgen]
pub fn efficiency_score(alpha: f64, beta: f64, t_ref_s: f64, t_off_s: f64, e_local_j: f64, e_off_j: f64) -> f64 {
    let time_term = (t_ref_s - t_off_s) / t_ref_s;
    let energy_term = (e_local_j - e_off_j) / e_local_j;
    let q = alpha * time_term + beta * energy_term;
    q.clamp(-1.0, 1.0)
}

#[wasm_bindgen]
pub fn power_for_target_rate_w(
    target_rate_bps: f64,
    bandwidth_hz: f64,
    subchannels_v: f64,
    channel_gain: f64,
    noise_watt: f64
) -> f64 {
    let exp_arg = target_rate_bps / (bandwidth_hz / subchannels_v);
    let snr_req = 2f64.powf(exp_arg) - 1.0;
    (noise_watt / channel_gain) * snr_req.max(0.0)
}

#[wasm_bindgen]
pub fn total_offload_time_s(data_bits: f64, rate_bps: f64, cycles: f64, dest_freq_hz: f64) -> f64 {
    tx_time_s(data_bits, rate_bps) + remote_compute_time_s(cycles, dest_freq_hz)
}

#[wasm_bindgen]
pub fn total_offload_energy_j(p_watt: f64, data_bits: f64, rate_bps: f64) -> f64 {
    let ttx = tx_time_s(data_bits, rate_bps);
    tx_energy_j(p_watt, ttx)
}
