use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const FULL: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn ordered(a: f32, b: f32) -> Self {
        if a < b {
            Self::new(a, b)
        } else {
            Self::new(b, a)
        }
    }

    pub const fn length(&self) -> f32 {
        self.max - self.min
    }

    pub const fn is_empty(&self) -> bool {
        self.min >= self.max
    }

    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub const fn expand(self, delta: f32) -> Self {
        let half = delta / 2.0;
        Self::new(self.min - half, self.max + half)
    }

    pub const fn intersect(self, other: Self) -> Self {
        Self::new(f32::max(self.min, other.min), f32::min(self.max, other.max))
    }

    pub const fn enclose(self, other: Self) -> Self {
        Self::new(f32::min(self.min, other.min), f32::max(self.max, other.max))
    }

    pub fn cmp_min(&self, other: &Self) -> Ordering {
        self.min.total_cmp(&other.min)
    }
}
