use std::{collections::{HashMap, VecDeque, hash_map::DefaultHasher}, default, hash::{Hash, Hasher}};


trait OnRecvPulse {
    fn on_recv(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    #[default]
    Low,
    High,
}

/// prefix %
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FlipFlopState {
    #[default]
    Off,
    On,
}

#[derive(Debug, Default)]
struct Mail {
    pulse: Pulse,
    from: String,
    to: String,
}

impl Mail {
    fn new(pulse: Pulse, from: String, to: String) -> Self {
        Self { pulse, from, to }
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct FlipFlop {
    inputs: Vec<String>,
    outputs: Vec<String>,
    state: FlipFlopState,
}

impl OnRecvPulse for FlipFlop {
    fn on_recv(&mut self, pulse: Pulse, _from: &str) -> Option<Pulse> {
        let state = &mut self.state;
        match pulse {
            Pulse::Low => {
                match state {
                    FlipFlopState::Off => {
                        *state = FlipFlopState::On;
                        Some(Pulse::High)
                    },
                    FlipFlopState::On => {
                        *state = FlipFlopState::Off;
                        Some(Pulse::Low)
                    },
                }
            },
            Pulse::High => None,
        }
    }
}

/// prefix &
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Conjunction {
    /// default: low pulse
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl OnRecvPulse for Conjunction {
    fn on_recv(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        *self.inputs.get_mut(from).unwrap() = pulse;
        if self.inputs.iter().all(|(_, p)| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}


// impl Hash for Conjunction {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let mut v = self.inputs.iter().collect::<Vec<_>>();
//         v.sort_by_key(|x|x.0);
//         v.hash(state);
//         self.outputs.hash(state);
//     }
// }

/// named broadcaster
/// When it receives a pulse, it sends the same pulse to all of its destination modules.
/// When you push the button, a single low pulse is sent directly to the broadcaster module.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Broadcast {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl OnRecvPulse for Broadcast {
    fn on_recv(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct UntypedModule {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl OnRecvPulse for UntypedModule {
    fn on_recv(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    UntypedModule(UntypedModule),
    Broadcast(Broadcast),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Module {
    fn get_outputs(&self) -> &Vec<String> {
        match self {
            Module::UntypedModule(m) => &m.outputs,
            Module::Broadcast(m) => &m.outputs,
            Module::FlipFlop(m) => &m.outputs,
            Module::Conjunction(m) => &m.outputs,
        }
    }
    fn get_outputs_mut(&mut self) -> &mut Vec<String> {
        match self {
            Module::UntypedModule(m) => &mut m.outputs,
            Module::Broadcast(m) => &mut m.outputs,
            Module::FlipFlop(m) => &mut m.outputs,
            Module::Conjunction(m) => &mut m.outputs,
        }
    }
}

impl OnRecvPulse for Module {
    fn on_recv(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match self {
            Module::UntypedModule(m) => m.on_recv(pulse, from),
            Module::Broadcast(m) => m.on_recv(pulse, from),
            Module::FlipFlop(m) => m.on_recv(pulse, from),
            Module::Conjunction(m) => m.on_recv(pulse, from),
        }
    }
}

#[derive(Debug)]
struct MailBuilder {
    count_low: usize,
    count_high: usize,
}
impl MailBuilder {
    fn new() -> Self {
        Self { count_low: 0, count_high: 0 }
    }
    fn build(&mut self, pulse: Pulse, from: String, to: String) -> Mail {
        match pulse {
            Pulse::Low => self.count_low += 1,
            Pulse::High => self.count_high += 1,
        }
        // println!("{from}  -{:?}->  {to}", pulse);
        if to == "rx" && pulse == Pulse::Low {
            println!("low pulse delivered");
        }
        Mail::new(pulse, from, to)
    }
}

#[test]
fn test() {
    let input = {"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
.trim()
    };

    let input = {"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
.trim()
    };

    let input = include_str!("input");

    let mut map = HashMap::new();

    let mut inputs = HashMap::new();

    input.split("\n")
        .for_each(|line| {
            let mut it = line.split(" -> ");
            let label = it.next().unwrap().trim();
            let outputs = it.next().unwrap().trim();

            let (name, module) = if label == "broadcaster" {
                (label, Module::Broadcast(Default::default()))
            } else {
                match &label[0..1] {
                    "%" => {
                        (&label[1..], Module::FlipFlop(Default::default()))
                    },
                    "&" => {
                        (&label[1..], Module::Conjunction(Default::default()))
                    },
                    s => {
                        (s, Module::UntypedModule(Default::default()))
                    },
                }
            };

            if map.get(name).is_none() {
                map.insert(name, module);
            }

            let outs = map.get_mut(name).unwrap().get_outputs_mut();
            outputs.split(",")
                .for_each(|o| {
                    let o = o.trim();
                    outs.push(o.into());

                    if inputs.get(o).is_none() {
                        inputs.insert(o, Vec::new());
                    }
                    inputs.get_mut(o).unwrap().push(name);
                });
        });

    // Init the inputs for conjunction modules
    for (k, v) in inputs.into_iter() {
        // some modules don't have outputs
        if map.get(k).is_none() {continue;}

        let m = map.get_mut(k).unwrap();
        if let Module::Conjunction(con) = m {
            con.inputs = v.into_iter()
                .map(|s| (s.to_string(), Pulse::Low))
                .collect();
        }
    }

    println!("inputs: {:#?}", map);

    // let mut init_vec = map.iter().map(|(k,m)| (k.to_owned(), m.clone())).collect::<Vec<_>>();
    // init_vec.sort_by_key(|x|x.0);

    let mut builder = MailBuilder::new();

    'outer: for i in 1.. {
        // println!("i: {}", i);
        let pulse = Pulse::Low;
        let mail = builder.build(pulse, "button".into(), "broadcaster".into());

        let mut mail_queue = VecDeque::new();
        mail_queue.push_back(mail);

        loop {
            if mail_queue.is_empty() {break;}
            let mail = mail_queue.pop_front().unwrap();
            if map.get(mail.to.as_str()).is_none() { continue; }

            let module = map.get_mut(mail.to.as_str()).unwrap();
            if let Some(pulse) = module.on_recv(mail.pulse, mail.from.as_str()) {
                for out in module.get_outputs() {
                    let new_mail = builder.build(pulse, mail.to.clone(), out.clone());
                    mail_queue.push_back(new_mail);

                    if pulse == Pulse::Low {
                        match mail.to.as_str() {
                            "mh" => println!("mh: {}", i), // 4051, 8102
                            "jt" => println!("jt: {}", i), // 3919, 7838
                            "pz" => println!("pz: {}", i), // 3761, 7522
                            "rn" => println!("rn: {}", i), // 3907, 7814
                            _ => (),
                        }
                        if out == "rx" {
                            break 'outer;
                        }
                    }
                }
            }
        }

        // println!("builder: {:?}", builder);
        // println!("cur map: {:#?}", map);

        // let mut v = map.iter().map(|(k,m)| (k.to_owned(), m.clone())).collect::<Vec<_>>();
        // v.sort_by_key(|x|x.0);
        // if v == init_vec { break; }
    }
}
