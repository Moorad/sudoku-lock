pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { x: 4, y: 4 }
    }

    pub fn right(&mut self) {
        self.x = self.x + 1;

        if self.x > 8 {
            self.x = 8;
        }
    }

    pub fn left(&mut self) {
        if self.x == 0 {
            return ();
        }

        self.x = self.x - 1;
    }

    pub fn up(&mut self) {
        if self.y == 0 {
            return ();
        }

        self.y = self.y - 1;
    }

    pub fn down(&mut self) {
        self.y = self.y + 1;

        if self.y > 8 {
            self.y = 8;
        }
    }
}
