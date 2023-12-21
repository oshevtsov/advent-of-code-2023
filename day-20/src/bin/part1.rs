use anyhow::Error;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let mut module_config: ModuleConfiguration = input.parse().expect("Failed to parse input");
    module_config.count_pulses(1000)
}

// module configuration representation
#[derive(Debug)]
struct ModuleConfiguration(HashMap<String, Module>);

impl ModuleConfiguration {
    fn count_pulses(&mut self, num_cycles: usize) -> usize {
        let mut results: HashSet<CycleResult> = HashSet::new();
        let mut period = 0;

        for cycle_idx in 1..=num_cycles {
            let result = self.run_cycle();
            if results.contains(&result) {
                break;
            }
            results.insert(result);
            period = cycle_idx;
        }

        let mut total_low = 0;
        let mut total_high = 0;

        results.iter().for_each(|result| {
            total_low += result.low_count;
            total_high += result.high_count;
        });

        if period == num_cycles {
            return total_low * total_high;
        }

        // remove full cycles
        let num_periods = num_cycles / period;
        total_low *= num_periods;
        total_high *= num_periods;

        // count the remaining cycles
        for _ in 0..num_cycles % period {
            let result = self.run_cycle();
            total_low += result.low_count;
            total_high += result.high_count;
        }

        total_low * total_high
    }

    fn run_cycle(&mut self) -> CycleResult {
        // we always start with a low pulse from the 'button module'
        let mut low_count: usize = 0;
        let mut high_count: usize = 0;
        let mut steps: VecDeque<CycleStep> = VecDeque::from([CycleStep {
            input_module_name: String::from("button"),
            input_pulse: Pulse::Low,
            module_name: String::from("broadcaster"),
        }]);

        while let Some(step) = steps.pop_front() {
            match step.input_pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            if let Some(module) = self.0.get_mut(&step.module_name) {
                if let Some(output_pulse) =
                    module.process_pulse(&step.input_module_name, step.input_pulse)
                {
                    let module_outputs = match module {
                        Module::FlipFlop { outputs, .. } => outputs,
                        Module::Conjunction { outputs, .. } => outputs,
                        Module::Broadcast { outputs } => outputs,
                    };

                    for next_module_name in module_outputs {
                        let new_step = CycleStep {
                            input_module_name: step.module_name.clone(),
                            input_pulse: output_pulse,
                            module_name: next_module_name.clone(),
                        };
                        steps.push_back(new_step);
                    }
                }
            }
        }

        CycleResult {
            state: self.state(),
            low_count,
            high_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CycleResult {
    state: String,
    low_count: usize,
    high_count: usize,
}

#[derive(Debug, Clone)]
struct CycleStep {
    input_module_name: String,
    input_pulse: Pulse,
    module_name: String,
}

impl FromStr for ModuleConfiguration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = HashMap::new();
        s.lines()
            .filter_map(|l| match l.trim() {
                trimmed if !trimmed.is_empty() => Some(trimmed),
                _ => None,
            })
            .for_each(|l| {
                if let Some((module_type, outputs_str)) = l.split_once(" -> ") {
                    let outputs: Vec<String> = outputs_str
                        .split(',')
                        .map(|name| name.trim().to_owned())
                        .collect();

                    if let Some(c) = module_type.chars().next() {
                        match c {
                            '%' => {
                                config.insert(
                                    module_type[1..].to_owned(),
                                    Module::FlipFlop { on: false, outputs },
                                );
                            }
                            '&' => {
                                config.insert(
                                    module_type[1..].to_owned(),
                                    Module::Conjunction {
                                        inputs: HashMap::new(),
                                        outputs,
                                    },
                                );
                            }
                            _ => {
                                config
                                    .insert(module_type.to_owned(), Module::Broadcast { outputs });
                            }
                        }
                    }
                }
            });

        init_conjunctions(&mut config);
        Ok(Self(config))
    }
}

fn init_conjunctions(config: &mut HashMap<String, Module>) {
    let mut visited = HashSet::new();
    let mut steps: VecDeque<String> = VecDeque::from([String::from("broadcaster")]);

    while let Some(module_name) = steps.pop_front() {
        if visited.contains(&module_name) {
            continue;
        }
        visited.insert(module_name.to_owned());
        if let Some(module) = config.get(&module_name) {
            let module_outputs = match module {
                Module::FlipFlop { outputs, .. } => outputs,
                Module::Conjunction { outputs, .. } => outputs,
                Module::Broadcast { outputs } => outputs,
            };

            for next_module_name in module_outputs.to_owned().iter() {
                steps.push_back(next_module_name.to_owned());
                if let Some(Module::Conjunction { inputs, .. }) = config.get_mut(next_module_name) {
                    inputs.insert(module_name.clone(), Pulse::Low);
                }
            }
        }
    }
}

impl State for ModuleConfiguration {
    fn state(&self) -> String {
        self.0.values().map(State::state).collect()
    }
}

// module representation
#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        on: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Broadcast {
        outputs: Vec<String>,
    },
}

impl Module {
    fn process_pulse(&mut self, input_module_name: &str, input_pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop { on, .. } => match input_pulse {
                Pulse::Low => {
                    let flip_flop_state = *on;
                    *on = !flip_flop_state;

                    if flip_flop_state {
                        Some(Pulse::Low)
                    } else {
                        Some(Pulse::High)
                    }
                }
                Pulse::High => None,
            },
            Module::Conjunction { inputs, .. } => {
                if let Some(prev_input_pulse) = inputs.get_mut(input_module_name) {
                    *prev_input_pulse = input_pulse;
                }

                if inputs.values().all(|pulse| *pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcast { .. } => Some(input_pulse),
        }
    }
}

impl State for Module {
    fn state(&self) -> String {
        match self {
            Module::FlipFlop { on, .. } => {
                if *on {
                    String::from("1")
                } else {
                    String::from("0")
                }
            }
            Module::Conjunction { inputs, .. } => inputs.values().map(State::state).collect(),
            Module::Broadcast { .. } => String::new(),
        }
    }
}

// pulse representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    Low,
    High,
}

impl State for Pulse {
    fn state(&self) -> String {
        match self {
            Pulse::Low => String::from("0"),
            Pulse::High => String::from("1"),
        }
    }
}

// traits
trait State {
    fn state(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process_1() {
        let input = r#"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "#;
        assert_eq!(32000000, process(input));
    }

    #[test]
    fn part1_process_2() {
        let input = r#"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "#;
        assert_eq!(11687500, process(input));
    }
}
