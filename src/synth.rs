pub struct Synth<'a> {
    sound: &'a super::sound::Sound,

    array: Vec<f64>,
    start_sample: usize,
    block_size: usize,

    transformers: Vec<Box<dyn Transformer>>,
}

impl<'a> Synth<'a> {
    pub fn new(sound: &'a super::sound::Sound) -> Self {
        let sample_rate = sound.sample_rate.0;
        let num_samples = 1.max((sample_rate * sound.duration()).ceil() as usize);
        let array = vec![0.0f64; num_samples];
        Self {
            sound,
            array,
            start_sample: 0,
            block_size: 10240,
            transformers: vec![
                Box::new(Generator::new(sound)),
                Box::new(Envelope::new(sound)),
                Box::new(Flanger::new(sound)),
                Box::new(BitCrush::new(sound)),
                Box::new(LowPass::new(sound)),
                Box::new(HighPass::new(sound)),
                Box::new(Compress::new(sound)),
                Box::new(Normalize::new(sound)),
                Box::new(Amplify::new(sound)),
            ],
        }
    }

    pub fn tick(&mut self) -> bool {
        let num_samples = self.array.len();

        if self.start_sample >= num_samples {
            return true;
        }

        let end_sample = (self.start_sample + self.block_size).min(num_samples);
        for transformer in self.transformers.iter_mut() {
            transformer.run(self.sound, self.array.as_mut_slice(), self.start_sample, end_sample);
        }
        self.start_sample = end_sample;

        self.start_sample >= num_samples
    }

    pub fn generate(mut self) -> Vec<f64> {
        while !self.tick() {}
        self.array
    }
}

trait Transformer {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize);
}

struct Generator {
    oscillators: Vec<Box<dyn super::oscillator::Oscillator>>,
    first_harmonic_amp: f64,
    phase: f64,
}

impl Generator {
    pub fn new(sound: &super::sound::Sound) -> Self {
        let mut amp = 1.0;
        let mut total_amp = 0.0;
        let oscillators = (0..=sound.harmonics.0)
            .map(|_| {
                total_amp += amp;
                amp *= sound.harmonics_falloff.0;
                let osc: Box<dyn super::oscillator::Oscillator> = match sound.waveform {
                    super::parameter::Waveform::Sine => Box::new(super::oscillator::SineOscillator::new(sound)),
                    super::parameter::Waveform::Triangle => Box::new(super::oscillator::TriangleOscillator::new(sound)),
                    super::parameter::Waveform::Sawtooth => Box::new(super::oscillator::SawtoothOscillator::new(sound)),
                    super::parameter::Waveform::Square => Box::new(super::oscillator::SquareOscillator::new(sound)),
                    super::parameter::Waveform::Tangent => Box::new(super::oscillator::TangentOscillator::new(sound)),
                    super::parameter::Waveform::Whistle => Box::new(super::oscillator::WhistleOscillator::new(sound)),
                    super::parameter::Waveform::Breaker => Box::new(super::oscillator::BreakerOscillator::new(sound)),
                    super::parameter::Waveform::Whitenoise => Box::new(super::oscillator::WhiteNoiseOscillator::new(sound)),
                    super::parameter::Waveform::Pinknoise => Box::new(super::oscillator::PinkNoiseOscillator::new(sound)),
                    super::parameter::Waveform::Brownnoise => Box::new(super::oscillator::BrownNoiseOscillator::new(sound)),
                };
                osc
            })
            .collect();
        Self {
            oscillators,
            first_harmonic_amp: 1.0 / total_amp,
            phase: 0.0,
        }
    }
}

