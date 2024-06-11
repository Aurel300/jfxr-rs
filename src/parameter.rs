pub trait FloatParameter: Copy + Default {
    const LABEL: &'static str;
    const DESCRIPTION: &'static str = "";
    const UNIT: &'static str = "";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64;
    const STEP: f64 = 1.0;
    const LOGARITHMIC: bool = false;
}

pub trait IntegerParameter: Copy + Default {
    const LABEL: &'static str;
    const DESCRIPTION: &'static str = "";
    const UNIT: &'static str = "";
    const MIN_VALUE: i32 = 0;
    const MAX_VALUE: i32;
    const STEP: i32 = 1;
}

pub trait BooleanParameter: Copy + Default {
    const LABEL: &'static str;
    const DESCRIPTION: &'static str = "";
    const UNIT: &'static str = "";
}

pub trait EnumParameter: Copy + Default + 'static {
    const LABEL: &'static str;
    const DESCRIPTION: &'static str = "";
    const UNIT: &'static str = "";
    const VALUES: &'static [Self];
}

// Sound properties

#[derive(Clone, Copy)]
pub struct SampleRate(pub f64);
impl Default for SampleRate {
    fn default() -> Self {
        Self(44100.0)
    }
}
impl FloatParameter for SampleRate {
    const LABEL: &'static str = "Sample rate";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 44100.0;
    const MAX_VALUE: f64 = 44100.0;
}

// Amplitude parameters

#[derive(Clone, Copy, Default)]
pub struct Attack(pub f64);
impl FloatParameter for Attack {
    const LABEL: &'static str = "Attack";
    const DESCRIPTION: &'static str = "Time from the start of the sound until the point where it reaches its maximum volume. Increase this for a gradual fade-in; decrease it to add more \"punch\".";
    const UNIT: &'static str = "s";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 5.0;
    const STEP: f64 = 0.01;
}

#[derive(Clone, Copy, Default)]
pub struct Sustain(pub f64);
impl FloatParameter for Sustain {
    const LABEL: &'static str = "Sustain";
    const DESCRIPTION: &'static str = "Amount of time for which the sound holds its maximum volume after the attack phase. Increase this to increase the sound's duration.";
    const UNIT: &'static str = "s";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 5.0;
    const STEP: f64 = 0.01;
}

#[derive(Clone, Copy, Default)]
pub struct SustainPunch(pub f64);
impl FloatParameter for SustainPunch {
    const LABEL: &'static str = "Sustain punch";
    const DESCRIPTION: &'static str = "Additional volume at the start of the sustain phase, which linearly fades back to the base level. Use this to add extra \"punch\" to the sustain phase.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 10.0;
}

