use {
    aoc_core::{lcm, Solution},
    std::collections::{HashMap, VecDeque},
};

pub struct Day20;

impl<'a> Solution<'a> for Day20 {
    type Input = (Broadcaster<'a>, HashMap<&'a str, Module<'a>>);
    type Output1 = usize;
    type Output2 = ();

    fn parse_input(data: &'a str) -> Self::Input {
        let mut broadcaster = None;
        let modules = data
            .lines()
            .filter_map(|line| {
                let (label, destinations) = line.split_once(" -> ").expect("Has arrow");
                let destinations = destinations.split(',').map(|s| s.trim()).collect();

                if label == "broadcaster" {
                    assert!(broadcaster.is_none(), "Only one broadcaster");
                    broadcaster = Some(Broadcaster { destinations });
                    return None;
                }

                let (kind, label) = label
                    .strip_prefix('%')
                    .map(|s| (ModuleKind::FlipFlop, s))
                    .unwrap_or_else(|| {
                        (
                            ModuleKind::Conjunction,
                            label
                                .strip_prefix('&')
                                .expect("All modules are flip-flop or conjunction"),
                        )
                    });
                let module = Module { destinations, kind };
                Some((label, module))
            })
            .collect();
        (broadcaster.expect("Must have broadcaster"), modules)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (broadcaster, modules) = input;
        let default_state = construct_default_state(&modules);

        let mut current_state = default_state.clone();
        let mut high_counts = Vec::new();
        let mut low_counts = Vec::new();
        let mut button_push_counts = 0;
        while button_push_counts < 1000 {
            let (new_state, low_count, high_count) =
                push_button(&current_state, &broadcaster, &modules);
            low_counts.push(low_count);
            high_counts.push(high_count);
            button_push_counts += 1;
            if new_state == default_state {
                break;
            }
            current_state = new_state;
        }
        let cycle_total_low = low_counts.iter().copied().sum::<usize>();
        let cycle_total_high = high_counts.iter().copied().sum::<usize>();

        if button_push_counts == 1000 {
            return cycle_total_low * cycle_total_high;
        }

        let n_cycles = 1000 / button_push_counts;
        let extra_steps = 1000 % button_push_counts;

        let total_low =
            (n_cycles * cycle_total_low) + low_counts.into_iter().take(extra_steps).sum::<usize>();
        let total_high = (n_cycles * cycle_total_high)
            + high_counts.into_iter().take(extra_steps).sum::<usize>();
        total_low * total_high
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (broadcaster, modules) = input;
        let default_state = construct_default_state(&modules);

        let Some(switches) = find_output_switches("rx", &modules, &default_state) else {
            // There's no example to test this on. The given example does not have
            // the "rx" output, so we just exit.
            return;
        };

        assert!(switches
            .iter()
            .all(|label| matches!(default_state.get(label), Some(ModuleState::FlipFlop(_)))));

        // There are four special switches with asymmetric cycle times.
        // These switches determine the cycle time for their whole sub-circuit.
        let sub_circuit_cycles = switches.into_iter().filter_map(|label| {
            let (on_cycle, off_cycle) =
                count_cycle_time(label, &default_state, &broadcaster, &modules);
            if on_cycle != off_cycle {
                Some(on_cycle + off_cycle)
            } else {
                None
            }
        });

        // All the sub-circuit cycles need to be aligned for the output
        // to be a low pulse. Therefore the answer is the lcm of the sub-circuit cycle lengths.
        let answer = sub_circuit_cycles.reduce(lcm).unwrap();
        println!("{answer}");
    }
}

fn count_cycle_time<'a>(
    label: &'a str,
    default_state: &SystemState<'a>,
    broadcaster: &Broadcaster<'a>,
    modules: &HashMap<&'a str, Module<'a>>,
) -> (u64, u64) {
    let mut current_state = default_state.clone();
    let on_cycle = {
        let mut count = 0;
        while !current_state.get(&label).unwrap().as_flip_flop() {
            let (new_state, _, _) = push_button(&current_state, broadcaster, modules);
            count += 1;
            current_state = new_state;
        }
        count
    };
    let off_cycle = {
        let mut count = 0;
        while current_state.get(&label).unwrap().as_flip_flop() {
            let (new_state, _, _) = push_button(&current_state, broadcaster, modules);
            count += 1;
            current_state = new_state;
        }
        count
    };
    (on_cycle, off_cycle)
}

