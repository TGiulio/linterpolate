pub struct Line {
    m: f64,
    q: f64
}

pub fn get_slope(x1: f64, y1: f64, x2: f64, y2: f64) -> Option<f64> {
    if x2 == x1{
        return None;
    }
    if x2 > x1 {
        Some((y2 - y1)/(x2 - x1))
    }else{
        Some((y1 - y2)/(x1 - x2))
    }
}

pub fn get_intercept(x: f64, y: f64, slope: f64) -> f64{
    y-(x*slope)
}

pub fn get_line_from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> Option<Line> {
    let mut line: Line = Line {m: 0.0, q: 0.0};
    match get_slope(x1, y1, x2, y2) {
        Some(slope) => {
            line.m = slope;
            line.q = get_intercept(x1, y1, slope);
        },
        None => return None,
    }
    Some(line)
}

pub fn get_line_from_point_and_slope(x: f64, y: f64, slope: f64) -> Line {
    Line {m: slope, q: get_intercept(x, y, slope)}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_slope_test() {
        let test_cases = vec![
            (1.0, 1.0, 2.0, 2.0, Some(1.0), "delta x = delta y"),
            (1.0, 1.0, 2.0, 3.0, Some(2.0), "slope 2"),
            (2.0, 3.0, 1.0, 1.0, Some(2.0), "slope 2 with inverted x"),
            (1.0, 1.0, 3.0, 2.0, Some(1.0/2.0), "slope 0.5"),
            (3.0, 2.0, 1.0, 1.0, Some(1.0/2.0), "slope 0.5 with inverted x"),
            (1.0, 1.0, 3.0, 0.0, Some(-1.0/2.0), "slope -0.5"),
            (3.0, 1.0, 1.0, 2.0, Some(-1.0/2.0), "slope -0.5 with inverted x"),
            (1.0, 1.0, 1.0, 2.0, None, "infinite slope")
            ];
            for (x1, y1, x2, y2, result, message) in test_cases {
                assert_eq!(get_slope(x1, y1, x2, y2), result, "test failed when testing {}", message);
            }
            
        }
        
    #[test]
    fn get_intercept_test(){
        let test_cases = vec![
            (1.0, 1.0, 1.0, 0.0, "slope 1"),
            (2.0, 2.0, 2.0, -2.0, "slope 2"),
            (2.0, 2.0, 0.5, 1.0, "slope 0.5"),
            (1.0, 1.0, -1.0, 2.0, "slope -1"),
            (2.0, 2.0, -2.0, 6.0, "slope -2"),
            (2.0, 2.0, -0.5, 3.0, "slope -0.5")
            ];
        for (x, y, slope, result, message) in test_cases {
            assert_eq!(get_intercept(x, y, slope), result, "test failed when testing {}", message);
        }
    }

    #[test]
    fn get_line_from_point_and_slope_test() {
        
    }
}
