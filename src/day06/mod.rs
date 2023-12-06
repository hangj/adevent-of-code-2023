//! Time: t, Distance: d
//! Hold the button for x milliseconds
//! The distance will be: (t-x) * x
//! 


pub struct Race {
    pub time: f64,
    pub distance: f64,
}

impl Race {
    pub fn new(time: f64, distance: f64) -> Race {
        Self { time, distance }
    }
}

impl Race {
    pub fn beat_ways(&self) -> u64 {
        let race = self;
        let x = (race.time.powf(2.) / 4. - race.distance).sqrt() + race.time / 2.;
        let start = if x > race.time / 2. {
            race.time - x.floor()
        } else {
            x.ceil()
        };
        let range = (start as u64)..=(race.time as u64 - start as u64);
        println!("range: {:?}", range);
        range.count() as u64
    }
}

#[test]
fn solve() {
    // let input = include_str!("input");
    let vec = vec![
        Race::new(59., 543.),
        Race::new(68., 1020.),
        Race::new(82., 1664.),
        Race::new(74., 1022.),
    ];

    // x = sqrt(pow(t)/4 - y) + t/2
    let product: u64 = vec.iter()
        .map(|race| {
            race.beat_ways()
        })
        .product();

    println!("product: {}", product);

    // Part Two
    let ways = Race::new(59688274., 543102016641022.).beat_ways();
    println!("ways: {}", ways);
}