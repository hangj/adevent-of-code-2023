use std::{cmp::Ordering, collections::HashMap, str::FromStr, ops::Range};

type Label = String;

#[derive(Debug)]
struct Condition {
    /// could be one of [x, m, a, s]
    var: Label,
    num: i32,
    /// var.cmp(num) 的期望结果
    order: Ordering,
    /// If var.cmp(num) == order, jump to this label.
    /// This label may be a normal label, or an Accept/Reject
    jmp_to: Label,
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(":");
        let cond = it.next().unwrap();
        let jmp_to = it.next().unwrap().to_string();

        let var = cond[0..1].to_string();
        let order = match &cond[1..2] {
            ">" => Ordering::Greater,
            "<" => Ordering::Less,
            // "=" => Ordering::Equal,
            _ => unreachable!(),
        };
        let num = cond[2..].parse::<i32>().unwrap();
        Ok(Self { var, num, order, jmp_to })
    }
}

#[derive(Debug)]
enum Rule {
    Accept,
    Reject,
    Condition(Condition),
    /// 无条件跳转
    Jmp(Label),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(":") {
            Ok(Self::Condition(s.parse::<Condition>().unwrap()))
        } else {
            match s {
                "A" => Ok(Self::Accept),
                "R" => Ok(Self::Reject),
                s => Ok(Self::Jmp(s.into())),
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

type Rating = HashMap<Label, i32>;

fn calc(workflows: &HashMap<Label, Workflow>, ratings: Vec<Rating>) -> i32 {
    ratings.into_iter()
        .map(|rating| {
            let mut work_label = "in";

            loop {
                let flow = workflows.get(work_label).unwrap();
                for rule in flow.rules.iter() {
                    match rule {
                        Rule::Accept => {
                            return rating.values().sum();
                        },
                        Rule::Reject => return 0,
                        Rule::Condition(c) => {
                            if rating.get(&c.var).unwrap().cmp(&c.num) == c.order {
                                let to = &c.jmp_to;
                                if to == "A" {
                                    return rating.values().sum();
                                }
                                if to == "R" {
                                    return 0;
                                }
                                work_label = to;
                                break;
                            }
                        },
                        Rule::Jmp(to) => {
                            work_label = to;
                            break;
                        },
                    }
                }
            }
            0
        })
        .sum::<i32>()
}

fn calc_combinations(workflows: &HashMap<Label, Workflow>) {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Comb {
        x: Range<i32>,
        m: Range<i32>,
        a: Range<i32>,
        s: Range<i32>,
    }
    impl Comb {
        fn new() -> Self {
            Comb {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            }
        }
        fn get_mut(&mut self, s: &str) -> &mut Range<i32> {
            match s {
                "x" => &mut self.x,
                "m" => &mut self.m,
                "a" => &mut self.a,
                "s" => &mut self.s,
                _ => unreachable!(),
            }
        }
    }

    let label = "in";
    let comb = Comb::new();
    let mut all_comb = Vec::new();

    fn try_comb(mut comb: Comb, label: &str, all_comb: &mut Vec<Comb>, workflows: &HashMap<Label, Workflow>) {
        if label == "A" {
            all_comb.push(comb);
            return;
        }
        if label == "R" { return; }

        let flow = workflows.get(label).unwrap();
        for rule in flow.rules.iter() {
            match rule {
                Rule::Accept => {
                    all_comb.push(comb.clone());
                    return;
                },
                Rule::Reject => return,
                Rule::Condition(c) => {
                    let to = &c.jmp_to;
                    let num = c.num;
                    match c.order {
                        Ordering::Less => {
                            let mut comb_1 = comb.clone();
                            let var = comb_1.get_mut(&c.var);
                            var.end = num;
                            if var.len() > 0 {
                                try_comb(comb_1, to, all_comb, workflows);
                            }

                            let var = comb.get_mut(&c.var);
                            var.start = num;
                            if var.len() != 0 {
                                continue;
                            } else {
                                return;
                            }
                        },
                        Ordering::Greater => {
                            let mut comb_1 = comb.clone();
                            let var = comb_1.get_mut(&c.var);
                            var.start = num + 1;
                            if var.len() > 0 {
                                try_comb(comb_1, to, all_comb, workflows)
                            }
                            let var = comb.get_mut(&c.var);
                            var.end = num + 1;
                            if var.len() > 0 {
                                continue;
                            } else {
                                return;
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                Rule::Jmp(to) => {
                    try_comb(comb.clone(), to, all_comb, workflows);
                },
            }
        }
    }

    try_comb(comb, label, &mut all_comb, workflows);

    let mut sum = 0;
    for comb in all_comb {
        println!("comb: {:?}", comb);
        sum += comb.x.len() * comb.m.len() * comb.a.len() * comb.s.len();
    }
    println!("combinations: {}", sum);
}


fn parse(input: &str) {
    let mut it = input.split("\n\n");
    let workflows = it.next().unwrap();
    let ratings = it.next().unwrap();

    let workflows = workflows.split("\n")
        .map(|line| {
            let mut it = line.split("{");
            let label = it.next().unwrap().to_string();
            let rules = it.next().unwrap().trim_matches('}');
            let rules = rules.split(",")
                .map(|rule| {
                    rule.parse::<Rule>().unwrap()
                })
                .collect::<Vec<_>>();
            let workflow = Workflow { rules };
            (label, workflow)
        })
        .collect::<HashMap<_, _>>();
    let ratings = ratings.split("\n")
        .map(|line| {
            let line = &line[1..line.len()-1];
            line.split(",")
                .map(|rating| {
                    let mut it = rating.split("=");
                    (it.next().unwrap().to_string(), it.next().unwrap().parse::<i32>().unwrap())
                })
                .collect::<Rating>()
        })
        .collect::<Vec<_>>();

    let sum = calc(&workflows, ratings);
    println!("sum: {:?}", sum);

    // Part Two
    calc_combinations(&workflows);
}

#[test]
fn test() {
    let input = {"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
".trim()
    };

    let input = include_str!("input");

    parse(input);
}