#[derive(Clone, Copy, Default)]
pub struct Decay(pub f64);
impl FloatParameter for Decay {
    const LABEL: &'static str = "Decay";
    const DESCRIPTION: &'static str = "Time it takes from the end of the sustain phase until the sound has faded away. Increase this for a gradual fade-out.";
    const UNIT: &'static str = "s";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 5.0;
    const STEP: f64 = 0.01;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct TremoloDepth(pub f64);
impl FloatParameter for TremoloDepth {
    const LABEL: &'static str = "Tremolo depth";
    const DESCRIPTION: &'static str = "Amount by which the volume oscillates as a sine wave around its base value.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 1.0;
}

#[derive(Clone, Copy)]
pub struct TremoloFrequency(pub f64);
impl Default for TremoloFrequency {
    fn default() -> Self {
        Self(10.0)
    }
}
impl FloatParameter for TremoloFrequency {
    const LABEL: &'static str = "Tremolo frequency";
    const DESCRIPTION: &'static str = "Frequency at which the volume oscillates as a sine wave around its base value.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 1000.0;
    const STEP: f64 = 1.0;
    const LOGARITHMIC: bool = true;
}

// Pitch parameters

#[derive(Clone, Copy)]
pub struct Frequency(pub f64);
impl Default for Frequency {
    fn default() -> Self {
        Self(500.0)
    }
}
impl FloatParameter for Frequency {
    const LABEL: &'static str = "Frequency";
    const DESCRIPTION: &'static str = "Initial frequency, or pitch, of the sound. This determines how high the sound starts out; higher values result in higher notes.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 10.0;
    const MAX_VALUE: f64 = 10000.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct FrequencySweep(pub f64);
impl FloatParameter for FrequencySweep {
    const LABEL: &'static str = "Frequency sweep";
    const DESCRIPTION: &'static str = "Amount by which the frequency is changed linearly over the duration of the sound.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = -10000.0;
    const MAX_VALUE: f64 = 10000.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct FrequencyDeltaSweep(pub f64);
impl FloatParameter for FrequencyDeltaSweep {
    const LABEL: &'static str = "Freq. delta sweep";
    const DESCRIPTION: &'static str = "Amount by which the frequency is changed quadratically over the duration of the sound.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = -10000.0;
    const MAX_VALUE: f64 = 10000.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct RepeatFrequency(pub f64);
impl FloatParameter for RepeatFrequency {
    const LABEL: &'static str = "Repeat frequency";
    const DESCRIPTION: &'static str = "Amount of times per second that the frequency is reset to its base value, and starts its sweep cycle anew.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 0.1;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy)]
pub struct FrequencyJump1Onset(pub f64);
impl Default for FrequencyJump1Onset {
    fn default() -> Self {
        Self(33.0)
    }
}
impl FloatParameter for FrequencyJump1Onset {
    const LABEL: &'static str = "Freq. jump 1 onset";
    const DESCRIPTION: &'static str = "Point in time, as a fraction of the repeat cycle, at which the frequency makes a sudden jump.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
}

#[derive(Clone, Copy, Default)]
pub struct FrequencyJump1Amount(pub f64);
impl FloatParameter for FrequencyJump1Amount {
    const LABEL: &'static str = "Freq. jump 1 amount";
    const DESCRIPTION: &'static str = "Amount by which the frequency jumps at the given onset, as a fraction of the current frequency.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = -100.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
}

#[derive(Clone, Copy)]
pub struct FrequencyJump2Onset(pub f64);
impl Default for FrequencyJump2Onset {
    fn default() -> Self {
        Self(66.0)
    }
}
impl FloatParameter for FrequencyJump2Onset {
    const LABEL: &'static str = "Freq. jump 2 onset";
    const DESCRIPTION: &'static str = "Point in time, as a fraction of the repeat cycle, at which the frequency makes a sudden jump.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
}

#[derive(Clone, Copy, Default)]
pub struct FrequencyJump2Amount(pub f64);
impl FloatParameter for FrequencyJump2Amount {
    const LABEL: &'static str = "Freq. jump 2 amount";
    const DESCRIPTION: &'static str = "Amount by which the frequency jumps at the given onset, as a fraction of the current frequency.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = -100.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
}

// Harmonics parameters

#[derive(Clone, Copy, Default)]
pub struct Harmonics(pub i32);
impl IntegerParameter for Harmonics {
    const LABEL: &'static str = "Harmonics";
    const DESCRIPTION: &'static str = "Number of harmonics (overtones) to add. Generates the same sound at several multiples of the base frequency (2×, 3×, …), and mixes them with the original sound. Note that this slows down rendering quite a lot, so you may want to leave it at 0 until the last moment.";
    const MIN_VALUE: i32 = 0;
    const MAX_VALUE: i32 = 5;
    const STEP: i32 = 1;
}

#[derive(Clone, Copy)]
pub struct HarmonicsFalloff(pub f64);
impl Default for HarmonicsFalloff {
    fn default() -> Self {
        Self(6.5)
    }
}
impl FloatParameter for HarmonicsFalloff {
    const LABEL: &'static str = "Harmonics falloff";
    const DESCRIPTION: &'static str = "Volume of each subsequent harmonic, as a fraction of the previous one.";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 1.0;
    const STEP: f64 = 0.01;
}

// Tone parameters

#[derive(Clone, Copy, Default)]
pub enum Waveform {
    #[default] Sine,
    Triangle,
    Sawtooth,
    Square,
    Tangent,
    Whistle,
    Breaker,
    Whitenoise,
    Pinknoise,
    Brownnoise,
}
impl EnumParameter for Waveform {
    const LABEL: &'static str = "Waveform";
    const DESCRIPTION: &'static str = "Shape of the waveform. This is the most important factor in determining the character, or timbre, of the sound.";
    const VALUES: &'static [Self] = &[
        Self::Sine,
        Self::Triangle,
        Self::Sawtooth,
        Self::Square,
        Self::Tangent,
        Self::Whistle,
        Self::Breaker,
        Self::Whitenoise,
        Self::Pinknoise,
        Self::Brownnoise,
    ];
}

#[derive(Clone, Copy)]
pub struct InterpolateNoise(pub bool);
impl Default for InterpolateNoise {
    fn default() -> Self {
        Self(true)
    }
}
impl BooleanParameter for InterpolateNoise {
    const LABEL: &'static str = "Interpolate noise";
    const DESCRIPTION: &'static str = "Whether to use linear interpolation between individual samples of noise. This results in a smoother sound.";
    /*
    disabledReason: function(sound) {
      var waveform = sound.waveform.value;
      if (waveform != 'whitenoise' && waveform != 'pinknoise' && waveform != 'brownnoise') {
        return 'Noise interpolation only applies to noise waveforms';
      }
    },
    */
}

#[derive(Clone, Copy, Default)]
pub struct VibratoDepth(pub f64);
impl FloatParameter for VibratoDepth {
    const LABEL: &'static str = "Vibrato depth";
    const DESCRIPTION: &'static str = "Amount by which to vibrate around the base frequency.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 1000.0;
    const STEP: f64 = 10.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy)]
pub struct VibratoFrequency(pub f64);
impl Default for VibratoFrequency {
    fn default() -> Self {
        Self(10.0)
    }
}
impl FloatParameter for VibratoFrequency {
    const LABEL: &'static str = "Vibrato frequency";
    const DESCRIPTION: &'static str = "Number of times per second to vibrate around the base frequency.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 1000.0;
    const STEP: f64 = 1.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy)]
pub struct SquareDuty(pub f64);
impl Default for SquareDuty {
    fn default() -> Self {
        Self(50.0)
    }
}
impl FloatParameter for SquareDuty {
    const LABEL: &'static str = "Square duty";
    const DESCRIPTION: &'static str = "For square waves only, the initial fraction of time the square is in the \"on\" state.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
    // disabledReason: isNotSquare,
}

#[derive(Clone, Copy, Default)]
pub struct SquareDutySweep(pub f64);
impl FloatParameter for SquareDutySweep {
    const LABEL: &'static str = "Square duty sweep";
    const DESCRIPTION: &'static str = "For square waves only, change the square duty linearly by this many percentage points over the course of the sound.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = -100.0;
    const MAX_VALUE: f64 = 100.0;
    const STEP: f64 = 5.0;
    // disabledReason: isNotSquare,
}

// Filter parameters

#[derive(Clone, Copy, Default)]
pub struct FlangerOffset(pub f64);
impl FloatParameter for FlangerOffset {
    const LABEL: &'static str = "Flanger offset";
    const DESCRIPTION: &'static str = "The initial offset for the flanger effect. Mixes the sound with itself, delayed initially by this amount.";
    const UNIT: &'static str = "ms";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 50.0;
    const STEP: f64 = 1.0;
}

#[derive(Clone, Copy, Default)]
pub struct FlangerOffsetSweep(pub f64);
impl FloatParameter for FlangerOffsetSweep {
    const LABEL: &'static str = "Flanger offset sweep";
    const DESCRIPTION: &'static str = "Amount by which the flanger offset changes linearly over the course of the sound.";
    const UNIT: &'static str = "ms";
    const MIN_VALUE: f64 = -50.0;
    const MAX_VALUE: f64 = 50.0;
    const STEP: f64 = 1.0;
}

#[derive(Clone, Copy)]
pub struct BitCrush(pub i32);
impl Default for BitCrush {
    fn default() -> Self {
        Self(16)
    }
}
impl IntegerParameter for BitCrush {
    const LABEL: &'static str = "Bit crush";
    const DESCRIPTION: &'static str = "Number of bits per sample. Reduces the number of bits in each sample by this amount, and then increase it again. The result is a lower-fidelity sound effect.";
    const UNIT: &'static str = "bits";
    const MIN_VALUE: i32 = 1;
    const MAX_VALUE: i32 = 16;
    const STEP: i32 = 1;
}

#[derive(Clone, Copy, Default)]
pub struct BitCrushSweep(pub i32);
impl IntegerParameter for BitCrushSweep {
    const LABEL: &'static str = "Bit crush sweep";
    const DESCRIPTION: &'static str = "Amount by which to change the bit crush value linearly over the course of the sound.";
    const UNIT: &'static str = "bits";
    const MIN_VALUE: i32 = -16;
    const MAX_VALUE: i32 = 16;
    const STEP: i32 = 1;
}

#[derive(Clone, Copy)]
pub struct LowPassCutoff(pub f64);
impl Default for LowPassCutoff {
    fn default() -> Self {
        Self(22050.0)
    }
}
impl FloatParameter for LowPassCutoff {
    const LABEL: &'static str = "Low-pass cutoff";
    const DESCRIPTION: &'static str = "Threshold above which frequencies should be filtered out, using a simple IIR low-pass filter. Use this to take some \"edge\" off the sound.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 22050.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct LowPassCutoffSweep(pub f64);
impl FloatParameter for LowPassCutoffSweep {
    const LABEL: &'static str = "Low-pass sweep";
    const DESCRIPTION: &'static str = "Amount by which to change the low-pass cutoff frequency over the course of the sound.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = -22050.0;
    const MAX_VALUE: f64 = 22050.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct HighPassCutoff(pub f64);
impl FloatParameter for HighPassCutoff {
    const LABEL: &'static str = "High-pass cutoff";
    const DESCRIPTION: &'static str = "Threshold below which frequencies should be filtered out, using a simple high-pass filter.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 22050.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}

#[derive(Clone, Copy, Default)]
pub struct HighPassCutoffSweep(pub f64);
impl FloatParameter for HighPassCutoffSweep {
    const LABEL: &'static str = "High-pass sweep";
    const DESCRIPTION: &'static str = "Amount by which to change the high-pass cutoff frequency over the course of the sound.";
    const UNIT: &'static str = "Hz";
    const MIN_VALUE: f64 = -22050.0;
    const MAX_VALUE: f64 = 22050.0;
    const STEP: f64 = 100.0;
    const LOGARITHMIC: bool = true;
}


