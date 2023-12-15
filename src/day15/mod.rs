
fn hash(s: &str) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value = (value * 17) % 256;
    }
    value
}

#[derive(Debug, Clone, Copy)]
struct Lens {
    label: &'static str,
    focal_length: usize,
}

#[derive(Debug)]
struct HashMap {
    boxes: Vec<Vec<Lens>>,
}

impl HashMap {
    pub fn new() -> Self {
        let mut boxes = Vec::with_capacity(256);
        boxes.resize(256, Vec::new());
        Self {
            boxes,
        }
    }

    pub fn update(&mut self, s: &'static str) {
        if s.ends_with('-') {
            let label = &s[..s.len()-1];
            let idx = hash(label);

            let v = &mut self.boxes[idx];
            for i in 0..v.len() {
                if v[i].label == label {
                    v.remove(i);
                    break;
                }
            }
        }else{
            let mut it = s.split('=');
            let label = it.next().unwrap();
            let focal_length = it.next().unwrap().parse().unwrap();

            let idx = hash(label);
            let v = &mut self.boxes[idx];
            for i in 0..v.len() {
                if v[i].label == label {
                    v[i].focal_length = focal_length;
                    return;
                }
            }
            v.push(Lens { label, focal_length })
        }
    }
}

#[test]
fn solve() {
    let input = include_str!("input");
    let sum = input.trim()
        .split(",")
        .map(|s|hash(s))
        .sum::<usize>();
    println!("{}", sum);
}

#[test]
fn test() {
    let input = "HASH";
    let ret = hash(input);
    println!("{}", ret);
    assert_eq!(ret, 52);

    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let sum = input.trim()
        .split(",")
        .map(|s|hash(s))
        .sum::<usize>();
    println!("{}", sum);
    assert_eq!(sum, 1320);

    // Part Two
    let input = include_str!("input");
    let mut hashmap = HashMap::new();
    input.trim()
        .split(",")
        .for_each(|s|hashmap.update(s));

    let mut sum = 0;
    
    for (box_id, b) in hashmap.boxes.iter().enumerate() {
        if b.is_empty() {continue;}
        sum += b.iter().enumerate().map(|(slot, lens)| {
            (box_id + 1) * (slot + 1) * lens.focal_length
        })
        .sum::<usize>();
    }

    println!("sum: {:?}", sum);
}

