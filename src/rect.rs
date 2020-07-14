use super::Vector2;

pub struct Rect {
    pub position: Vector2,
    pub size: Vector2,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            position: Vector2::new(x, y),
            size: Vector2::new(w, h),
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.position.x <= other.end().x
            && self.end().x >= other.position.x
            && self.position.y <= other.end().y
            && self.end().y >= other.position.y
    }

    pub fn center(&self) -> Vector2 {
        Vector2::new(
            (self.position.x + self.end().x) / 2,
            (self.position.y + self.end().y) / 2,
        )
    }

    pub fn end(&self) -> Vector2 {
        Vector2::new(self.position.x + self.size.x, self.position.y + self.size.y)
    }
}