fn construct_default_state<'a>(modules: &HashMap<&'a str, Module<'a>>) -> SystemState<'a> {
    let mut inputs_map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for (source, module) in modules.iter() {
        for dest in &module.destinations {
            let entry = inputs_map.entry(*dest).or_default();
            entry.push(*source);
        }
    }

    modules
        .iter()
        .map(|(label, module)| {
            let state = match module.kind {
                ModuleKind::FlipFlop => ModuleState::flip_flop(),
                ModuleKind::Conjunction => {
                    ModuleState::conjunction(inputs_map[label].iter().copied())
                }
            };
            (*label, state)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<'a> {
    destinations: Vec<&'a str>,
    kind: ModuleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKind {
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Broadcaster<'a> {
    destinations: Vec<&'a str>,
}

// In the system the output in connected to a single conjunction,
// which takes input from few conjunctions, each of which take input
// from several flip-flops. Therefore, to get a low pulse output
// we need all those switches to turn on simultaneously.
// This function finds the set of flip-flops we need to focus on.
fn find_output_switches<'a>(
    output_label: &'a str,
    modules: &HashMap<&'a str, Module<'a>>,
    default_state: &SystemState<'a>,
) -> Option<Vec<&'a str>> {
    fn to_inputs<'a, 'b, I>(
        labels: I,
        state: &'b SystemState<'a>,
    ) -> impl Iterator<Item = &'b &'a str>
    where
        I: Iterator<Item = &'b &'a str>,
    {
        labels.flat_map(|label| {
            state
                .get(label)
                .expect("Sub Con has state")
                .as_conjunction()
                .keys()
        })
    }

    let con = modules.iter().find_map(|(label, module)| {
        if module.destinations == [output_label] {
            Some(label)
        } else {
            None
        }
    })?;
    let con_inputs = default_state
        .get(con)
        .expect("Con has state")
        .as_conjunction()
        .keys();
    let result = to_inputs(to_inputs(con_inputs, default_state), default_state)
        .copied()
        .collect();
    Some(result)
}

fn push_button<'a>(
    init_state: &SystemState<'a>,
    broadcaster: &Broadcaster<'a>,
    modules: &HashMap<&'a str, Module<'a>>,
) -> (SystemState<'a>, usize, usize) {
    let mut low_count = broadcaster.destinations.len() + 1;
    let mut high_count = 0;

    let mut state = init_state.clone();
    let mut pulses = VecDeque::new();
    for label in &broadcaster.destinations {
        pulses.push_back(Pulse {
            from: "broadcaster",
            to: label,
            kind: PulseKind::Low,
        });
    }

    while let Some(pulse) = pulses.pop_front() {
        let response_pulse = match state.get_mut(pulse.to) {
            Some(ModuleState::FlipFlop(is_on)) => match pulse.kind {
                PulseKind::High => None,
                PulseKind::Low => {
                    if *is_on {
                        *is_on = false;
                        Some(PulseKind::Low)
                    } else {
                        *is_on = true;
                        Some(PulseKind::High)
                    }
                }
            },
            Some(ModuleState::Conjunction(input_states)) => {
                *input_states.get_mut(pulse.from).expect("All inputs known") = pulse.kind;
                match pulse.kind {
                    PulseKind::Low => Some(PulseKind::High),
                    PulseKind::High => {
                        if input_states.values().all(|k| matches!(k, PulseKind::High)) {
                            Some(PulseKind::Low)
                        } else {
                            Some(PulseKind::High)
                        }
                    }
                }
            }
            None => {
                assert!(pulse.to == "rx" || pulse.to == "output");
                None
            }
        };
        match response_pulse {
            None => (),
            Some(PulseKind::Low) => {
                let destinations = &modules[pulse.to].destinations;
                low_count += destinations.len();
                for label in destinations {
                    pulses.push_back(Pulse {
                        from: pulse.to,
                        to: label,
                        kind: PulseKind::Low,
                    });
                }
            }
            Some(PulseKind::High) => {
                let destinations = &modules[pulse.to].destinations;
                high_count += destinations.len();
                for label in destinations {
                    pulses.push_back(Pulse {
                        from: pulse.to,
                        to: label,
                        kind: PulseKind::High,
                    });
                }
            }
        }
    }

    (state, low_count, high_count)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    kind: PulseKind,
}

type SystemState<'a> = HashMap<&'a str, ModuleState<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleState<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, PulseKind>),
}

impl<'a> ModuleState<'a> {
    fn flip_flop() -> Self {
        Self::FlipFlop(false)
    }

    fn conjunction<I>(inputs: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::Conjunction(
            inputs
                .into_iter()
                .map(|label| (label, PulseKind::Low))
                .collect(),
        )
    }

    fn as_conjunction(&self) -> &HashMap<&'a str, PulseKind> {
        match self {
            Self::Conjunction(inputs) => inputs,
            Self::FlipFlop(_) => panic!("Should be conjunction"),
        }
    }

    fn as_flip_flop(&self) -> bool {
        match self {
            Self::FlipFlop(inner) => *inner,
            Self::Conjunction(_) => panic!("Should be flip-flop"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseKind {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day20_example.txt");

    #[test]
    fn test_part1() {
        let input = Day20::parse_input(EXAMPLE_INPUT);
        let output = Day20::part_1(input);
        assert_eq!(output, 11_687_500);
    }

    #[test]
    #[allow(clippy::let_unit_value, clippy::unit_cmp)]
    fn test_part2() {
        let input = Day20::parse_input(EXAMPLE_INPUT);
        let output = Day20::part_2(input);
        assert_eq!(output, ());
    }
}
