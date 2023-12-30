use aoc_2023_rust::Puzzle;
use num::bigint::BigInt;
use num::FromPrimitive;

#[derive(Debug, Clone)]
pub struct Day24 {
    input: Input,
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {
            input: Input::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = Input::new();
    }
}

impl Puzzle for Day24 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/24.input");
        for line in INPUT.lines() {
            let (ps, vs) = line.split_once(" @ ").unwrap();
            let p = ps
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .map(|x| BigInt::from_i128(x).unwrap())
                .collect::<Vec<_>>();
            self.input
                .positions
                .push((p[0].clone(), p[1].clone(), p[2].clone()));
            let v = vs
                .split(", ")
                .map(|x| x.trim().parse::<i128>().unwrap())
                .map(|x| BigInt::from_i128(x).unwrap())
                .collect::<Vec<_>>();
            self.input
                .velocities
                .push((v[0].clone(), v[1].clone(), v[2].clone()));
        }
    }

    fn part1(&self) -> String {
        let n = self.input.positions.len();
        let low = BigInt::from_i128(200000000000000).unwrap();
        let high = BigInt::from_i128(400000000000000).unwrap();
        let mut count = 0;
        let zero = BigInt::from(0);
        for i in 0..n {
            for j in i + 1..n {
                let p1 = self.input.positions[i].clone();
                let v1 = self.input.velocities[i].clone();
                let p2 = self.input.positions[j].clone();
                let v2 = self.input.velocities[j].clone();
                let d_ = &v1.1 * &v2.0 - &v1.0 * &v2.1;
                let t1num_ = &v2.0 * (&p2.1 - &p1.1) - &v2.1 * (&p2.0 - &p1.0);
                let t2num_ = &v1.0 * (&p2.1 - &p1.1) - &v1.1 * (&p2.0 - &p1.0);
                let d = if d_ >= zero { d_.clone() } else { -d_.clone() };
                let t1num = if d_ > zero { t1num_ } else { -t1num_ };
                let t2num = if d_ > zero { t2num_ } else { -t2num_ };

                if d > zero
                    && &t1num * &d >= zero
                    && &t2num * &d >= zero
                    && &d * &low <= &d * &p1.0 + &t1num * &v1.0
                    && &d * &high >= &d * &p1.0 + &t1num * &v1.0
                    && &d * &low <= &d * &p1.1 + &t1num * &v1.1
                    && &d * &high >= &d * &p1.1 + &t1num * &v1.1
                {
                    count += 1;
                }
            }
        }
        format!("{:?}", count)
    }

    fn part2(&self) -> String {
        let (x0, x1, x2) = self.input.positions[0].clone();
        let (v0, v1, v2) = self.input.velocities[0].clone();
        let positions = self
            .input
            .positions
            .iter()
            .map(|(x, y, z)| (x - &x0, y - &x1, z - &x2))
            .collect::<Vec<_>>();
        let velocities = self
            .input
            .velocities
            .iter()
            .map(|(x, y, z)| (x - &v0, y - &v1, z - &v2))
            .collect::<Vec<_>>();

        // Those are the coefficients of the plane that contains 0 and the line of the 2nd heil
        let point1 = positions[1].clone();
        let point2 = velocities[1].clone();
        let c = cross_product(point1, point2);

        let t2 = crossing_time(positions[2].clone(), velocities[2].clone(), c.clone());
        let p2 = crossing_position(positions[2].clone(), velocities[2].clone(), t2.clone());

        let t3 = crossing_time(positions[3].clone(), velocities[3].clone(), c);
        let p3 = crossing_position(positions[3].clone(), velocities[3].clone(), t3.clone());

        let x = (&t3 * &p2.0 - &t2 * &p3.0) / (&t3 - &t2);
        let y = (&t3 * &p2.1 - &t2 * &p3.1) / (&t3 - &t2);
        let z = (&t3 * &p2.2 - &t2 * &p3.2) / (&t3 - &t2);

        format!("{:?}", x + y + z + x0 + x1 + x2)
    }
}

fn crossing_position(x: Position, v: Velocity, t: BigInt) -> (BigInt, BigInt, BigInt) {
    (x.0 + &t * v.0, x.1 + &t * v.1, x.2 + &t * v.2)
}

fn crossing_time(x: Position, v: Velocity, c: (BigInt, BigInt, BigInt)) -> BigInt {
    let left: BigInt = dot_product(c.clone(), v);
    let right: BigInt = -dot_product(c, x);
    right / left
}

fn dot_product(a: (BigInt, BigInt, BigInt), b: (BigInt, BigInt, BigInt)) -> BigInt {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn cross_product(
    a: (BigInt, BigInt, BigInt),
    b: (BigInt, BigInt, BigInt),
) -> (BigInt, BigInt, BigInt) {
    (
        &a.1 * &b.2 - &a.2 * &b.1,
        &a.2 * &b.0 - &a.0 * &b.2,
        &a.0 * &b.1 - &a.1 * &b.0,
    )
}

type Position = (BigInt, BigInt, BigInt);
type Velocity = (BigInt, BigInt, BigInt);

#[derive(Debug, Clone)]
struct Input {
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
}

impl Input {
    fn new() -> Self {
        Input {
            positions: Vec::new(),
            velocities: Vec::new(),
        }
    }
}
