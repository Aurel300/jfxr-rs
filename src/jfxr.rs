use crate::sound::Sound;

/// Error encountered while parsing a `jfxr` sound.
#[derive(Debug, PartialEq, Eq)]
pub enum JfxrFormatError {
    /// Error related to JSON parsing.
    Json(json::Error),

    /// The given JSON was not an object, as expected for a `jfxr` sound.
    NotAnObject,

    /// A field was expected, but not found.
    MissingField(&'static str),

    /// A field had an invalid value or wrong type.
    InvalidField(&'static str),

    /// The file was encoded with a newer version of `jfxr`.
    UnsupportedVersion,
}

impl From<json::Error> for JfxrFormatError {
    fn from(value: json::Error) -> Self {
        Self::Json(value)
    }
}

/// This is the version written out to sound files. We maintain backwards
/// compatibility with files written by older versions where possible, but
/// refuse to read files written by newer versions. Only bump the version
/// number if older versions of `jfxr` would be unable to correctly interpret
/// files written by this version.
pub const VERSION: u32 = 1;

/// Parses a string as a `jfxr` file and outputs the parsed [`Sound`], if
/// successful.
pub fn read_jfxr(jfxr: &str) -> Result<Sound, JfxrFormatError> {
    let json = match json::parse(jfxr)? {
        json::JsonValue::Object(o) => o,
        _ => return Err(JfxrFormatError::NotAnObject),
    };
    macro_rules! read_field {
        ($name:literal, $get:ident) => {
            json.get($name)
                .ok_or(JfxrFormatError::MissingField($name))?
                .$get()
                .ok_or(JfxrFormatError::InvalidField($name))?
        };
    }
    macro_rules! read_param {
        ($ty:ident, $name:literal, $get:ident) => {
            crate::parameter::$ty(read_field!($name, $get))
        };
    }
    let version = read_field!("_version", as_u32);
    if version > VERSION {
        return Err(JfxrFormatError::UnsupportedVersion);
    }
    let name = read_field!("_name", as_str).to_string();
    // TODO: _locked field
    Ok(Sound {
        name,

        sample_rate: read_param!(SampleRate, "sampleRate", as_f64),
        attack: read_param!(Attack, "attack", as_f64),
        sustain: read_param!(Sustain, "sustain", as_f64),
        sustain_punch: read_param!(SustainPunch, "sustainPunch", as_f64),
        decay: read_param!(Decay, "decay", as_f64),
        tremolo_depth: read_param!(TremoloDepth, "tremoloDepth", as_f64),
        tremolo_frequency: read_param!(TremoloFrequency, "tremoloFrequency", as_f64),
        frequency: read_param!(Frequency, "frequency", as_f64),
        frequency_sweep: read_param!(FrequencySweep, "frequencySweep", as_f64),
        frequency_delta_sweep: read_param!(FrequencyDeltaSweep, "frequencyDeltaSweep", as_f64),
        repeat_frequency: read_param!(RepeatFrequency, "repeatFrequency", as_f64),
        frequency_jump1_onset: read_param!(FrequencyJump1Onset, "frequencyJump1Onset", as_f64),
        frequency_jump1_amount: read_param!(FrequencyJump1Amount, "frequencyJump1Amount", as_f64),
        frequency_jump2_onset: read_param!(FrequencyJump2Onset, "frequencyJump2Onset", as_f64),
        frequency_jump2_amount: read_param!(FrequencyJump2Amount, "frequencyJump2Amount", as_f64),
        harmonics: read_param!(Harmonics, "harmonics", as_i32),
        harmonics_falloff: read_param!(HarmonicsFalloff, "harmonicsFalloff", as_f64),
        waveform: match read_field!("waveform", as_str) {
            "sine" => crate::parameter::Waveform::Sine,
            "triangle" => crate::parameter::Waveform::Triangle,
            "sawtooth" => crate::parameter::Waveform::Sawtooth,
            "square" => crate::parameter::Waveform::Square,
            "tangent" => crate::parameter::Waveform::Tangent,
            "whistle" => crate::parameter::Waveform::Whistle,
            "breaker" => crate::parameter::Waveform::Breaker,
            "whitenoise" => crate::parameter::Waveform::Whitenoise,
            "pinknoise" => crate::parameter::Waveform::Pinknoise,
            "brownnoise" => crate::parameter::Waveform::Brownnoise,
            _ => return Err(JfxrFormatError::InvalidField("waveform")),
        },
        interpolate_noise: read_param!(InterpolateNoise, "interpolateNoise", as_bool),
        vibrato_depth: read_param!(VibratoDepth, "vibratoDepth", as_f64),
        vibrato_frequency: read_param!(VibratoFrequency, "vibratoFrequency", as_f64),
        square_duty: read_param!(SquareDuty, "squareDuty", as_f64),
        square_duty_sweep: read_param!(SquareDutySweep, "squareDutySweep", as_f64),
        flanger_offset: read_param!(FlangerOffset, "flangerOffset", as_f64),
        flanger_offset_sweep: read_param!(FlangerOffsetSweep, "flangerOffsetSweep", as_f64),
        bit_crush: read_param!(BitCrush, "bitCrush", as_i32),
        bit_crush_sweep: read_param!(BitCrushSweep, "bitCrushSweep", as_i32),
        low_pass_cutoff: read_param!(LowPassCutoff, "lowPassCutoff", as_f64),
        low_pass_cutoff_sweep: read_param!(LowPassCutoffSweep, "lowPassCutoffSweep", as_f64),
        high_pass_cutoff: read_param!(HighPassCutoff, "highPassCutoff", as_f64),
        high_pass_cutoff_sweep: read_param!(HighPassCutoffSweep, "highPassCutoffSweep", as_f64),
        compression: read_param!(Compression, "compression", as_f64),
        normalization: read_param!(Normalization, "normalization", as_bool),
        amplification: read_param!(Amplification, "amplification", as_f64),
    })
}

/// Encodes a [`Sound`] to the `jfxr` format.
pub fn write_jfxr(sound: Sound) -> String {
    let mut json = json::object::Object::new();
    json.insert("_version", VERSION.into());
    json.insert("_name", sound.name.into());
    json.insert("_locked", json::JsonValue::new_array());
    json.insert("sampleRate", sound.sample_rate.0.into());
    json.insert("attack", sound.attack.0.into());
    json.insert("sustain", sound.sustain.0.into());
    json.insert("sustainPunch", sound.sustain_punch.0.into());
    json.insert("decay", sound.decay.0.into());
    json.insert("tremoloDepth", sound.tremolo_depth.0.into());
    json.insert("tremoloFrequency", sound.tremolo_frequency.0.into());
    json.insert("frequency", sound.frequency.0.into());
    json.insert("frequencySweep", sound.frequency_sweep.0.into());
    json.insert("frequencyDeltaSweep", sound.frequency_delta_sweep.0.into());
    json.insert("repeatFrequency", sound.repeat_frequency.0.into());
    json.insert("frequencyJump1Onset", sound.frequency_jump1_onset.0.into());
    json.insert("frequencyJump1Amount", sound.frequency_jump1_amount.0.into());
    json.insert("frequencyJump2Onset", sound.frequency_jump2_onset.0.into());
    json.insert("frequencyJump2Amount", sound.frequency_jump2_amount.0.into());
    json.insert("harmonics", sound.harmonics.0.into());
    json.insert("harmonicsFalloff", sound.harmonics_falloff.0.into());
    json.insert("waveform", match sound.waveform {
        crate::parameter::Waveform::Sine => "sine",
        crate::parameter::Waveform::Triangle => "triangle",
        crate::parameter::Waveform::Sawtooth => "sawtooth",
        crate::parameter::Waveform::Square => "square",
        crate::parameter::Waveform::Tangent => "tangent",
        crate::parameter::Waveform::Whistle => "whistle",
        crate::parameter::Waveform::Breaker => "breaker",
        crate::parameter::Waveform::Whitenoise => "whitenoise",
        crate::parameter::Waveform::Pinknoise => "pinknoise",
        crate::parameter::Waveform::Brownnoise => "brownnoise",
    }.into());
    json.insert("interpolateNoise", sound.interpolate_noise.0.into());
    json.insert("vibratoDepth", sound.vibrato_depth.0.into());
    json.insert("vibratoFrequency", sound.vibrato_frequency.0.into());
    json.insert("squareDuty", sound.square_duty.0.into());
    json.insert("squareDutySweep", sound.square_duty_sweep.0.into());
    json.insert("flangerOffset", sound.flanger_offset.0.into());
    json.insert("flangerOffsetSweep", sound.flanger_offset_sweep.0.into());
    json.insert("bitCrush", sound.bit_crush.0.into());
    json.insert("bitCrushSweep", sound.bit_crush_sweep.0.into());
    json.insert("lowPassCutoff", sound.low_pass_cutoff.0.into());
    json.insert("lowPassCutoffSweep", sound.low_pass_cutoff_sweep.0.into());
    json.insert("highPassCutoff", sound.high_pass_cutoff.0.into());
    json.insert("highPassCutoffSweep", sound.high_pass_cutoff_sweep.0.into());
    json.insert("compression", sound.compression.0.into());
    json.insert("normalization", sound.normalization.0.into());
    json.insert("amplification", sound.amplification.0.into());
    json.dump()
}
