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
        let space = self.params.space.clamp(0.0, 1.0);
        let wet = self.params.wet.clamp(0.0, 1.0);
        // Speakers: keep rotation milder so it doesn't cancel in the room.
        let speaker_scale = if self.mode == 1 { 0.55 } else { 1.0 };
        let depth = depth * speaker_scale;
        let space = space * if self.mode == 1 { 0.65 } else { 1.0 };

        let phase_inc = TAU * speed / self.sample_rate;
        let max_itd = (self.sample_rate * MAX_ITD_MS * 0.001).clamp(1.0, (DELAY_LEN - 2) as f32);
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

            // Equal-power rotating mono image + retained stereo body.
            let mono = (dry_l + dry_r) * 0.5;
            let side = (dry_l - dry_r) * 0.5;
            let body_l = mono + side * (1.0 - 0.35 * depth);
            let body_r = mono - side * (1.0 - 0.35 * depth);

            let orbit_l = mono * (0.5 + 0.5 * cos_a);
            let orbit_r = mono * (0.5 + 0.5 * sin_a);

            // ITD: delay the far ear.
            let itd = sin_a * max_itd * depth;
            let delay_l_samp = if itd > 0.0 { itd } else { 0.0 };
            let delay_r_samp = if itd < 0.0 { -itd } else { 0.0 };

            let wet_l0 = body_l * (1.0 - depth) + orbit_l * depth;
            let wet_r0 = body_r * (1.0 - depth) + orbit_r * depth;

            self.delay_l[self.delay_pos] = wet_l0;
            self.delay_r[self.delay_pos] = wet_r0;

            let read_l = Self::read_delay(&self.delay_l, self.delay_pos, delay_l_samp);
            let read_r = Self::read_delay(&self.delay_r, self.delay_pos, delay_r_samp);
            self.delay_pos = (self.delay_pos + 1) % DELAY_LEN;

            // Far-side darkening for headphone externalization.
            let far_l = (1.0 - cos_a).clamp(0.0, 1.0); // darker when source is more to the right/back
            let far_r = (1.0 + cos_a).clamp(0.0, 1.0);
            // Use sin for left/right cue: positive sin => right, darken left.
            let dark_l = (sin_a.max(0.0) * depth).clamp(0.0, 1.0);
            let dark_r = ((-sin_a).max(0.0) * depth).clamp(0.0, 1.0);
            let _ = (far_l, far_r);

            self.lp_l += lp_coeff * (read_l - self.lp_l);
            self.lp_r += lp_coeff * (read_r - self.lp_r);
            let shaped_l = read_l * (1.0 - 0.55 * dark_l) + self.lp_l * (0.55 * dark_l);
            let shaped_r = read_r * (1.0 - 0.55 * dark_r) + self.lp_r * (0.55 * dark_r);

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
