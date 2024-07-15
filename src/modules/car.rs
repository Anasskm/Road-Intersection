extern crate rand;

use super::consts::*;
use super::se_base::*;

use rand::Rng;
use sdl2::pixels::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub direction: Direction,
    pub side: Side,
    pub velocity: i32,
}

impl Car {
    pub fn new(side: Side) -> Car {
        let random_number = rand::thread_rng().gen_range(0..3);
        let velocity = rand::thread_rng().gen_range(MIN_VELOCITY..MAX_VELOCITY);
        let direction: Direction;
        let color: Color;
        match random_number {
            0 => {
                direction = Direction::Left;
                color = CAR_COLOR_LEFT;
            }
            1 => {
                direction = Direction::Straight;
                color = CAR_COLOR_STRAIGHT;
            }
            _ => {
                direction = Direction::Right;
                color = CAR_COLOR_RIGHT;
            }
        }
        match side {
            Side::FromEast => {
                let x = 0;
                let y = OUTPUT_HEIGHT / 2;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromNorth => {
                let x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                let y = 0;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromSouth => {
                let x = OUTPUT_WIDTH / 2;
                let y = OUTPUT_HEIGHT - CAR_HEIGHT;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
            Side::FromWest => {
                let x = OUTPUT_WIDTH - CAR_WIDTH;
                let y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                return Car {
                    x,
                    y,
                    color,
                    direction,
                    side,
                    velocity,
                };
            }
        }
    }
    pub fn random_car() -> Car {
        let random_number = rand::thread_rng().gen_range(0..4);
        match random_number {
            0 => Car::new(Side::FromEast),
            1 => Car::new(Side::FromNorth),
            2 => Car::new(Side::FromSouth),
            _ => Car::new(Side::FromWest),
        }
    }
    pub fn moove(&mut self, feu: TrafficLight) {
        match self.side {
            Side::FromEast => {
                if self.x + self.velocity < OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH {
                    self.x += self.velocity;
                } else if self.x + self.velocity < OUTPUT_WIDTH / 2 - CAR_WIDTH {
                    if feu.color == Light::Green {
                        self.x += self.velocity;
                    } else {
                        self.x = OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.x = OUTPUT_WIDTH / 2;
                            self.y -= self.velocity;
                        }
                        Direction::Right => {
                            self.x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                            self.y += self.velocity;
                        }
                        Direction::Straight => {
                            self.x += self.velocity;
                        }
                    }
                }
            }
            Side::FromNorth => {
                if self.y + self.velocity < OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT {
                    self.y += self.velocity;
                } else if self.y + self.velocity < OUTPUT_HEIGHT / 2 - CAR_HEIGHT {
                    if feu.color == Light::Green {
                        self.y += self.velocity;
                    } else {
                        self.y = OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.y = OUTPUT_HEIGHT / 2;
                            self.x += self.velocity;
                        }
                        Direction::Right => {
                            self.y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                            self.x -= self.velocity;
                        }
                        Direction::Straight => {
                            self.y += self.velocity;
                        }
                    }
                }
            }
            Side::FromWest => {
                if self.x - self.velocity > OUTPUT_WIDTH / 2 + 2 * CAR_WIDTH {
                    self.x -= self.velocity;
                } else if self.x - self.velocity > OUTPUT_WIDTH / 2 {
                    if feu.color == Light::Green {
                        self.x -= self.velocity;
                    } else {
                        self.x = OUTPUT_WIDTH / 2 + CAR_WIDTH;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.x = OUTPUT_WIDTH / 2 - CAR_WIDTH;
                            self.y += self.velocity;
                        }
                        Direction::Right => {
                            self.x = OUTPUT_WIDTH / 2;
                            self.y -= self.velocity;
                        }
                        Direction::Straight => {
                            self.x -= self.velocity;
                        }
                    }
                }
            }
            Side::FromSouth => {
                if self.y - self.velocity > OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT {
                    self.y -= self.velocity;
                } else if self.y - self.velocity > OUTPUT_HEIGHT / 2 {
                    if feu.color == Light::Green {
                        self.y -= self.velocity;
                    } else {
                        self.y = OUTPUT_HEIGHT / 2 + CAR_HEIGHT;
                    }
                } else {
                    match self.direction {
                        Direction::Left => {
                            self.y = OUTPUT_HEIGHT / 2 - CAR_HEIGHT;
                            self.x -= self.velocity;
                        }
                        Direction::Right => {
                            self.y = OUTPUT_HEIGHT / 2;
                            self.x += self.velocity;
                        }
                        Direction::Straight => {
                            self.y -= self.velocity;
                        }
                    }
                }
            }
        }
    }
}
