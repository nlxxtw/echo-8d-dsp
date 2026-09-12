//! Realtime rotating 8D / binaural swirl processor.
//!
//! `process` must not allocate. Parameter changes happen in `configure`.

use std::f32::consts::TAU;

const DELAY_LEN: usize = 256;
const COMB_LEN: usize = 2048;
const MAX_ITD_MS: f32 = 0.75;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Full rotations per second.
    pub speed_hz: f32,
    /// 0..1 pan / ITD intensity.
    pub depth: f32,
    /// 0..1 early space / reverb.
    pub space: f32,
    /// 0..1 wet mix.
    pub wet: f32,
}

impl Params {
    pub const fn classic() -> Self {
        Self {
            speed_hz: 0.12,
            depth: 0.82,
            space: 0.42,
            wet: 0.92,
        }
    }
}

pub struct Processor {
    sample_rate: f32,
    mode: u32,
    phase: f32,
    params: Params,
    delay_l: [f32; DELAY_LEN],
    delay_r: [f32; DELAY_LEN],
    delay_pos: usize,
    comb: [[f32; COMB_LEN]; 4],
    comb_pos: [usize; 4],
    lp_l: f32,
    lp_r: f32,
    limiter_env: f32,
}

impl Processor {
    pub fn new(sample_rate: u32, channels: u32, mode: u32, params: Params) -> Self {
        let _ = channels;
        Self {
            sample_rate: sample_rate.max(1) as f32,
            mode,
            phase: 0.0,
            params,
            delay_l: [0.0; DELAY_LEN],
            delay_r: [0.0; DELAY_LEN],
            delay_pos: 0,
            comb: [[0.0; COMB_LEN]; 4],
            comb_pos: [0; 4],
            lp_l: 0.0,
            lp_r: 0.0,
            limiter_env: 0.0,
        }
    }

    pub fn set_params(&mut self, params: Params) {
        self.params = params;
    }

    pub fn params(&self) -> Params {
        self.params
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.delay_l.fill(0.0);
        self.delay_r.fill(0.0);
        self.delay_pos = 0;
        for buf in &mut self.comb {
            buf.fill(0.0);
        }
        self.comb_pos = [0; 4];
        self.lp_l = 0.0;
        self.lp_r = 0.0;
        self.limiter_env = 0.0;
    }