impl Transformer for Generator {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let mut phase = self.phase;
        for i in start_sample..end_sample {
            let time = i as f64 / sound.sample_rate.0;
            let current_frequency = sound.frequency_at(time);
            phase = (phase + current_frequency / sound.sample_rate.0).fract();
            let mut sample = 0.0;
            let mut amp = self.first_harmonic_amp;
            for harmonic_index in 0..=sound.harmonics.0 as usize {
                let harmonic_phase = (phase * (harmonic_index + 1) as f64).fract();
                sample += amp * self.oscillators[harmonic_index].get_sample(sound, harmonic_phase, time);
                amp *= sound.harmonics_falloff.0;
            }
            array[i] = sample;
        }
        self.phase = phase;
    }
}

struct Envelope;

impl Envelope {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Transformer for Envelope {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        if sound.attack.0 == 0.0 && sound.sustain_punch.0 == 0.0 && sound.decay.0 == 0.0 && sound.tremolo_depth.0 == 0.0 {
            return;
        }
        for i in start_sample..end_sample {
            let time = i as f64 / sound.sample_rate.0;
            array[i] *= sound.amplitude_at(time);
        }
    }
}

struct Flanger {
    buffer: Option<Vec<f64>>,
    buffer_pos: usize,
}

impl Flanger {
    pub fn new(sound: &super::sound::Sound) -> Self {
        let mut buffer = None;
        if sound.flanger_offset.0 != 0.0 || sound.flanger_offset_sweep.0 != 0.0 {
            // Maximum 100ms offset
            buffer = Some(vec![0.; (sound.sample_rate.0 * 0.1).ceil() as usize]);
        }
        Self {
            buffer,
            buffer_pos: 0,
        }
    }
}

impl Transformer for Flanger {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        if let Some(buffer) = self.buffer.as_mut() {
            let num_samples = array.len();
            let sample_rate = sound.sample_rate.0;
            let flanger_offset = sound.flanger_offset.0;
            let flanger_offset_sweep = sound.flanger_offset_sweep.0;

            let mut buffer_pos = self.buffer_pos;
            let buffer_length = buffer.len();

            for i in start_sample..end_sample {
                buffer[buffer_pos] = array[i];

                let mut offset_samples = ((flanger_offset + i as f64 / num_samples as f64 * flanger_offset_sweep) / 1000.0 * sample_rate).round() as usize;
                offset_samples = offset_samples.clamp(0, buffer_length - 1);
                array[i] += buffer[(buffer_pos - offset_samples + buffer_length) % buffer_length];
                buffer_pos = (buffer_pos + 1) % buffer_length;
            }

            self.buffer_pos = buffer_pos;
        }
    }
}

struct BitCrush;

impl BitCrush {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Transformer for BitCrush {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let num_samples = array.len();
        let bit_crush = sound.bit_crush.0;
        let bit_crush_sweep = sound.bit_crush_sweep.0;

        if bit_crush == 0 && bit_crush_sweep == 0 {
            return;
        }

        for i in start_sample..end_sample {
            let mut bits = (bit_crush as f64 + i as f64 / num_samples as f64 * bit_crush_sweep as f64).round() as usize;
            bits = bits.clamp(1, 16);
            let steps = f64::powf(2.0, bits as f64);
            array[i] = -1.0 + 2.0 * ((0.5 + 0.5 * array[i]) * steps).round() / steps;
        }
    }
}



struct LowPass {
    low_pass_prev: f64,
}

impl LowPass {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self {
            low_pass_prev: 0.0,
        }
    }
}

impl Transformer for LowPass {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let num_samples = array.len();
        let low_pass_cutoff = sound.low_pass_cutoff.0;
        let low_pass_cutoff_sweep = sound.low_pass_cutoff_sweep.0;
        let sample_rate = sound.sample_rate.0;

        if low_pass_cutoff >= sample_rate / 2.0 && low_pass_cutoff + low_pass_cutoff_sweep >= sample_rate / 2.0 {
            return;
        }

        let mut low_pass_prev = self.low_pass_prev;

