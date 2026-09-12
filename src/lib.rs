mod abi;
mod dsp;

use abi::{EchoDspApi, EchoDspConfig, EchoDspInfo, PROVIDER_ABI_VERSION};
use dsp::{Params, Processor};
use serde::Deserialize;
use serde_json::{json, Value};
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::ptr;

const PROVIDER_ID: &str = "echo-8d-dsp";
const PROVIDER_VERSION: &str = "1.1.0";

const MANIFEST_JSON: &str = r#"{
  "schemaVersion": 1,
  "displayName": "动态 8D 环绕",
  "description": "实时旋转声像与双耳延迟。耳机预设偏绕头；扬声器预设偏左右扫动、弱双耳延迟。",
  "vendor": "local",
  "resources": [],
  "presets": [
    {
      "id": "classic-8d",
      "label": "经典 8D",
      "description": "适中转速与深度，最接近常见 8D 听感",
      "recommendedDevice": "headphone",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 8,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 82,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 42,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 92,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "slow-orbit",
      "label": "慢速环绕",
      "description": "更慢的环绕，适合流行与人声",
      "recommendedDevice": "headphone",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 14,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 70,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 38,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 88,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "fast-spin",
      "label": "快速旋转",
      "description": "更明显的绕头运动",
      "recommendedDevice": "headphone",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 4,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 95,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 30,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 95,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "binaural-swirl",
      "label": "双耳旋涡",
      "description": "更强 ITD/空间感，偏沉浸",
      "recommendedDevice": "headphone",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 10,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 88,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 58,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 90,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "speaker-sweep",
      "label": "音箱左右扫",
      "description": "扬声器专用：大幅左右扫动，弱双耳延迟，适合音响",
      "recommendedDevice": "speaker",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 7,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 92,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 28,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 94,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "speaker-wide",
      "label": "音箱宽扫",
      "description": "扬声器专用：更慢更宽的左右移动，混响更少",
      "recommendedDevice": "speaker",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 12,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 98,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 18,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 96,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    },
    {
      "id": "speaker-pingpong",
      "label": "音箱快切",
      "description": "扬声器专用：更快的左右来回，接近乒乓扫动",
      "recommendedDevice": "speaker",
      "controls": [
        {
          "id": "period",
          "type": "number",
          "label": "旋转周期",
          "defaultValue": 3.5,
          "unit": "秒",
          "range": { "min": 2, "max": 30, "step": 0.5, "minLabel": "快", "maxLabel": "慢" },
          "ownership": "provider"
        },
        {
          "id": "depth",
          "type": "number",
          "label": "环绕深度",
          "defaultValue": 100,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "space",
          "type": "number",
          "label": "空间混响",
          "defaultValue": 12,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        },
        {
          "id": "wet",
          "type": "number",
          "label": "效果比例",
          "defaultValue": 97,
          "unit": "%",
          "range": { "min": 0, "max": 100, "step": 1 },
          "ownership": "provider"
        }
      ]
    }
  ],
  "controls": []
}"#;

#[derive(Debug, Deserialize)]
struct PresetRequest {
    #[serde(default)]
    #[serde(rename = "presetId")]
    preset_id: String,
    #[serde(default)]
    controls: Value,
}

struct Engine {
    processor: Processor,
    preset_id: String,
    preset_label: String,
    provider_id: CString,
    provider_version: CString,
    manifest_json: CString,
    state_json: CString,
}

