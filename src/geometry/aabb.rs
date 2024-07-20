type TNum = i32;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub top_left: (TNum, TNum),
    pub size: (TNum, TNum),
}

impl AABB {
    /// Creates an AABB from two points,
    /// one indicating the AABB top left,
    /// and another indicating the AABB's size.
    pub fn new(top_left: (TNum, TNum), size: (TNum, TNum)) -> Self {
        Self { top_left, size }
    }

    /// Returns a new AABB expanded outwards by `value`.
    /// Negative values expand inwards.
    pub fn expand_radius(self, value: TNum) -> Self {
        Self {
            top_left: (self.top_left.0 - value, self.top_left.1 - value),
            size: (self.size.0 + value + value, self.size.1 + value + value),
        }
    }

    pub fn contains_point(&self, point: (i32, i32)) -> bool {
        point.0 > self.top_left.0
            && point.0 < self.top_left.0 + self.size.0
            && point.1 > self.top_left.1
            && point.1 < self.top_left.1 + self.size.1
    }
}
