#[derive(Clone, Default)]
pub struct Sound {
    pub sample_rate: crate::parameter::SampleRate,
    pub attack: crate::parameter::Attack,
    pub sustain: crate::parameter::Sustain,
    pub sustain_punch: crate::parameter::SustainPunch,
    pub decay: crate::parameter::Decay,
    pub tremolo_depth: crate::parameter::TremoloDepth,
    pub tremolo_frequency: crate::parameter::TremoloFrequency,
    pub frequency: crate::parameter::Frequency,
    pub frequency_sweep: crate::parameter::FrequencySweep,
    pub frequency_delta_sweep: crate::parameter::FrequencyDeltaSweep,
    pub repeat_frequency: crate::parameter::RepeatFrequency,
    pub frequency_jump1_onset: crate::parameter::FrequencyJump1Onset,
    pub frequency_jump1_amount: crate::parameter::FrequencyJump1Amount,
    pub frequency_jump2_onset: crate::parameter::FrequencyJump2Onset,
    pub frequency_jump2_amount: crate::parameter::FrequencyJump2Amount,
    pub harmonics: crate::parameter::Harmonics,
    pub harmonics_falloff: crate::parameter::HarmonicsFalloff,
    pub waveform: crate::parameter::Waveform,
    pub interpolate_noise: crate::parameter::InterpolateNoise,
    pub vibrato_depth: crate::parameter::VibratoDepth,
    pub vibrato_frequency: crate::parameter::VibratoFrequency,
    pub square_duty: crate::parameter::SquareDuty,
    pub square_duty_sweep: crate::parameter::SquareDutySweep,
    pub flanger_offset: crate::parameter::FlangerOffset,
    pub flanger_offset_sweep: crate::parameter::FlangerOffsetSweep,
    pub bit_crush: crate::parameter::BitCrush,
    pub bit_crush_sweep: crate::parameter::BitCrushSweep,
    pub low_pass_cutoff: crate::parameter::LowPassCutoff,
    pub low_pass_cutoff_sweep: crate::parameter::LowPassCutoffSweep,
    pub high_pass_cutoff: crate::parameter::HighPassCutoff,
    pub high_pass_cutoff_sweep: crate::parameter::HighPassCutoffSweep,
    pub compression: crate::parameter::Compression,
    pub normalization: crate::parameter::Normalization,
    pub amplification: crate::parameter::Amplification,
}

impl Sound {
    pub fn duration(&self) -> f64 {
        self.attack.0 + self.sustain.0 + self.decay.0
    }
    pub fn effective_repeat_frequency(&self) -> f64 {
        self.repeat_frequency.0.max(1.0 / self.duration())
    }
    pub fn frequency_at(&self, time: f64) -> f64 {
        let repeat_frequency = self.effective_repeat_frequency();
        let fraction_in_repetition = (time * repeat_frequency).fract();
        let mut freq = self.frequency.0
            + fraction_in_repetition * self.frequency_sweep.0
            + fraction_in_repetition * fraction_in_repetition * self.frequency_delta_sweep.0;
        if fraction_in_repetition > self.frequency_jump1_onset.0 / 100.0 {
            freq *= 1.0 + self.frequency_jump1_amount.0 / 100.0;
        }
        if fraction_in_repetition > self.frequency_jump2_onset.0 / 100.0 {
            freq *= 1.0 + self.frequency_jump2_amount.0 / 100.0;
        }
        if self.vibrato_depth.0 != 0.0 {
            freq += 1.0 - self.vibrato_depth.0 * (0.5 - 0.5 * (2.0 * std::f64::consts::PI * time * self.vibrato_frequency.0).sin());
        }
        freq.max(0.0)
    }
    pub fn square_duty_at(&self, time: f64) -> f64 {
        let repeat_frequency = self.effective_repeat_frequency();
        let fraction_in_repetition = (time * repeat_frequency).fract();
        (self.square_duty.0 + fraction_in_repetition * self.square_duty_sweep.0) / 100.0
    }
    pub fn amplitude_at(&self, time: f64) -> f64 {
        let attack = self.attack.0;
        let sustain = self.sustain.0;
        let sustain_punch = self.sustain_punch.0;
        let decay = self.decay.0;
        let tremolo_depth = self.tremolo_depth.0;
        let mut amp;
        if time < attack {
            amp = time / attack;
        } else if time < attack + sustain {
            amp = 1.0 + sustain_punch / 100.0 * (1.0 - (time - attack) / sustain);
        } else if time < attack + sustain + decay {
            amp = 1.0 - (time - attack - sustain) / decay;
        } else { // This can happen due to roundoff error because the sample count is an integer.
            amp = 0.0;
        }
        if tremolo_depth != 0.0 {
            amp *= 1.0 - (tremolo_depth / 100.0) * (0.5 + 0.5 * (2.0 * std::f64::consts::PI * time * self.tremolo_frequency.0).cos());
        }
        amp
    }
}
