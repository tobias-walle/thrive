use shared::Coordinate;

#[derive(Debug, Clone, Copy)]
pub struct FocusedCoordinate(pub Option<Coordinate>);

impl FocusedCoordinate {
    pub fn coord(&self) -> Option<&Coordinate> {
        self.0.as_ref()
    }

    pub fn is_focused(&self, coord: &Coordinate) -> bool {
        match self.0 {
            Some(focused_coord) => focused_coord == *coord,
            None => false,
        }
    }
}
