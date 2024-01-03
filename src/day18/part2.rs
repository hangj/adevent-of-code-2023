use std::{collections::{HashMap, HashSet}, fmt::Display};

use super::{Dir, DigCmd};


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vertex {
    row: i32,
    col: i32,
}

impl Vertex {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    up: Option<Vertex>,
    down: Option<Vertex>,
    left: Option<Vertex>,
    right: Option<Vertex>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        if let Some(v) = self.up {
            write!(f, "up: ({}, {}), ", v.row, v.col)?;
            count += 1;
        }
        if let Some(v) = self.down {
            write!(f, "down: ({}, {}), ", v.row, v.col)?;
            count += 1;
        }
        if let Some(v) = self.left {
            write!(f, "left: ({}, {})", v.row, v.col)?;
            count += 1;
        }
        if let Some(v) = self.right {
            write!(f, "right: ({}, {})", v.row, v.col)?;
            count += 1;
        }

        // assert_eq!(count, 2);
        Ok(())
    }
}

fn calc(cmds: Vec<DigCmd>) -> i32 {
    let mut hashes = HashMap::new();
    let mut vertex = Vertex::new(0, 0);
    hashes.insert(vertex, Node::default());

    cmds.iter()
        .for_each(|cmd| {
            let steps = cmd.steps;
            let last_vertex = vertex;
            let node = hashes.get_mut(&vertex).unwrap();
            match cmd.dir {
                Dir::Up => {
                    vertex.row -= steps;
                    node.up = Some(vertex);
                },
                Dir::Down => {
                    vertex.row += steps;
                    node.down = Some(vertex);
                },
                Dir::Left => {
                    vertex.col -= steps;
                    node.left = Some(vertex);
                },
                Dir::Right => {
                    vertex.col += steps;
                    node.right = Some(vertex);
                },
            }
            if hashes.get(&vertex).is_none() {
                hashes.insert(vertex, Node::default());
            }
            let new_node = hashes.get_mut(&vertex).unwrap();
            match cmd.dir {
                Dir::Up => new_node.down = Some(last_vertex),
                Dir::Down => new_node.up = Some(last_vertex),
                Dir::Left => new_node.right = Some(last_vertex),
                Dir::Right => new_node.left = Some(last_vertex),
            }
        });

    let mut round = 0;
    let mut sum = 0;
    println!("round {}: {}", round, sum);
    // for (k, v) in hashes.iter() {
    //     println!("{k}: {v}");
    // }

    loop {
        if hashes.is_empty() {break;}
        sum += do_calc(&mut hashes);

        round += 1;
        println!("round {}: {}", round, sum);
        // for (k, v) in hashes.iter() {
        //     println!("{k}: {v}");
        // }
    }
    sum
}

