use std::{collections::{HashSet, HashMap}, str::FromStr, ops::Range, vec};

#[derive(Debug, Clone, Copy)]
pub struct MyMap {
    /// source
    pub src: u128,
    /// destination
    pub dst: u128,
    pub range_len: u128,
}

impl MyMap {
    pub fn get(&self, key: u128) -> Option<u128> {
        if key >= self.src && key < self.src + self.range_len {
            Some(self.dst + key - self.src)
        } else {
            None
        }
    }
}

impl FromStr for MyMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" ").filter(|s|!s.is_empty());
        let dst = it.next().unwrap().parse().unwrap();
        let src = it.next().unwrap().parse().unwrap();
        let range_len = it.next().unwrap().parse().unwrap();
        Ok(Self {
            src,
            dst,
            range_len,
        })
    }
}


#[derive(Debug)]
pub struct VecMyMap(pub Vec<MyMap>);

impl VecMyMap {
    pub fn get(&self, key: u128) -> u128 {
        for map in self.0.iter() {
            if let Some(v) = map.get(key) {
                return v;
            }
        }
        return key;
    }

    pub fn get_dest_ranges(&self, range: Vec<Range<u128>>) -> Vec<Range<u128>> {
        let mut vec = range;
        let mut ret = Vec::new();

        while !vec.is_empty() {
            let range = vec.pop().unwrap();
            let mut intersected = false;
            for r in self.0.iter() {
                // 没有交集
                // xxxx
                //     xxxx
                if r.src+r.range_len <= range.start || range.end <= r.src {
                    continue;
                }
                intersected = true;
                // 有交集
                // xxxxx
                //   xxxxx
                let start = r.src.max(range.start);
                let end = (r.src+r.range_len).min(range.end);

                let seg1 = range.start..start;
                let seg2 = end..range.end;
                if !seg1.is_empty() {
                    vec.push(seg1);
                }
                if !seg2.is_empty() {
                    vec.push(seg2);
                }

                let diff_start = start - r.src;
                let len = end - start;
                let dst_start = r.dst + diff_start;
                ret.push(dst_start..dst_start+len);
                break;
            }

            // 完全没有交集
            if !intersected {
                ret.push(range);
            }
        }

        ret
    }
}

impl From<Vec<MyMap>> for VecMyMap {
    fn from(value: Vec<MyMap>) -> Self {
        Self(value)
    }
}


#[test]
fn solve() {
    let input = include_str!("input");

    let mut it = input.split("\n\n");

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let seeds = nums.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec::<_>>();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let seed_to_soil: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let soil_to_fertilizer: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let fertilizer_to_water: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let water_to_light: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let light_to_temperature: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let temperature_to_humidity: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();

    let group = it.next().unwrap();
    let nums = group.split(":").skip(1).next().unwrap();
    let humidity_to_location: VecMyMap = nums.split("\n")
        .filter(|s|!s.is_empty())
        .map(|line|{
            line.parse::<MyMap>().unwrap()
        })
        .collect::<Vec<_>>()
        .into();


    let min = seeds.iter()
        .map(|seed| {
            let seed = seed_to_soil.get(*seed);
            let seed = soil_to_fertilizer.get(seed);
            let seed = fertilizer_to_water.get(seed);
            let seed = water_to_light.get(seed);
            let seed = light_to_temperature.get(seed);
            let seed = temperature_to_humidity.get(seed);
            let seed = humidity_to_location.get(seed);
            seed
        })
        .min()
        .unwrap();

    println!("min: {}", min);

    // --------- Part Two -------------
    let mut seed_ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start =seeds[i];
        let len = seeds[i + 1];
        let range = start .. start + len;
        seed_ranges.push(range);
    }


    let ranges = seed_to_soil.get_dest_ranges(seed_ranges);
    let ranges = soil_to_fertilizer.get_dest_ranges(ranges);
    let ranges = fertilizer_to_water.get_dest_ranges(ranges);
    let ranges = water_to_light.get_dest_ranges(ranges);
    let ranges = light_to_temperature.get_dest_ranges(ranges);
    let ranges = temperature_to_humidity.get_dest_ranges(ranges);
    let ranges = humidity_to_location.get_dest_ranges(ranges);
    // println!("ranges: {:?}", ranges);

    let min = ranges.iter().min_by_key(|r|r.start).unwrap();
    println!("min: {}", min.start);
}
