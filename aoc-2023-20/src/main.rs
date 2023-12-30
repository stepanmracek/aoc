use kdam::tqdm;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

#[derive(Debug)]
struct Broadcaster {}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    Low,
    High,
}

struct IO {
    pulse: Pulse,
    src: String,
}

struct Stats {
    low_count: usize,
    high_count: usize,
    rx_low: bool,
}

trait ProcessPulse {
    fn process(&mut self, io: &IO) -> Option<Pulse>;
}

impl ProcessPulse for Broadcaster {
    fn process(&mut self, io: &IO) -> Option<Pulse> {
        Some(io.pulse.clone())
    }
}

impl ProcessPulse for FlipFlop {
    fn process(&mut self, io: &IO) -> Option<Pulse> {
        if io.pulse == Pulse::High {
            // If a flip-flop module receives a high pulse, it is ignored and nothing happens
            None
        } else if !self.state {
            // If it was off, it turns on and sends a high pulse
            self.state = true;
            Some(Pulse::High)
        } else {
            // If it was on, it turns off and sends a low pulse.
            self.state = false;
            Some(Pulse::Low)
        }
    }
}

impl ProcessPulse for Conjunction {
    fn process(&mut self, io: &IO) -> Option<Pulse> {
        // When a pulse is received, the conjunction module first updates its memory for that input.
        self.inputs.insert(io.src.clone(), io.pulse.clone());

        if self.inputs.values().all(|p| p == &Pulse::High) {
            // Then, if it remembers high pulses for all inputs, it sends a low pulse;
            Some(Pulse::Low)
        } else {
            // otherwise, it sends a high pulse.
            Some(Pulse::High)
        }
    }
}

type Machine = HashMap<String, Module>;

fn parse_machine(s: &str) -> Machine {
    let mut machine = HashMap::new();
    for line in s.lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<String> = outputs.split(',').map(|o| o.trim().to_string()).collect();
        let module = match module.chars().next().unwrap() {
            '%' => Module {
                // they are initially off
                kind: ModuleType::FlipFlop(FlipFlop { state: false }),
                name: module.chars().skip(1).collect(),
                outputs,
            },
            '&' => Module {
                kind: ModuleType::Conjunction(Conjunction {
                    inputs: HashMap::new(),
                }),
                name: module.chars().skip(1).collect(),
                outputs,
            },
            'b' => Module {
                kind: ModuleType::Broadcaster(Broadcaster {}),
                name: module.chars().collect(),
                outputs,
            },
            _ => panic!("unknown module: {}", module),
        };
        machine.insert(module.name.clone(), module);
    }

    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (module_name, module) in machine.iter() {
        for output in module.outputs.iter() {
            if let Some(other_module) = machine.get(output) {
                if let ModuleType::Conjunction(_) = other_module.kind {
                    if inputs.contains_key(&other_module.name) {
                        inputs
                            .get_mut(&other_module.name)
                            .unwrap()
                            .push(module_name.clone());
                    } else {
                        inputs.insert(other_module.name.clone(), vec![module_name.clone()]);
                    }
                }
            }
        }
    }
    for (module_name, inputs) in inputs {
        if let Some(module) = machine.get_mut(&module_name) {
            if let ModuleType::Conjunction(c) = &mut module.kind {
                c.inputs = inputs
                    .into_iter()
                    // they initially default to remembering a low pulse for each input
                    .map(|input| (input, Pulse::Low))
                    .collect()
            }
        }
    }
    machine
}

