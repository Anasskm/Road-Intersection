use super::car::*;
use super::consts::*;
use super::se_base::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Road {
    pub cars_before_stop_north: Vec<Car>,
    pub cars_before_stop_south: Vec<Car>,
    pub cars_before_stop_east: Vec<Car>,
    pub cars_before_stop_west: Vec<Car>,
    pub cars_in_intersection: Vec<Car>,
    pub cars_after_stop_north: Vec<Car>,
    pub cars_after_stop_south: Vec<Car>,
    pub cars_after_stop_east: Vec<Car>,
    pub cars_after_stop_west: Vec<Car>,
    pub north_lights: TrafficLight,
    pub east_lights: TrafficLight,
    pub south_lights: TrafficLight,
    pub west_lights: TrafficLight,
}

impl Road {
    pub fn new() -> Road {
        Road {
            cars_before_stop_north: vec![],
            cars_before_stop_south: vec![],
            cars_before_stop_east: vec![],
            cars_before_stop_west: vec![],
            cars_in_intersection: vec![],
            cars_after_stop_north: vec![],
            cars_after_stop_south: vec![],
            cars_after_stop_east: vec![],
            cars_after_stop_west: vec![],
            north_lights: TrafficLight { color: Light::Red },
            east_lights: TrafficLight { color: Light::Red },
            south_lights: TrafficLight { color: Light::Red },
            west_lights: TrafficLight { color: Light::Red },
        }
    }
    pub fn simulation_loop(&mut self) {
        if self.south_lights.color == Light::Green
            || self.north_lights.color == Light::Green
            || self.east_lights.color == Light::Green
            || self.west_lights.color == Light::Green
        {
            self.east_lights = TrafficLight { color: Light::Red };
            self.north_lights = TrafficLight { color: Light::Red };
            self.south_lights = TrafficLight { color: Light::Red };
            self.west_lights = TrafficLight { color: Light::Red };
        } else {
            let a = self.cars_before_stop_east.len();
            let b = self.cars_before_stop_west.len();
            let c = self.cars_before_stop_north.len();
            let d = self.cars_before_stop_south.len();
            if a >= b && a >= c && a >= d {
                self.east_lights = TrafficLight {
                    color: Light::Green,
                };
                self.north_lights = TrafficLight { color: Light::Red };
                self.south_lights = TrafficLight { color: Light::Red };
                self.west_lights = TrafficLight { color: Light::Red };
            } else {
                if b >= a && b >= c && b >= d {
                    self.east_lights = TrafficLight { color: Light::Red };
                    self.north_lights = TrafficLight { color: Light::Red };
                    self.south_lights = TrafficLight { color: Light::Red };
                    self.west_lights = TrafficLight {
                        color: Light::Green,
                    };
                } else {
                    if c >= a && c >= b && c >= d {
                        self.east_lights = TrafficLight { color: Light::Red };
                        self.north_lights = TrafficLight {
                            color: Light::Green,
                        };
                        self.south_lights = TrafficLight { color: Light::Red };
                        self.west_lights = TrafficLight { color: Light::Red };
                    } else {
                        self.east_lights = TrafficLight { color: Light::Red };
                        self.north_lights = TrafficLight { color: Light::Red };
                        self.south_lights = TrafficLight {
                            color: Light::Green,
                        };
                        self.west_lights = TrafficLight { color: Light::Red };
                    }
                }
            }
        }

        if self.cars_in_intersection.len() != 0 {
            self.cars_in_intersection[0].moove(TrafficLight {
                color: Light::Green,
            });
            if (self.cars_in_intersection[0].x > OUTPUT_WIDTH / 2 + CAR_WIDTH
                || self.cars_in_intersection[0].x < OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH)
                || (self.cars_in_intersection[0].y > OUTPUT_HEIGHT / 2 + CAR_HEIGHT
                    || self.cars_in_intersection[0].y < OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT)
            {
                if (self.cars_in_intersection[0].side == Side::FromEast
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromSouth
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromWest
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_north
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromSouth
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromWest
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromNorth
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_east
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromWest
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromNorth
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromEast
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_south
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                } else if (self.cars_in_intersection[0].side == Side::FromNorth
                    && self.cars_in_intersection[0].direction == Direction::Left)
                    || (self.cars_in_intersection[0].side == Side::FromEast
                        && self.cars_in_intersection[0].direction == Direction::Straight)
                    || (self.cars_in_intersection[0].side == Side::FromSouth
                        && self.cars_in_intersection[0].direction == Direction::Right)
                {
                    self.cars_after_stop_west
                        .push(self.cars_in_intersection[0].clone());
                    self.cars_in_intersection.pop();
                }
            }
        }
        if self.cars_after_stop_east.len() > 0 {
            self.cars_after_stop_east[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_east[0].clone();
            for i in 1..self.cars_after_stop_east.len() {
                if self.cars_after_stop_east[i].x - self.cars_after_stop_east[i].velocity
                    > front_car.x + SECURITY_DISTANCE
                {
                    self.cars_after_stop_east[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_east[i].clone();
                }
            }
        }
        if self.cars_after_stop_north.len() > 0 {
            self.cars_after_stop_north[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_north[0].clone();
            for i in 1..self.cars_after_stop_north.len() {
                if self.cars_after_stop_north[i].y - self.cars_after_stop_north[i].velocity
                    > front_car.y + SECURITY_DISTANCE
                {
                    self.cars_after_stop_north[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_north[i].clone();
                }
            }
        }
        if self.cars_after_stop_west.len() > 0 {
            self.cars_after_stop_west[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_west[0].clone();
            for i in 1..self.cars_after_stop_west.len() {
                if self.cars_after_stop_west[i].x + self.cars_after_stop_west[i].velocity
                    < front_car.x - SECURITY_DISTANCE
                {
                    self.cars_after_stop_west[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_west[i].clone();
                }
            }
        }
        if self.cars_after_stop_south.len() > 0 {
            self.cars_after_stop_south[0].moove(TrafficLight {
                color: Light::Green,
            });
            let mut front_car = self.cars_after_stop_south[0].clone();
            for i in 1..self.cars_after_stop_south.len() {
                if self.cars_after_stop_south[i].y + self.cars_after_stop_south[i].velocity
                    < front_car.y - SECURITY_DISTANCE
                {
                    self.cars_after_stop_south[i].moove(TrafficLight {
                        color: Light::Green,
                    });
                    front_car = self.cars_after_stop_south[i].clone();
                }
            }
        }
        if self.cars_before_stop_north.len() > 0 {
            if self.cars_before_stop_north[0].y < (OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT) {
                self.cars_before_stop_north[0].moove(self.north_lights.clone());

                let mut front_car_y = self.cars_before_stop_north[0].y;
                for i in 1..self.cars_before_stop_north.len() {
                    if self.cars_before_stop_north[i].y + self.cars_before_stop_north[i].velocity
                        < front_car_y - SECURITY_DISTANCE
                    {
                        self.cars_before_stop_north[i].moove(self.north_lights.clone());
                        front_car_y = self.cars_before_stop_north[i].y;
                    }
                }
            }
            if self.cars_before_stop_north[0].y >= (OUTPUT_HEIGHT / 2 - 2 * CAR_HEIGHT)
                && self.north_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_north[0].clone());
                    self.cars_before_stop_north.remove(0);
                }
            }
        }
        if self.cars_before_stop_south.len() > 0 {
            if self.cars_before_stop_south[0].y > (OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT) {
                self.cars_before_stop_south[0].moove(self.south_lights.clone());

                let mut front_car_y = self.cars_before_stop_south[0].y;
                for i in 1..self.cars_before_stop_south.len() {
                    if self.cars_before_stop_south[i].y - self.cars_before_stop_south[i].velocity
                        > front_car_y + SECURITY_DISTANCE
                    {
                        self.cars_before_stop_south[i].moove(self.south_lights.clone());
                        front_car_y = self.cars_before_stop_south[i].y;
                    }
                }
            }
            if self.cars_before_stop_south[0].y <= (OUTPUT_HEIGHT / 2 + 2 * CAR_HEIGHT)
                && self.south_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_south[0].clone());
                    self.cars_before_stop_south.remove(0);
                }
            }
        }
        if self.cars_before_stop_east.len() > 0 {
            if self.cars_before_stop_east[0].x < (OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH) {
                self.cars_before_stop_east[0].moove(self.east_lights.clone());

                let mut front_car_x = self.cars_before_stop_east[0].x;
                for i in 1..self.cars_before_stop_east.len() {
                    if self.cars_before_stop_east[i].x + self.cars_before_stop_east[i].velocity
                        < front_car_x - SECURITY_DISTANCE
                    {
                        self.cars_before_stop_east[i].moove(self.east_lights.clone());
                        front_car_x = self.cars_before_stop_east[i].x;
                    }
                }
            }
            if self.cars_before_stop_east[0].x >= (OUTPUT_WIDTH / 2 - 2 * CAR_WIDTH)
                && self.east_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_east[0].clone());
                    self.cars_before_stop_east.remove(0);
                }
            }
        }
        if self.cars_before_stop_west.len() > 0 {
            if self.cars_before_stop_west[0].x > (OUTPUT_WIDTH / 2 + CAR_WIDTH) {
                self.cars_before_stop_west[0].moove(self.west_lights.clone());

                let mut front_car_x = self.cars_before_stop_west[0].x;
                for i in 1..self.cars_before_stop_west.len() {
                    if self.cars_before_stop_west[i].x - self.cars_before_stop_west[i].velocity
                        > front_car_x + SECURITY_DISTANCE
                    {
                        self.cars_before_stop_west[i].moove(self.west_lights.clone());
                        front_car_x = self.cars_before_stop_west[i].x;
                    }
                }
            }
            if self.cars_before_stop_west[0].x <= (OUTPUT_WIDTH / 2 + CAR_WIDTH)
                && self.west_lights.color == Light::Green
            {
                if self.cars_in_intersection.len() == 0 {
                    self.cars_in_intersection
                        .push(self.cars_before_stop_west[0].clone());
                    self.cars_before_stop_west.remove(0);
                }
            }
        }
    }
}
