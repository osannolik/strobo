use charlieplexing::{BoolPatternMatrix, ALL_OFF, ALL_ON};
use num;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Rpm(f32);

impl Rpm {
    pub fn to_degps(self) -> f32 {
        360.0 * self.0 / 60.0
    }
}

pub trait ToRpm {
    fn rpm(self) -> Rpm;
}

impl ToRpm for f32 {
    fn rpm(self) -> Rpm {
        Rpm(self)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Degree(f32);

impl Degree {
    pub fn to_rad(self) -> f32 {
        self.0 * core::f32::consts::PI / 180.0
    }
}

pub trait ToDegree {
    fn deg(self) -> Degree;
}

impl ToDegree for f32 {
    fn deg(self) -> Degree {
        Degree(self)
    }
}

type TimingArray = [f32; 2];
type PatternArray = [BoolPatternMatrix; 2];

pub struct StrobeSteps {
    seq: PatternArray,
    times: TimingArray,
    index: usize,
    angle: Degree,
}

const STROBE_WIDTH: f32 = 0.1;

fn timing(velocity: Rpm, angle: Degree) -> TimingArray {
    assert_ne!(velocity.0, 0.0);
    let period = num::clamp(angle.0 / velocity.to_degps(), 0.0, 1.0);
    let high_time = STROBE_WIDTH * period;
    [period - high_time, high_time]
}

const PERIOD_33RPM_12DEG: f32 = 12.0 / (100.0 / 3.0 / 60.0 * 360.0);

pub const RPM33_DEFAULT: StrobeSteps = StrobeSteps {
    seq: [ALL_OFF, ALL_ON],
    times: [
        (1.0 - STROBE_WIDTH) * PERIOD_33RPM_12DEG,
        STROBE_WIDTH * PERIOD_33RPM_12DEG,
    ],
    index: 0,
    angle: Degree { 0: 12.0 },
};

impl StrobeSteps {
    pub fn new(velocity: Rpm, angle: Degree) -> StrobeSteps {
        StrobeSteps {
            seq: [ALL_OFF, ALL_ON],
            times: timing(velocity, angle),
            index: 0,
            angle,
        }
    }

    pub fn set_timing(&mut self, velocity: Rpm) {
        self.times = timing(velocity, self.angle);
    }

    pub fn time(&self) -> f32 {
        self.times[self.index]
    }

    pub fn pattern(&self) -> BoolPatternMatrix {
        self.seq[self.index]
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.seq.len();
    }
}