fn push_button(machine: &mut Machine) -> Stats {
    // When you push the button, a single low pulse is sent directly to the broadcaster module
    let mut low_count = 1;
    let mut high_count = 0;
    let mut rx_low = false;
    let mut queue = VecDeque::new();
    queue.push_back((
        IO {
            pulse: Pulse::Low,
            src: String::from("button"),
        },
        String::from("broadcaster"),
    ));

    while let Some((queue_item, dest)) = queue.pop_front() {
        if let Some(module) = machine.get_mut(&dest) {
            let output_pulse = match &mut module.kind {
                ModuleType::Broadcaster(b) => b.process(&queue_item),
                ModuleType::Conjunction(c) => c.process(&queue_item),
                ModuleType::FlipFlop(f) => f.process(&queue_item),
            };
            // if the output pulse is something
            if let Some(output_pulse) = output_pulse {
                // send it to each output
                for new_dest in module.outputs.iter() {
                    queue.push_back((
                        IO {
                            pulse: output_pulse.clone(),
                            src: module.name.clone(),
                        },
                        new_dest.clone(),
                    ));
                    //print!("{}", module.name);
                    if output_pulse == Pulse::High {
                        high_count += 1;
                        //print!(" -high-> ");
                    } else {
                        low_count += 1;
                        //print!(" -low-> ");
                    }
                    //println!("{}", new_dest);

                    if output_pulse == Pulse::Low && new_dest == &String::from("rx") {
                        rx_low = true;
                    }
                }
            }
        }
    }

    Stats {
        low_count,
        high_count,
        rx_low,
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut machine = parse_machine(&file_content);
    for (module_name, module) in machine.iter() {
        println!("{}: {:?}", module_name, module);
    }
    let stats = (1..=1000).map(|_| push_button(&mut machine)).fold(
        Stats {
            high_count: 0,
            low_count: 0,
            rx_low: false,
        },
        |acc, x| Stats {
            high_count: acc.high_count + x.high_count,
            low_count: acc.low_count + x.low_count,
            rx_low: acc.rx_low || x.rx_low,
        },
    );
    let result = stats.low_count * stats.high_count;
    println!("Part 1: {result}");

    let mut machine = parse_machine(&file_content);
    for count in tqdm!(1..) {
        let stats = push_button(&mut machine);
        if stats.rx_low {
            println!("Part 2: {count}");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_flip_flop() {
        let mut flip_flop = FlipFlop { state: false };

        let out = flip_flop.process(&crate::IO {
            pulse: crate::Pulse::High,
            src: String::from("a"),
        });
        assert!(out.is_none());

        let out = flip_flop.process(&crate::IO {
            pulse: crate::Pulse::Low,
            src: String::from("a"),
        });
        assert_eq!(out, Some(Pulse::High));
        assert_eq!(flip_flop.state, true);

        let out = flip_flop.process(&crate::IO {
            pulse: crate::Pulse::Low,
            src: String::from("a"),
        });
        assert_eq!(out, Some(Pulse::Low));
        assert_eq!(flip_flop.state, false);
    }

    #[test]
    fn test_conjunction() {
        let mut conjunction = Conjunction {
            inputs: HashMap::from([
                (String::from("a"), Pulse::Low),
                (String::from("b"), Pulse::Low),
            ]),
        };

        let out = conjunction.process(&crate::IO {
            pulse: crate::Pulse::High,
            src: String::from("a"),
        });
        assert_eq!(out, Some(Pulse::High));

        let out = conjunction.process(&crate::IO {
            pulse: crate::Pulse::High,
            src: String::from("b"),
        });
        assert_eq!(out, Some(Pulse::Low));

        let out = conjunction.process(&crate::IO {
            pulse: crate::Pulse::High,
            src: String::from("a"),
        });
        assert_eq!(out, Some(Pulse::Low));

        let out = conjunction.process(&crate::IO {
            pulse: crate::Pulse::Low,
            src: String::from("a"),
        });
        assert_eq!(out, Some(Pulse::High));
    }

    #[test]
    fn test_push_button() {
        let mut machine =
            parse_machine("broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a");
        let stats = push_button(&mut machine);
        assert_eq!(stats.high_count, 4);
        assert_eq!(stats.low_count, 8);

        let mut machine =
            parse_machine("broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output");
        let stats = (1..=1000).map(|_| push_button(&mut machine)).fold(
            Stats {
                high_count: 0,
                low_count: 0,
                rx_low: false,
            },
            |acc, x| Stats {
                high_count: acc.high_count + x.high_count,
                low_count: acc.low_count + x.low_count,
                rx_low: acc.rx_low || x.rx_low,
            },
        );
        assert_eq!(stats.high_count, 2750);
        assert_eq!(stats.low_count, 4250);
    }
}
