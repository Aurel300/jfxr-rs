use std::f64::consts::PI;

fn lerp(a: f64, b: f64, f: f64) -> f64 {
    (1.0 - f) * a + f * b
}

struct Random {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        let mut ret = Self {
            x: seed,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        };
        for _ in 0..32 {
            ret.uint32();
        }
        ret
    }

    pub fn uint32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w.wrapping_add(0x80000000)
    }

    pub fn uniform(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.uint32() as f64 / 0xffffffffu64 as f64
    }
}

/*
  
  Random.prototype.uniform = function(min, max) {
    if (min === undefined && max === undefined) {
      min = 0;
      max = 1;
    } else if (max === undefined) {
      max = min;
      min = 0;
    }
    return min + (max - min) * this.uint32() / 0xffffffff;
  };
  
  Random.prototype.int = function(min, max) {
    return Math.floor(this.uniform(min, max));
  };
  
  Random.prototype.boolean = function(trueProbability) {
    return this.uniform() < trueProbability;
  };
  
  Random.prototype.fromArray = function(array) {
    return array[this.int(array.length)];
  };
*/

pub trait Oscillator {
    fn get_sample(&mut self, sound: &super::sound::Sound, phase: f64, time: f64) -> f64;
}

pub struct SineOscillator;

impl SineOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for SineOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        (2.0 * PI * phase).sin()
    }
}

pub struct TriangleOscillator;

impl TriangleOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for TriangleOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        if phase < 0.25 { return 4.0 * phase; }
        if phase < 0.75 { return 2.0 - 4.0 * phase; }
        -4.0 + 4.0 * phase
    }
}

pub struct SawtoothOscillator;

impl SawtoothOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for SawtoothOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        if phase < 0.5 { return 2.0 * phase; }
        -2.0 + 2.0 * phase
    }
}

pub struct SquareOscillator;

impl SquareOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for SquareOscillator {
    fn get_sample(&mut self, sound: &super::sound::Sound, phase: f64, time: f64) -> f64 {
        if phase < sound.square_duty_at(time) { return 1.0; }
        -1.0
    }
}

pub struct TangentOscillator;

impl TangentOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for TangentOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        // Arbitrary cutoff value to make normalization behave.
        (0.3 * (PI * phase).tan()).clamp(-2.0, 2.0)
    }
}

pub struct WhistleOscillator;

impl WhistleOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for WhistleOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        0.75 * (2.0 * PI * phase).sin() + 0.25 * (40.0 * PI * phase).sin()
    }
}

pub struct BreakerOscillator;

impl BreakerOscillator {
    pub fn new(_sound: &super::sound::Sound) -> Self {
        Self
    }
}

impl Oscillator for BreakerOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        // Make sure to start at a zero crossing.
        let p = (phase + 0.75f64.sqrt()).fract();
        -1.0 + 2.0 * (1.0 - p * p * 2.0).abs()
    }
}

pub struct WhiteNoiseOscillator {
    interpolate_noise: bool,
    random: Random,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
}

impl WhiteNoiseOscillator {
    pub fn new(sound: &super::sound::Sound) -> Self {
        Self {
            interpolate_noise: sound.interpolate_noise.0,
            random: Random::new(0x3cf78ba3),
            prev_phase: 0.0,
            prev_random: 0.0,
            curr_random: 0.0,
        }
    }
}

impl Oscillator for WhiteNoiseOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        // Need two samples per phase in order to include the desired frequencies.
        let phase = (phase * 2.0).fract();
        if phase < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = self.random.uniform(-1.0, 1.0);
        }
        self.prev_phase = phase;
        if self.interpolate_noise { return lerp(self.prev_random, self.curr_random, phase); }
        self.curr_random
    }
}

pub struct PinkNoiseOscillator {
    interpolate_noise: bool,
    random: Random,
    prev_phase: f64,
    b: [f64; 7],
    prev_random: f64,
    curr_random: f64,
}

impl PinkNoiseOscillator {
    pub fn new(sound: &super::sound::Sound) -> Self {
        Self {
            interpolate_noise: sound.interpolate_noise.0,
            random: Random::new(0x3cf78ba3),
            prev_phase: 0.0,
            b: [0.0; 7],
            prev_random: 0.0,
            curr_random: 0.0,
        }
    }
}

impl Oscillator for PinkNoiseOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        // Need two samples per phase in order to include the desired frequencies.
        let phase = (phase * 2.0).fract();
        if phase < self.prev_phase {
            self.prev_random = self.curr_random;
            // Method pk3 from http://www.firstpr.com.au/dsp/pink-noise/,
            // due to Paul Kellet.
            let white = self.random.uniform(-1.0, 1.0);
            self.b[0] = 0.99886 * self.b[0] + white * 0.0555179;
            self.b[1] = 0.99332 * self.b[1] + white * 0.0750759;
            self.b[2] = 0.96900 * self.b[2] + white * 0.1538520;
            self.b[3] = 0.86650 * self.b[3] + white * 0.3104856;
            self.b[4] = 0.55000 * self.b[4] + white * 0.5329522;
            self.b[5] = -0.7616 * self.b[5] + white * 0.0168980;
            self.curr_random = (self.b[0] + self.b[1] + self.b[2] + self.b[3] + self.b[4] + self.b[5] + self.b[6] + white * 0.5362) / 7.0;
            self.b[6] = white * 0.115926;
        }
        self.prev_phase = phase;
        if self.interpolate_noise { return lerp(self.prev_random, self.curr_random, phase); }
        self.curr_random
    }
}



pub struct BrownNoiseOscillator {
    interpolate_noise: bool,
    random: Random,
    prev_phase: f64,
    prev_random: f64,
    curr_random: f64,
}

impl BrownNoiseOscillator {
    pub fn new(sound: &super::sound::Sound) -> Self {
        Self {
            interpolate_noise: sound.interpolate_noise.0,
            random: Random::new(0x3cf78ba3),
            prev_phase: 0.0,
            prev_random: 0.0,
            curr_random: 0.0,
        }
    }
}

impl Oscillator for BrownNoiseOscillator {
    fn get_sample(&mut self, _sound: &super::sound::Sound, phase: f64, _time: f64) -> f64 {
        // Need two samples per phase in order to include the desired frequencies.
        let phase = (phase * 2.0).fract();
        if phase < self.prev_phase {
            self.prev_random = self.curr_random;
            self.curr_random = (self.curr_random + 0.1 * self.random.uniform(-1.0, 1.0)).clamp(-1.0, 1.0);
        }
        self.prev_phase = phase;
        if self.interpolate_noise { return lerp(self.prev_random, self.curr_random, phase); }
        self.curr_random
    }
}