        for i in start_sample..end_sample {
            let fraction = i as f64 / num_samples as f64;
            let cutoff = (low_pass_cutoff + fraction * low_pass_cutoff_sweep).clamp(0.0, sample_rate / 2.0);
            let wc = cutoff / sample_rate * std::f64::consts::PI; // Don't we need a factor 2pi instead of pi?
            let cos_wc = wc.cos();
            let mut low_pass_alpha;
            if cos_wc <= 0.0 {
                low_pass_alpha = 1.0;
            } else {
                // From somewhere on the internet: cos wc = 2a / (1+a^2)
                low_pass_alpha = 1.0 / cos_wc - (1.0 / (cos_wc * cos_wc) - 1.0).sqrt();
                low_pass_alpha = 1.0 - low_pass_alpha; // Probably the internet's definition of alpha is different.
            }
            let mut sample = array[i];
            sample = low_pass_alpha * sample + (1.0 - low_pass_alpha) * low_pass_prev;
            low_pass_prev = sample;
            array[i] = sample;
        }

        self.low_pass_prev = low_pass_prev;
    }
}

struct HighPass {
    high_pass_prev_in: f64,
    high_pass_prev_out: f64,
}

impl HighPass {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self {
            high_pass_prev_in: 0.0,
            high_pass_prev_out: 0.0,
        }
    }
}

impl Transformer for HighPass {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let num_samples = array.len();
        let high_pass_cutoff = sound.high_pass_cutoff.0;
        let high_pass_cutoff_sweep = sound.high_pass_cutoff_sweep.0;
        let sample_rate = sound.sample_rate.0;

        if high_pass_cutoff <= 0.0 && high_pass_cutoff + high_pass_cutoff_sweep <= 0.0 {
          return;
        }

        let mut high_pass_prev_in = self.high_pass_prev_in;
        let mut high_pass_prev_out = self.high_pass_prev_out;

        for i in start_sample..end_sample {
            let fraction = i as f64 / num_samples as f64;
            let cutoff = (high_pass_cutoff + fraction * high_pass_cutoff_sweep).clamp(0.0, sample_rate / 2.0);
            let wc = cutoff / sample_rate * std::f64::consts::PI;
            // From somewhere on the internet: a = (1 - sin wc) / cos wc
            let high_pass_alpha = (1.0 - wc.sin()) / wc.cos();
            let mut sample = array[i];
            let orig_sample = sample;
            sample = high_pass_alpha * (high_pass_prev_out - high_pass_prev_in + sample);
            high_pass_prev_in = orig_sample;
            high_pass_prev_out = sample;
            array[i] = sample;
        }

        self.high_pass_prev_in = high_pass_prev_in;
        self.high_pass_prev_out = high_pass_prev_out;
    }
}

struct Compress;

impl Compress {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Transformer for Compress {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let compression = sound.compression.0;

        if compression == 1.0 {
            return;
        }
    
        for i in start_sample..end_sample {
            let mut sample = array[i];
            if sample >= 0.0 {
                sample = f64::powf(sample, compression);
            } else {
                sample = -f64::powf(-sample, compression);
            }
            array[i] = sample;
        }
    }
}

struct Normalize {
    max_sample: f64,
}

impl Normalize {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self {
            max_sample: 0.0,
        }
    }
}

impl Transformer for Normalize {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        if !sound.normalization.0 {
            return;
        }

        let mut max_sample = self.max_sample;
        for i in start_sample..end_sample {
            max_sample = max_sample.max(array[i].abs());
        }
        self.max_sample = max_sample;

        let num_samples = array.len();
        if end_sample == num_samples {
            let factor = 1.0 / max_sample;
            for i in 0..end_sample {
                array[i] *= factor;
            }
        }
    }
}

struct Amplify;

impl Amplify {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Transformer for Amplify {
    fn run(&mut self, sound: &super::sound::Sound, array: &mut [f64], start_sample: usize, end_sample: usize) {
        let factor = sound.amplification.0 / 100.0;

        if factor == 1.0 {
            return;
        }

        for i in start_sample..end_sample {
            array[i] *= factor;
        }
    }
}