impl Engine {
    fn create(config: &EchoDspConfig) -> Option<Box<Self>> {
        if config.abi_version != 0 && config.abi_version != PROVIDER_ABI_VERSION {
            return None;
        }
        let sample_rate = if config.sample_rate == 0 {
            48000
        } else {
            config.sample_rate
        };
        let channels = if config.channels == 0 {
            2
        } else {
            config.channels
        };
        if channels > 8 {
            return None;
        }

        let mut engine = Box::new(Self {
            processor: Processor::new(sample_rate, channels, config.mode, Params::classic()),
            preset_id: "classic-8d".to_string(),
            preset_label: "经典 8D".to_string(),
            provider_id: CString::new(PROVIDER_ID).ok()?,
            provider_version: CString::new(PROVIDER_VERSION).ok()?,
            manifest_json: CString::new(MANIFEST_JSON).ok()?,
            state_json: CString::new("{}").ok()?,
        });

        let preset = optional_cstr(config.preset_json);
        if let Some(preset) = preset {
            if engine.apply_preset_json(preset).is_err() {
                // Keep defaults if initial preset is empty/invalid.
                let _ = engine.apply_preset_json(r#"{"presetId":"classic-8d"}"#);
            }
        } else {
            let _ = engine.apply_preset_json(r#"{"presetId":"classic-8d"}"#);
        }
        engine.refresh_state_json();
        Some(engine)
    }

    fn apply_preset_json(&mut self, raw: &str) -> Result<(), String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        let request: PresetRequest =
            serde_json::from_str(trimmed).map_err(|e| format!("invalid preset JSON: {e}"))?;
        let preset_id = if request.preset_id.trim().is_empty() {
            self.preset_id.clone()
        } else {
            request.preset_id.trim().to_string()
        };

        let (label, mut params) = defaults_for_preset(&preset_id)?;
        overlay_controls(&mut params, &request.controls)?;
        if !(0.0..=1.0).contains(&params.depth)
            || !(0.0..=1.0).contains(&params.space)
            || !(0.0..=1.0).contains(&params.wet)
            || !(0.01..=1.5).contains(&params.speed_hz)
        {
            return Err("control out of range".to_string());
        }

        self.preset_id = preset_id;
        self.preset_label = label.to_string();
        self.processor.set_params(params);
        self.refresh_state_json();
        Ok(())
    }

    fn refresh_state_json(&mut self) {
        let p = self.processor.params();
        let period = (1.0 / p.speed_hz.max(1e-6)).clamp(2.0, 30.0);
        let state = json!({
            "schemaVersion": 1,
            "effect": { "id": self.preset_id, "name": self.preset_label },
            "presetId": self.preset_id,
            "latencyFrames": 0,
            "controls": {
                "period": { "value": round1(period), "ownership": "provider" },
                "depth": { "value": round0(p.depth * 100.0), "ownership": "provider" },
                "space": { "value": round0(p.space * 100.0), "ownership": "provider" },
                "wet": { "value": round0(p.wet * 100.0), "ownership": "provider" }
            }
        });
        self.state_json = CString::new(state.to_string()).unwrap_or_else(|_| CString::new("{}").unwrap());
    }

    fn fill_info(&self, info: &mut EchoDspInfo) {
        info.abi_version = PROVIDER_ABI_VERSION;
        info.latency_frames = 0;
        info.preferred_block_frames = 512;
        info.max_channels = 8;
        info.provider_id = self.provider_id.as_ptr();
        info.provider_version = self.provider_version.as_ptr();
        info.manifest_json = self.manifest_json.as_ptr();
        info.state_json = self.state_json.as_ptr();
    }
}

fn defaults_for_preset(id: &str) -> Result<(&'static str, Params), String> {
    match id {
        "classic-8d" => Ok((
            "经典 8D",
            Params {
                speed_hz: 1.0 / 8.0,
                depth: 0.82,
                space: 0.42,
                wet: 0.92,
            },
        )),
        "slow-orbit" => Ok((
            "慢速环绕",
            Params {
                speed_hz: 1.0 / 14.0,
                depth: 0.70,
                space: 0.38,
                wet: 0.88,
            },
        )),
        "fast-spin" => Ok((
            "快速旋转",
            Params {
                speed_hz: 1.0 / 4.0,
                depth: 0.95,
                space: 0.30,
                wet: 0.95,
            },
        )),
        "binaural-swirl" => Ok((
            "双耳旋涡",
            Params {
                speed_hz: 1.0 / 10.0,
                depth: 0.88,
                space: 0.58,
                wet: 0.90,
            },
        )),
        "speaker-sweep" => Ok((
            "音箱左右扫",
            Params {
                speed_hz: 1.0 / 7.0,
                depth: 0.92,
                space: 0.28,
                wet: 0.94,
            },
        )),
        "speaker-wide" => Ok((
            "音箱宽扫",
            Params {
                speed_hz: 1.0 / 12.0,
                depth: 0.98,
                space: 0.18,
                wet: 0.96,
            },
        )),
        "speaker-pingpong" => Ok((
            "音箱快切",
            Params {
                speed_hz: 1.0 / 3.5,
                depth: 1.0,
                space: 0.12,
                wet: 0.97,
            },
        )),
        _ => Err(format!("unknown preset: {id}")),
    }
}

fn overlay_controls(params: &mut Params, controls: &Value) -> Result<(), String> {
    let Some(map) = controls.as_object() else {
        return Ok(());
    };
    for (key, entry) in map {
        let value = control_value(entry)?;
        match key.as_str() {
            "period" => {
                let period = value.as_f64().ok_or("period must be number")? as f32;
                if !(2.0..=30.0).contains(&period) {
                    return Err("period out of range".into());
                }
                params.speed_hz = 1.0 / period;
            }
            "depth" => {
                let pct = value.as_f64().ok_or("depth must be number")? as f32;
                if !(0.0..=100.0).contains(&pct) {
                    return Err("depth out of range".into());
                }
                params.depth = pct / 100.0;
            }
            "space" => {
                let pct = value.as_f64().ok_or("space must be number")? as f32;
                if !(0.0..=100.0).contains(&pct) {
                    return Err("space out of range".into());
                }
                params.space = pct / 100.0;
            }
            "wet" => {
                let pct = value.as_f64().ok_or("wet must be number")? as f32;
                if !(0.0..=100.0).contains(&pct) {
                    return Err("wet out of range".into());
                }
                params.wet = pct / 100.0;
            }
            _ => return Err(format!("unknown control: {key}")),
        }
    }
    Ok(())
}

fn control_value(entry: &Value) -> Result<&Value, String> {
    if let Some(obj) = entry.as_object() {
        if let Some(value) = obj.get("value") {
            return Ok(value);
        }
    }
    Ok(entry)
}

fn optional_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

fn round0(v: f32) -> i64 {
    v.round() as i64
}

fn round1(v: f32) -> f64 {
    (v * 10.0).round() as f64 / 10.0
}

unsafe extern "C" fn create(config: *const EchoDspConfig, info: *mut EchoDspInfo) -> *mut c_void {
    if config.is_null() || info.is_null() {
        return ptr::null_mut();
    }
    let config = &*config;
    let Some(engine) = Engine::create(config) else {
        return ptr::null_mut();
    };
    engine.fill_info(&mut *info);
    Box::into_raw(engine) as *mut c_void
}

unsafe extern "C" fn process(
    instance: *mut c_void,
    buffer: *mut f32,
    frames: u32,
    channels: u32,
) -> i32 {
    if instance.is_null() || buffer.is_null() || frames == 0 || channels == 0 {
        return -1;
    }
    let engine = &mut *(instance as *mut Engine);
    let len = (frames as usize).saturating_mul(channels as usize);
    let data = std::slice::from_raw_parts_mut(buffer, len);
    engine
        .processor
        .process_interleaved(data, frames as usize, channels);
    0
}

unsafe extern "C" fn drain(
    instance: *mut c_void,
    _buffer: *mut f32,
    _capacity_frames: u32,
    written_frames: *mut u32,
) -> i32 {
    if instance.is_null() {
        return -1;
    }
    if !written_frames.is_null() {
        *written_frames = 0;
    }
    0
}

unsafe extern "C" fn reset(instance: *mut c_void) -> i32 {
    if instance.is_null() {
        return -1;
    }
    let engine = &mut *(instance as *mut Engine);
    engine.processor.reset();
    0
}

unsafe extern "C" fn configure(instance: *mut c_void, preset_json: *const c_char) -> i32 {
    if instance.is_null() {
        return -1;
    }
    let engine = &mut *(instance as *mut Engine);
    let raw = optional_cstr(preset_json).unwrap_or("");
    match engine.apply_preset_json(raw) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

unsafe extern "C" fn get_state_json(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return ptr::null();
    }
    let engine = &*(instance as *mut Engine);
    engine.state_json.as_ptr()
}

unsafe extern "C" fn destroy(instance: *mut c_void) {
    if instance.is_null() {
        return;
    }
    drop(Box::from_raw(instance as *mut Engine));
}

static API: EchoDspApi = EchoDspApi {
    abi_version: PROVIDER_ABI_VERSION,
    create: Some(create),
    process: Some(process),
    drain: Some(drain),
    reset: Some(reset),
    configure: Some(configure),
    get_state_json: Some(get_state_json),
    destroy: Some(destroy),
};

#[no_mangle]
pub extern "C" fn echo_dsp_get_api() -> *const EchoDspApi {
    &API
}
