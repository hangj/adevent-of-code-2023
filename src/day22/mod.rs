use std::{str::FromStr, ops::RangeInclusive, collections::{HashMap, HashSet}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",")
            .map(|s|s.parse::<i32>().unwrap());
        let x = it.next().unwrap();
        let y = it.next().unwrap();
        let z = it.next().unwrap();
        Ok(Pos::new(x, y, z))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("~");
        let start = it.next().unwrap();
        let end = it.next().unwrap();

        let start = start.parse::<Pos>().unwrap();
        let end = end.parse::<Pos>().unwrap();
        Ok(Self {
            x: start.x ..= end.x,
            y: start.y ..= end.y,
            z: start.z ..= end.z,
        })
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.z.start().partial_cmp(&other.z.start()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.z.end().partial_cmp(&other.z.end()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.start().partial_cmp(&other.x.start()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.end().partial_cmp(&other.x.end()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.y.start().partial_cmp(&other.y.start()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.end().partial_cmp(&other.y.end())
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Ground {
    /// (x, y) -> z
    /// 俯视, 当前 (x, y) 的位置上的地面 z 坐标
    state: HashMap<(i32, i32), i32>,
    id: HashMap<(i32, i32), usize>,
}

impl Ground {
    fn new() -> Self {
        Self { state: HashMap::new(), id: HashMap::new() }
    }
    fn get_z(&mut self, x: i32, y: i32) -> i32 {
        if !self.state.contains_key(&(x, y)) {
            self.state.insert((x, y), 0);
        }
        *self.state.get(&(x, y)).unwrap()
    }
    fn set_z(&mut self, x: i32, y: i32, z: i32) {
        self.state.insert((x, y), z);
    }
    fn get_id(&self, x: i32, y: i32) -> Option<usize> {
        self.id.get(&(x, y)).map(|x|*x)
    }
    fn set_id(&mut self, x: i32, y: i32, id: usize) {
        self.id.insert((x, y), id);
    }
}

#[derive(Debug, Default)]
struct Node {
    up: HashSet<usize>,
    down: HashSet<usize>,
}

#[test]
fn test() {
    let input = {"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9".trim()
    };

    let input = include_str!("input");

    let mut bricks = input.split("\n")
        .map(|line| line.parse::<Brick>().unwrap())
        .collect::<Vec<_>>();

    let origin = bricks.clone();

    bricks.sort();

    println!("{}", origin == bricks);

    let mut up_to_down = HashMap::new();

    let mut ground = Ground::new();

    for (id, brick) in bricks.iter_mut().enumerate() {
        let z = *brick.z.start();
        let z_end = *brick.z.end();

        let mut max_ground = 0;
        let mut occupied = HashSet::new();

        for x in brick.x.clone() {
            for y in brick.y.clone() {
                let g = ground.get_z(x, y);
                if g != 0 && g < z {
                    if g == max_ground {
                        occupied.insert(ground.get_id(x, y).unwrap());
                    }
                    else if g > max_ground {
                        max_ground = g;
                        occupied.clear();
                        occupied.insert(ground.get_id(x, y).unwrap());
                    }
                }
            }
        }

        // 降落
        let new_z = max_ground + 1;
        let z_end = z_end - (z - new_z);
        brick.z = new_z ..= z_end;

        for x in brick.x.clone() {
            for y in brick.y.clone() {
                ground.set_z(x, y, z_end);
                ground.set_id(x, y, id);
            }
        }

        up_to_down.insert(id, occupied);
    }

    let mut nodes = HashMap::new();

    up_to_down.iter()
        .for_each(|(id, p)| {
            if !nodes.contains_key(id) {
                nodes.insert(*id, Node::default());
            }

            p.iter().for_each(|p_id| {
                nodes.get_mut(id).unwrap().down.insert(*p_id);

                if !nodes.contains_key(p_id) {
                    nodes.insert(*p_id, Node::default());
                }

                nodes.get_mut(p_id).unwrap().up.insert(*id);
            });
        });

    
    let mut can_be_disintergrated = HashSet::new();

    nodes.iter()
        .for_each(|(id, node)| {
            // If no upper bricks depend on me, then I can be disintegrated
            if node.up.is_empty() {
                can_be_disintergrated.insert(*id);
            }
            let mut without_id = HashSet::new();
            without_id.insert(*id);
            // If all my upper bricks can find a way down to the ground without me, then I'm ok to be disintergrated
            if node.up.iter().all(|up_id| {
                can_find_a_way_down(&bricks, &nodes, *up_id, &without_id)
            }) {
                can_be_disintergrated.insert(*id);
            }
        });

    println!("{}", can_be_disintergrated.len());

    // Part Two
    let sum = nodes.iter()
        .filter(|(id, _)| !can_be_disintergrated.contains(*id))
        .map(|(id, _)| {
            let without_id = HashSet::new();
            let mut falling_ids = HashSet::new();
            calc_fall_count(&bricks, &nodes, *id, without_id, &mut falling_ids);
            println!("{}: {:?}", id, falling_ids);
            falling_ids.len()
        })
        .sum::<usize>();

    println!("sum: {}", sum);

}

fn can_find_a_way_down(bricks: &Vec<Brick>, nodes: &HashMap<usize, Node>, id: usize, without_id: &HashSet<usize>) -> bool {
    if without_id.contains(&id) {
        return false;
    }
    // 已经在地面上了
    if *bricks.get(id).unwrap().z.start() == 1 {
        return true;
    }

    let node = nodes.get(&id).unwrap();
    if node.down.is_empty() {
        return false;
    }
    node.down.iter()
        .any(|id| can_find_a_way_down(bricks, nodes, *id, without_id))
}

/// calculate the number of other bricks that would fall
fn calc_fall_count(bricks: &Vec<Brick>, nodes: &HashMap<usize, Node>, id: usize, mut without_id: HashSet<usize>, falling_ids: &mut HashSet<usize>) {
    let node = nodes.get(&id).unwrap();
    if node.up.is_empty() {
        return;
    }
    without_id.insert(id);

    let mut ids = Vec::new();

    for up_id in node.up.iter() {
        if !can_find_a_way_down(bricks, nodes, *up_id, &without_id) {
            without_id.insert(*up_id);
            falling_ids.insert(*up_id);
            ids.push(*up_id);
        }
    }

    ids.iter()
        .for_each(|up_id| {
            let without_id = without_id.clone();
            calc_fall_count(bricks, nodes, *up_id, without_id, falling_ids);
        });
}
