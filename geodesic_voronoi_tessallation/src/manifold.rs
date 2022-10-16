pub struct Manifold {
    points: [[f64; 50]; 50],
    max: (f64, usize, usize),
    min: (f64, usize, usize),
}

impl Manifold {
    pub fn new() -> Manifold {
        let grid = [[0.0; 50]; 50];
        Self {
            points: grid,
            max: (0.0, 0, 0),
            min: (0.0, 0, 0),
        }
    }

    pub fn update_point(&mut self, index_x: usize, index_y: usize, value: f64) {
        self.points[index_x][index_y] = value;
        if value > self.max.0 {
            self.max = (value, index_x, index_y);
        }

        if value < self.min.0 {
            self.min = (value, index_x, index_y);
        }
    }

    pub fn max(&self) -> (f64, usize, usize) {
        self.max
    }

    pub fn min(&self) -> (f64, usize, usize) {
        self.min
    }

    pub fn get_point(&self, index_x: usize, index_y: usize) -> f64 {
        self.points[index_x][index_y]
    }

    pub fn bded_avg(&self) -> f64 {
        let mut total: f64 = 0.0;
        for x in [0, 49] {
            for y in 0..50 {
                total = total + self.get_point(x, y);
                total = total + self.get_point(y, x);
            }
        }

        total = total / 196.0;
        total
    }

    pub fn ball_around_x(&self, x1: usize, y1: usize, radius: usize) -> Vec<(u8, i64, f64)> {
        let mut y_and_func_vals: Vec<(u8, i64, f64)> = Vec::new();
        let mut index_start = 0;
        if x1 as i32 - radius as i32 > 0 {
            index_start = x1 - radius;
        }
        for x in index_start..(x1 + radius) {
            let positive_root =
                (f64::sqrt(f64::powi(radius as f64, 2) - (x as f64 - f64::powi(x1 as f64, 2)))
                    + (y1 as f64))
                    .round() as i64;
            let negative_root =
                (-f64::sqrt(f64::powi(radius as f64, 2) - (x as f64 - f64::powi(x1 as f64, 2)))
                    + (y1 as f64))
                    .round() as i64;
            if positive_root == negative_root {
                if x <= 50 {
                    if positive_root >= 0 || positive_root <= 5 {
                        y_and_func_vals.push((
                            x.try_into().unwrap(),
                            positive_root,
                            self.get_point(x, positive_root.try_into().unwrap()),
                        ));
                    }
                }
            } else {
                if x <= 50 {
                    if positive_root >= 0 && positive_root <= 5 {
                        y_and_func_vals.push((
                            x.try_into().unwrap(),
                            positive_root,
                            self.get_point(x, positive_root as usize),
                        ));
                    }

                    if negative_root >= 0 && negative_root <= 5 {
                        y_and_func_vals.push((
                            x.try_into().unwrap(),
                            negative_root,
                            self.get_point(x, negative_root.try_into().unwrap()),
                        ));
                    }
                }
            }
        }

        y_and_func_vals
    }

    /*
     * DEPRECRATED
     * no time to fix
    pub fn ball_around_y(&self, x1: usize, y1: usize, radius: usize) -> Vec<(u8, i64, f64)> {
        let mut x_and_func_vals: Vec<(u8, i64, f64)> = Vec::new();
        for y in (y1 - radius)..(y1 + radius) {
            let positive_root =
                (f64::sqrt(f64::powi(radius as f64, 2) - (y as f64 - f64::powi(y1 as f64, 2)))
                    + (x1 as f64))
                    .round() as i64;
            let negative_root =
                (-f64::sqrt(f64::powi(radius as f64, 2) - (y as f64 - f64::powi(y1 as f64, 2)))
                    + (x1 as f64))
                    .round() as i64;
            if positive_root == negative_root {
                x_and_func_vals.push((
                    y.try_into().unwrap(),
                    positive_root,
                    self.get_point(positive_root.try_into().unwrap(), y),
                ));
            } else {
                x_and_func_vals.push((
                    y.try_into().unwrap(),
                    positive_root,
                    self.get_point(positive_root.try_into().unwrap(), y),
                ));
                x_and_func_vals.push((
                    y.try_into().unwrap(),
                    negative_root,
                    self.get_point(negative_root.try_into().unwrap(), y),
                ));
            }
        }

        x_and_func_vals
    }
    */
}
