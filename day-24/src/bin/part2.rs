#![allow(dead_code)]
use core::f64;
use std::str::FromStr;

use anyhow::{anyhow, Error};
use z3::{ast::Ast, Config, Context, SatResult, Solver};
fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

fn process(input: &str) -> i64 {
    let hailstones = parse_input(input);

    let pos = find_rock_position(&hailstones);
    pos.iter().sum()
}

// The problem is an example of a system of quadratic Diophantine equations, see
// https://en.wikipedia.org/wiki/Diophantine_equation. We are interested in integer solutions only.
// We can use one of the available symbolic equations prover/checker,
// https://github.com/Z3Prover/z3. If there is a solution, all the constraints imposed by the
// problem must be satisfied, in particular that eventually all hailstones will meet a rock on
// integer coordinates.
//
// If (x,y,z) is rock initial position, (v_x, v_y, v_z) is rock velocity, (x_i, y_i, z_i) is
// hailstone initial position, (v_ix, v_iy, v_iz) is hailstone velocity, N_i is hailstone-specific
// number steps required to meet the rock, and i = 1,n where n is the number of hailstones.
// x + N_i * v_x - x_i - N_i * v_ix = 0
// y + N_i * v_y - y_i - N_i * v_iy = 0
// z + N_i * v_z - z_i - N_i * v_iz = 0
//
// This is a system of 3*n equations over n + 6 integer unknowns. The system is overcomplete, and
// therefore should have at most one solution, which this problem asks to find.
fn find_rock_position(hailstones: &[Hailstone]) -> [i64; 3] {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = z3::ast::Int::new_const(&ctx, "x");
    let y = z3::ast::Int::new_const(&ctx, "y");
    let z = z3::ast::Int::new_const(&ctx, "z");
    let vx = z3::ast::Int::new_const(&ctx, "vx");
    let vy = z3::ast::Int::new_const(&ctx, "vy");
    let vz = z3::ast::Int::new_const(&ctx, "vz");
    let steps: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::new_const(&ctx, format!("N{i}")))
        .collect();

    // hailstone positions
    let hail_pos_x: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].pos[0] as i64))
        .collect();
    let hail_pos_y: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].pos[1] as i64))
        .collect();
    let hail_pos_z: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].pos[2] as i64))
        .collect();

    // hailstone velocities
    let hail_vel_x: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].vel[0] as i64))
        .collect();
    let hail_vel_y: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].vel[1] as i64))
        .collect();
    let hail_vel_z: Vec<_> = (0..hailstones.len())
        .map(|i| z3::ast::Int::from_i64(&ctx, hailstones[i].vel[2] as i64))
        .collect();

    let solver = Solver::new(&ctx);
    let zero = z3::ast::Int::from_i64(&ctx, 0);
    (0..hailstones.len()).for_each(|i| {
        // x
        let rock_offset_x = z3::ast::Int::mul(&ctx, &[&vx, &steps[i]]);
        let rock_pos_x = z3::ast::Int::add(&ctx, &[&x, &rock_offset_x]);
        let hail_offset_x = z3::ast::Int::mul(&ctx, &[&hail_vel_x[i], &steps[i]]);
        let hail_pos_x = z3::ast::Int::add(&ctx, &[&hail_pos_x[i], &hail_offset_x]);
        let diff_x = z3::ast::Int::sub(&ctx, &[&rock_pos_x, &hail_pos_x]);
        solver.assert(&diff_x._eq(&zero));

        // y
        let rock_offset_y = z3::ast::Int::mul(&ctx, &[&vy, &steps[i]]);
        let rock_pos_y = z3::ast::Int::add(&ctx, &[&y, &rock_offset_y]);
        let hail_offset_y = z3::ast::Int::mul(&ctx, &[&hail_vel_y[i], &steps[i]]);
        let hail_pos_y = z3::ast::Int::add(&ctx, &[&hail_pos_y[i], &hail_offset_y]);
        let diff_y = z3::ast::Int::sub(&ctx, &[&rock_pos_y, &hail_pos_y]);
        solver.assert(&diff_y._eq(&zero));

        // y
        let rock_offset_z = z3::ast::Int::mul(&ctx, &[&vz, &steps[i]]);
        let rock_pos_z = z3::ast::Int::add(&ctx, &[&z, &rock_offset_z]);
        let hail_offset_z = z3::ast::Int::mul(&ctx, &[&hail_vel_z[i], &steps[i]]);
        let hail_pos_z = z3::ast::Int::add(&ctx, &[&hail_pos_z[i], &hail_offset_z]);
        let diff_z = z3::ast::Int::sub(&ctx, &[&rock_pos_z, &hail_pos_z]);
        solver.assert(&diff_z._eq(&zero));
    });

    let result = solver.check();
    assert_eq!(result, SatResult::Sat);

    let model = solver.get_model().unwrap();
    let x_val = model.eval(&x, true).unwrap().as_i64().unwrap();
    let y_val = model.eval(&y, true).unwrap().as_i64().unwrap();
    let z_val = model.eval(&z, true).unwrap().as_i64().unwrap();
    [x_val, y_val, z_val]
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => {
                Some(trimmed.parse().unwrap_or_else(|err| panic!("{err}")))
            }
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Hailstone {
    fn intersects(&self, other: &Hailstone, min_c: isize, max_c: isize) -> bool {
        if self.is_parallel(other) {
            return false;
        }

        // find intersection point and check that it is in bounds
        let (slope, offset) = self.xy_trajectory();
        let (other_slope, other_offset) = other.xy_trajectory();

        let x_i = (other_offset - offset) / (slope - other_slope);
        let y_i = slope * x_i + offset;
        let min_bound = min_c as f64;
        let max_bound = max_c as f64;
        let num_steps = (x_i - self.pos[0] as f64) / self.vel[0] as f64;
        let num_steps_other = (x_i - other.pos[0] as f64) / other.vel[0] as f64;

        if num_steps < 0.0 || num_steps_other < 0.0 {
            return false;
        }

        (x_i >= min_bound && x_i <= max_bound) && (y_i >= min_bound && y_i <= max_bound)
    }

    fn is_parallel(&self, other: &Hailstone) -> bool {
        let cross = self.vel[0] * other.vel[1] - self.vel[1] * other.vel[0];
        cross == 0
    }

    fn xy_trajectory(&self) -> (f64, f64) {
        let slope: f64 = self.vel[1] as f64 / self.vel[0] as f64;
        let offset = self.pos[1] as f64 - slope * (self.pos[0] as f64);
        (slope, offset)
    }
}

impl FromStr for Hailstone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((pos_str, vel_str)) = s.split_once(" @ ") {
            let mut pos: [isize; 3] = [0; 3];
            for (idx, pos_c) in pos_str.split(',').enumerate() {
                pos[idx] = pos_c.trim().parse::<isize>()?;
            }

            let mut vel: [isize; 3] = [0; 3];
            for (idx, vel_c) in vel_str.split(',').enumerate() {
                vel[idx] = vel_c.trim().parse::<isize>()?;
            }
            return Ok(Self { pos, vel });
        }
        Err(anyhow!("Failed to parse hailstone: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "#;
        assert_eq!(47, process(input));
    }
}