    pub fn process_interleaved(&mut self, data: &mut [f32], frames: usize, channels: u32) {
        let ch = channels.max(1) as usize;
        if data.len() < frames * ch {
            return;
        }

        let speed = self.params.speed_hz.clamp(0.01, 1.5);
        let depth = self.params.depth.clamp(0.0, 1.0);
        let space_in = self.params.space.clamp(0.0, 1.0);
        let wet = self.params.wet.clamp(0.0, 1.0);
        let speaker = self.mode == 1;

        // Speakers: wide L/R amplitude sweep, weak ITD (room crosstalk kills binaural cues).
        // Headphones: full orbit + ITD + far-ear darkening.
        let space = if speaker { space_in * 0.85 } else { space_in };
        let itd_scale = if speaker { 0.18 } else { 1.0 };
        let dark_scale = if speaker { 0.12 } else { 0.55 };
        // Speaker pan emphasis: push orbit harder so L/R travel is obvious on boxes.
        let orbit_mix = if speaker {
            (depth * 1.15).clamp(0.0, 1.0)
        } else {
            depth
        };

        let phase_inc = TAU * speed / self.sample_rate;
        let max_itd =
            (self.sample_rate * MAX_ITD_MS * 0.001 * itd_scale).clamp(0.0, (DELAY_LEN - 2) as f32);
        let lp_coeff = 1.0 - (-2.0 * std::f32::consts::PI * 4200.0 / self.sample_rate).exp();

        for frame in 0..frames {
            let base = frame * ch;
            let dry_l = data[base];
            let dry_r = if ch > 1 { data[base + 1] } else { dry_l };

            let angle = self.phase;
            self.phase = (self.phase + phase_inc) % TAU;

            // Continuous orbit: front->right->back->left.
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let mono = (dry_l + dry_r) * 0.5;
            let side = (dry_l - dry_r) * 0.5;

            let (wet_l0, wet_r0) = if speaker {
                // Equal-power hard-ish L/R sweep for loudspeakers (sin/cos pair).
                // Keep a little stereo body so it doesn't collapse to mono ping-pong.
                let body_keep = (1.0 - 0.72 * orbit_mix).clamp(0.15, 1.0);
                let body_l = mono + side * body_keep;
                let body_r = mono - side * body_keep;
                // Constant-power pan: full left when cos≈1, full right when sin≈1.
                let g_l = (0.5 + 0.5 * cos_a).sqrt();
                let g_r = (0.5 + 0.5 * sin_a).sqrt();
                let orbit_l = mono * g_l;
                let orbit_r = mono * g_r;
                (
                    body_l * (1.0 - orbit_mix) + orbit_l * orbit_mix,
                    body_r * (1.0 - orbit_mix) + orbit_r * orbit_mix,
                )
            } else {
                let body_l = mono + side * (1.0 - 0.35 * depth);
                let body_r = mono - side * (1.0 - 0.35 * depth);
                let orbit_l = mono * (0.5 + 0.5 * cos_a);
                let orbit_r = mono * (0.5 + 0.5 * sin_a);
                (
                    body_l * (1.0 - depth) + orbit_l * depth,
                    body_r * (1.0 - depth) + orbit_r * depth,
                )
            };

            // ITD: delay the far side (weak on speakers).
            let itd = sin_a * max_itd * depth;
            let delay_l_samp = if itd > 0.0 { itd } else { 0.0 };
            let delay_r_samp = if itd < 0.0 { -itd } else { 0.0 };

            self.delay_l[self.delay_pos] = wet_l0;
            self.delay_r[self.delay_pos] = wet_r0;

            let read_l = Self::read_delay(&self.delay_l, self.delay_pos, delay_l_samp);
            let read_r = Self::read_delay(&self.delay_r, self.delay_pos, delay_r_samp);
            self.delay_pos = (self.delay_pos + 1) % DELAY_LEN;

            // Far-side darkening (headphone externalization; mild on speakers).
            let dark_l = (sin_a.max(0.0) * depth).clamp(0.0, 1.0);
            let dark_r = ((-sin_a).max(0.0) * depth).clamp(0.0, 1.0);

            self.lp_l += lp_coeff * (read_l - self.lp_l);
            self.lp_r += lp_coeff * (read_r - self.lp_r);
            let shaped_l = read_l * (1.0 - dark_scale * dark_l) + self.lp_l * (dark_scale * dark_l);
            let shaped_r = read_r * (1.0 - dark_scale * dark_r) + self.lp_r * (dark_scale * dark_r);

            let (space_l, space_r) = self.tick_space(shaped_l, shaped_r, space);
            let mut out_l = dry_l * (1.0 - wet) + (shaped_l + space_l) * wet;
            let mut out_r = dry_r * (1.0 - wet) + (shaped_r + space_r) * wet;

            // Stereo-linked soft limiter.
            let peak = out_l.abs().max(out_r.abs());
            let target = if peak > 0.98 { 0.98 / peak } else { 1.0 };
            self.limiter_env = if target < self.limiter_env {
                target
            } else {
                self.limiter_env + 0.002 * (target - self.limiter_env)
            };
            out_l *= self.limiter_env;
            out_r *= self.limiter_env;

            data[base] = sanitize(out_l);
            if ch > 1 {
                data[base + 1] = sanitize(out_r);
            }
            // Leave extra channels untouched.
        }
    }

    fn read_delay(buf: &[f32; DELAY_LEN], write_pos: usize, delay: f32) -> f32 {
        let delay = delay.clamp(0.0, (DELAY_LEN - 2) as f32);
        let base = delay.floor() as usize;
        let frac = delay - base as f32;
        let i1 = (write_pos + DELAY_LEN - base) % DELAY_LEN;
        let i2 = (write_pos + DELAY_LEN - base - 1) % DELAY_LEN;
        buf[i1] * (1.0 - frac) + buf[i2] * frac
    }

    fn tick_space(&mut self, l: f32, r: f32, amount: f32) -> (f32, f32) {
        if amount <= 0.0001 {
            return (0.0, 0.0);
        }
        // Four comb filters with fixed prime-ish delays.
        const DELAYS: [usize; 4] = [1553, 1613, 1493, 1663];
        const FEEDBACK: [f32; 4] = [0.72, 0.69, 0.74, 0.67];
        let mut acc_l = 0.0;
        let mut acc_r = 0.0;
        for i in 0..4 {
            let len = DELAYS[i].min(COMB_LEN - 1);
            let pos = self.comb_pos[i] % len;
            let delayed = self.comb[i][pos];
            let input = if i % 2 == 0 { l } else { r };
            let next = input + delayed * FEEDBACK[i];
            self.comb[i][pos] = next;
            self.comb_pos[i] = (pos + 1) % len;
            if i % 2 == 0 {
                acc_l += delayed;
            } else {
                acc_r += delayed;
            }
        }
        let scale = 0.22 * amount;
        (acc_l * scale, acc_r * scale)
    }
}

fn sanitize(x: f32) -> f32 {
    if x.is_finite() {
        x.clamp(-1.0, 1.0)
    } else {
        0.0
    }
}
