use std::{str::FromStr, ops::Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn new(x: i64, y: i64, z: i64) -> Self { Self { x, y, z } }
    fn cross(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",").map(|s|s.trim()).filter(|s|!s.is_empty());
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();
        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug, Clone, Copy)]
struct FPos {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hailstone {
    // position
    position: Pos,
    // velocity 
    velocity: Pos,
}

impl Hailstone {
    fn new(position: Pos, velocity: Pos) -> Self { Self { position, velocity } }
    /// 返回碰撞点
    fn intersect_point(&self, other: &Self) -> Option<FPos> {
        // a x b = x1 * y2 - x2 * y1
        // if < 0 then b 在 a 的顺时针方向, =0 平行, > 0 b 在 a 的逆时针方向
        let x1 = self.velocity.cross(&other.velocity);
        if x1 == 0 {
            return None;
        }
        let v = other.position - self.position;
        let x2 = self.velocity.cross(&v);

        // 符号相同
        if x1.signum() * x2.signum() >= 0 {
            return None;
        }

        let x1 = other.velocity.cross(&self.velocity);
        if x1 == 0 {
            return None;
        }
        let v = self.position - other.position;
        let x2 = other.velocity.cross(&v);

        // 符号相同
        if x1.signum() * x2.signum() >= 0 {
            return None;
        }

        // 相交
        {
            let k1 = self.velocity.y as f64 / self.velocity.x as f64;
            let n1 = self.position.y as f64 - self.position.x as f64 * k1;
            let k2 = other.velocity.y as f64 / other.velocity.x as f64;
            let n2 = other.position.y as f64 - other.position.x as f64 * k2;

            let x = (n2 - n1) / (k1 - k2);
            let y = (k1 * n2 - k2 * n1) / (k1 - k2);

            // need to check if the crossed point is in the past for self or other

            return Some(FPos{x, y, z: 0.})
        }

        None
    }
}

#[test]
fn test() {
    let input = {"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
".trim()};

    let input = include_str!("input");

    let hailstones = input.split("\n")
        .map(|s| {
            let mut it = s.split(" @ ");
            let position = it.next().unwrap().parse().unwrap();
            let velocity = it.next().unwrap().parse().unwrap();
            Hailstone::new(position, velocity)
        })
        .collect::<Vec<_>>();


    let min = 200000000000000_f64;
    let max = 400000000000000_f64;

    let mut sum = 0;

    for i in 0..hailstones.len() {
        let stone = &hailstones[i];
        for j in i+1..hailstones.len() {
            if let Some(p) = stone.intersect_point(&hailstones[j]) {
                // println!("A:{:?}, B:{:?} @ (x={},y={})", stone.position, hailstones[j].position, p.x, p.y);
                if p.x > min && p.x < max && p.y > min && p.y < max {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);

    // Part Two
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut min_z = i64::MAX;
    for stone in hailstones.iter() {
        let pos = &stone.position;
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        min_z = min_z.min(pos.z);
    }

    let min_pos = Pos::new(min_x, min_y, min_z);
    println!("{:?}", min_pos);

    let mut hailstones = hailstones;
    hailstones.iter_mut()
        .for_each(|stone| {
            let pos = stone.position - min_pos;
            stone.position = pos;
            println!("{}, {}, {} @ {}, {}, {}", pos.x, pos.y, pos.z, stone.velocity.x, stone.velocity.y, stone.velocity.z);
        });
}


