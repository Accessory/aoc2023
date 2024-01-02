use utils::{get_input_path, parse_file_into};

#[derive(Debug)]
struct Line {
    x: f64,
    y: f64,
    z: f64,
    v_x: f64,
    v_y: f64,
    v_z: f64,
    m: f64,
    b: f64,
}

impl From<String> for Line {
    fn from(value: String) -> Self {
        let mut split = value.split('@');
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        let mut left_split = left.split([',']);
        let mut right_split = right.split([',']);
        let x = left_split.next().unwrap().trim().parse().unwrap();
        let y = left_split.next().unwrap().trim().parse().unwrap();
        let z = left_split.next().unwrap().trim().parse().unwrap();

        let v_x = right_split.next().unwrap().trim().parse().unwrap();
        let v_y = right_split.next().unwrap().trim().parse().unwrap();
        let v_z = right_split.next().unwrap().trim().parse().unwrap();
        let m = v_y / v_x;
        let b = y - m * x;
        Self {
            x,
            y,
            z,
            v_x,
            v_y,
            v_z,
            m,
            b,
        }
    }
}

fn intersect_in(line1: &Line, line2: &Line, min: f64, max: f64) -> bool {
    if line1.m == line2.m {
        // println!("Parallel");
        return false;
    }

    let x = (line2.b - line1.b) / (line1.m - line2.m);
    let y = line1.m * x + line1.b;

    let intersect = x >= min && y >= min && x <= max && y <= max;

    if intersect {
        let in_past_x_1 = (x - line1.x) / line1.v_x;
        let in_past_y_1 = (y - line1.y) / line1.v_y;
        if in_past_x_1 < 0.0 || in_past_y_1 < 0.0 {
            // print!(" Intersection is in the past.");
            return false;
        }
        let in_past_x_2 = (x - line2.x) / line2.v_x;
        let in_past_y_2 = (y - line2.y) / line2.v_y;
        if in_past_x_2 < 0.0 || in_past_y_2 < 0.0 {
            // print!(" Intersection is in the past.");
            return false;
        }
    }

    intersect
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const MIN: f64 = 7.0;
    #[cfg(test)]
    const MAX: f64 = 27.0;
    #[cfg(not(test))]
    const MIN: f64 = 200000000000000.0;
    #[cfg(not(test))]
    const MAX: f64 = 400000000000000.0;
    let mut result: usize = 0;
    // Parse
    let values: Vec<Line> = parse_file_into(input_file);

    for (i, line1) in values.iter().enumerate() {
        for line2 in values.iter().skip(i + 1) {
            if intersect_in(line1, line2, MIN, MAX) {
                result += 1;
                // println!("Intersection of \nl1: {:?} and \nl2: {:?}", line1, line1);
            }
        }
    }

    // Solve
    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // Parse
    let lines: Vec<Line> = parse_file_into(input_file);

    // Calculations need the range of `i128`.
    let widen = |i: usize| {
        [
            lines[i].x as i128,
            lines[i].y as i128,
            lines[i].z as i128,
            lines[i].v_x as i128,
            lines[i].v_y as i128,
            lines[i].v_z as i128,
        ]
    };
    let [a, b, c, d, e, f] = widen(0);
    let [g, h, i, j, k, l] = widen(1);
    let [m, n, o, p, q, r] = widen(2);

    // Coefficients for the 6 simulataneous linear equations.
    // Columns are px, py, pz, vx, vy, vz of the rock equal to a constant.
    let mut matrix = [
        [
            0,
            l - f,
            e - k,
            0,
            c - i,
            h - b,
            e * c - b * f + h * l - k * i,
        ],
        [
            0,
            r - f,
            e - q,
            0,
            c - o,
            n - b,
            e * c - b * f + n * r - q * o,
        ],
        [
            f - l,
            0,
            j - d,
            i - c,
            0,
            a - g,
            a * f - d * c + j * i - g * l,
        ],
        [
            f - r,
            0,
            p - d,
            o - c,
            0,
            a - m,
            a * f - d * c + p * o - m * r,
        ],
        [
            k - e,
            d - j,
            0,
            b - h,
            g - a,
            0,
            d * b - a * e + g * k - j * h,
        ],
        [
            q - e,
            d - p,
            0,
            b - n,
            m - a,
            0,
            d * b - a * e + m * q - p * n,
        ],
    ];

    // Use Gaussian elimination to solve for the 6 unknowns.
    // Forward elimination, processing columns from left to right.
    // This will leave a matrix in row echelon form.
    for pivot in 0..6 {
        // Make leading coefficient of each row positive to make subsequent calculations easier.
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                // Flip signs of each coefficient.
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            // Reduce by GCD each time otherwise coefficients will overflow even a `i128`.
            for row in &mut matrix[pivot..] {
                let mut factor = 0;

                for &next in &row[pivot..] {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.abs();
                        } else {
                            factor = gcd(factor, next.abs());
                        }
                    }
                }

                row[pivot..].iter_mut().for_each(|c| *c /= factor);
            }

            let column = matrix.map(|row| row[pivot]);

            // If only one non-zero coefficient remaining in the column then we're done.
            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                // Move this row into the pivot location
                let index = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            // Find the row with the lowest non-zero leading coefficient.
            let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index = column.iter().rposition(|&c| c == min).unwrap();

            // Subtract as many multiples of this minimum row from each other row as possible
            // to shrink the coefficients of our column towards zero.
            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor = column[row] / min;

                    for col in pivot..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    // Back substitution, processing columns from right to left.
    // This will leave the matrix in reduced row echelon form.
    // The solved unknowns are then in the 7th column.
    for pivot in (0..6).rev() {
        // We're explicitly told that the results are integers so integer division is safe
        // and will not mangle result.
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    // x + y + z
    let result = matrix[0][6] + matrix[1][6] + matrix[2][6];
    println!("Result of part 2 is {}", result);
}

fn gcd(mut factor: i128, mut b: i128) -> i128 {
    while b != 0 {
        (factor, b) = (b, factor % b);
    }
    factor
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
