pub struct GameState{
    width: usize,
    height: usize,

}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}


#[derive(Clone, Copy)]
pub struct Ball {
    pos: Point,
    radius: f64,
    velocity: Point // velocity 2d vector

}