  // Output parameters

#[derive(Clone, Copy)]
pub struct Compression(pub f64);
impl Default for Compression {
    fn default() -> Self {
        Self(1.0)
    }
}
impl FloatParameter for Compression {
    const LABEL: &'static str = "Compression";
    const DESCRIPTION: &'static str = "Power to which sample values should be raised. 1 is the neutral setting. Use a value less than 1 to increase the volume of quiet parts of the sound, higher than 1 to make quiet parts even quieter.";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 5.0;
    const STEP: f64 = 0.1;
}

#[derive(Clone, Copy)]
pub struct Normalization(pub bool);
impl Default for Normalization {
    fn default() -> Self {
        Self(true)
    }
}
impl BooleanParameter for Normalization {
    const LABEL: &'static str = "Normalization";
    const DESCRIPTION: &'static str = "Whether to adjust the volume of the sound so that the peak volume is at 100%.";
}

#[derive(Clone, Copy)]
pub struct Amplification(pub f64);
impl Default for Amplification {
    fn default() -> Self {
        Self(100.0)
    }
}
impl FloatParameter for Amplification {
    const LABEL: &'static str = "Amplification";
    const DESCRIPTION: &'static str = "Percentage to amplify the sound by, after any normalization has occurred. Note that setting this too high can result in clipping.";
    const UNIT: &'static str = "%";
    const MIN_VALUE: f64 = 0.0;
    const MAX_VALUE: f64 = 500.0;
    const STEP: f64 = 10.0;
}