fn do_calc(hashes: &mut HashMap<Vertex, Node>) -> i32 {
    if hashes.is_empty() {return 0;}
    if hashes.len() == 1 { hashes.clear(); return 1; }

    // 找到最上面那一行中最靠左的顶点
    let mut vec = hashes.iter().map(|(v, _)|*v).collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        if a.row == b.row {
            a.col.cmp(&b.col)
        } else {
            a.row.cmp(&b.row)
        }
    });
    // 这个顶点是最上面那一行中, 最左边的顶点
    // 这个顶点必然有一个相邻的右侧顶点和一个相邻的下侧顶点
    // 且右侧顶点必然有一个下侧顶点
    // u-------r
    // |       |
    // |       |
    // d       |
    //         d
    let up_left = vec[0];
    let node_up_left = hashes.get(&up_left).unwrap();
    let up_right = node_up_left.right.unwrap_or(up_left);
    let down_left = node_up_left.down.unwrap_or(up_left);

    let node_up_right = hashes.get(&up_right).unwrap();
    let down_right = node_up_right.down.unwrap_or(up_right);

    // 寻找这几个点构成的最小矩形中的 up-left 顶点
    let row_min = up_left.row;
    let row_max = down_left.row.min(down_right.row);
    let col_min = up_left.col;
    let col_max = up_right.col;
    let mut vertices = hashes.iter()
        .filter(|(v, _)| {
            v.row > row_min && v.row <= row_max && v.col >= col_min && v.col <= col_max
        })
        .map(|(v, _)| *v)
        .collect::<Vec<_>>();
    vertices.sort_by(|a, b| {
        if a.row == b.row {
            a.col.cmp(&b.col)
        } else {
            a.row.cmp(&b.row)
        }
    });

    // 只有一个顶点的情况下
    if vertices.is_empty() {
        hashes.remove(&up_left);
        return 1;
    }
    let inner_up_left = vertices[0];
    // 内部 up-left 可能落在 down-left or down-right
    // 也可能是内部的其它点
    if inner_up_left == down_left {
        // 正好是 down-left
        // axxxxxxxxb
        // x        x
        // x        x
        // cxd      x
        //   e      f
        //   x      x
        //   g      x
        //          h
        // or
        //   axxxxxxb
        //   x      x
        //   x      x
        // dxc      x
        // e        f
        // x        x
        // g        x
        //          h
        // or 
        // axxxxxxxxb
        // x        x
        // cxd   ixxh
        //   e   j
        //   x   x
        //   g   k
        // or 
        //     axxxb
        //     x   x
        //  dxxc   hxxxxxi
        //  e         
        //  x         
        //  g         
        let a = up_left;
        let b = up_right;
        let c= inner_up_left;
        let node_c = hashes.get(&c).unwrap();
        let d = if let Some(right) = node_c.right {
            right
        } else {
            node_c.left.unwrap_or(c)
        };
        let e = Vertex::new(d.row + 1, d.col);
        let mut f = Vertex::new(d.row + 1, b.col);
        let g = hashes.get(&d).unwrap().down.unwrap_or(d);
        let mut h = down_right;

        hashes.remove(&a); hashes.remove(&b); hashes.remove(&c); hashes.remove(&d);

        if d == h { return (b.col - a.col + 1) * (c.row - a.row + 1); }

        let mut extra = 0;

        if d.row == h.row {
            let i = {
                let node = hashes.get(&h).unwrap();
                node.left.or(node.right).unwrap_or(h)
            };
            let mut j = i;
            j.row += 1;
            let k = hashes.get(&i).unwrap().down.unwrap_or(i);

            hashes.remove(&h); hashes.remove(&i);
            extra += if i.col > h.col {i.col - h.col} else {0};

            f = j;
            h = k;
        }

        hashes.get_mut(&g).unwrap().up = None;
        if e.row < g.row {
            hashes.get_mut(&g).unwrap().up = Some(e);
            hashes.insert(e, Node {
                down: Some(g),
                right: Some(f),
                ..Default::default()
            });
        }
        hashes.get_mut(&h).unwrap().up = None;
        if f.row < h.row {
            hashes.get_mut(&h).unwrap().up = Some(f);
            hashes.insert(f, Node {
                down: Some(h),
                left: Some(e),
                ..Default::default()
            });
        }

        extra += if c.col > d.col {c.col - d.col} else {0};

        return (b.col - a.col + 1) * (c.row - a.row + 1) + extra;
    } else if inner_up_left == down_right {
        // 正好是 down-right
        // axxxxxxxxb
        // x        x
        // x        x
        // x        cxd
        // f          e
        // x          x
        // x          g
        // h
        let a = up_left;
        let b = up_right;
        let c= inner_up_left;
        let node_c = hashes.get(&c).unwrap();
        let d = if let Some(right) = node_c.right {
            right
        } else {
            node_c.left.unwrap_or(c)
        };
        let e = Vertex::new(d.row + 1, d.col);
        let f = Vertex::new(d.row + 1, a.col);
        let g = hashes.get(&d).unwrap().down.unwrap_or(d);
        let h = down_left;

        hashes.remove(&a); hashes.remove(&b); hashes.remove(&c); hashes.remove(&d);

        hashes.get_mut(&g).unwrap().up = None;
        if e.row < g.row {
            hashes.get_mut(&g).unwrap().up = Some(e);
            hashes.insert(e, Node {
                down: Some(g),
                left: Some(f),
                ..Default::default()
            });
        }
        hashes.get_mut(&h).unwrap().up = None;
        if f.row < h.row {
            hashes.get_mut(&h).unwrap().up = Some(f);
            hashes.insert(f, Node {
                down: Some(h),
                right: Some(e),
                ..Default::default()
            });
        }

        let extra = if d.col > c.col {d.col-c.col} else {0};
        return (c.row - a.row + 1) * (c.col - a.col + 1) + extra;
    } else {
        // 是内部其它顶点
        // axxxxxexxxxxxxxxxxf
        // x                 x
        // x    xdxxxxxg     x
        // b    c            x
        // x    x
        // x    i
        // h
        let a= up_left;
        let e = Vertex::new(a.row, inner_up_left.col + 1);
        let f = up_right;
        let d = Vertex::new(inner_up_left.row, inner_up_left.col + 1);
        let g = hashes.get(&inner_up_left).unwrap().right.unwrap_or(inner_up_left);
        let c = Vertex::new(inner_up_left.row + 1, inner_up_left.col);
        let b = Vertex::new(inner_up_left.row + 1, a.col);
        let h = down_left;
        let i = hashes.get(&inner_up_left).unwrap().down.unwrap_or(inner_up_left);

        hashes.remove(&a);
        hashes.remove(&inner_up_left);

        hashes.get_mut(&f).unwrap().left = None;
        if e.col < f.col {
            hashes.get_mut(&f).unwrap().left = Some(e);
            hashes.insert(e, Node {
                down: Some(d),
                right: Some(f),
                ..Default::default()
            });
        }
        hashes.get_mut(&g).unwrap().left = None;
        if d.col < g.col {
            hashes.get_mut(&g).unwrap().left = Some(d);
            hashes.insert(d, Node {
                up: Some(e),
                right: Some(g),
                ..Default::default()
            });
        }
        hashes.get_mut(&h).unwrap().up = None;
        if b.row < h.row {
            hashes.get_mut(&h).unwrap().up = Some(b);
            hashes.insert(b, Node {
                down: Some(h),
                right: Some(c),
                ..Default::default()
            });
        }
        hashes.get_mut(&i).unwrap().up = None;
        if c.row < i.row {
            hashes.get_mut(&i).unwrap().up = Some(c);
            hashes.insert(c, Node {
                down: Some(i),
                left: Some(b),
                ..Default::default()
            });
        }

        return (inner_up_left.row - a.row + 1) * (inner_up_left.col - a.col + 1);
    }
}

fn solve(input: &str) -> i32 {
    let cmds = input.trim().split("\n")
        .map(|line| {
            let mut it = line.split(" ");
            let dir: Dir = it.next().unwrap().parse().unwrap();
            let steps: i32 = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();
            let color = &color[2..8];

            let p2_steps = i32::from_str_radix(&color[..5], 16).unwrap();
            let p2_dir = match &color[5..] {
                "0" => Dir::Right,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "3" => Dir::Up,
                _ => unreachable!(),
            };
            // DigCmd::new(p2_dir, p2_steps)
            DigCmd::new(dir, steps)
        })
        .collect::<Vec<_>>();

    calc(cmds)
}

#[test]
fn test() {
    let input = {"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
".trim()
    };
    let input = include_str!("input");
    let sum = solve(input);
    println!("sum: {:?}", sum);
}

