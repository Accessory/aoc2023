use std::collections::{HashMap, VecDeque};

use utils::{get_input_path, lcm, parse_file_into};

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Broadcast,
    Conjunction(HashMap<String, bool>),
}

impl From<char> for ModuleType {
    fn from(value: char) -> Self {
        match value {
            '%' => Self::FlipFlop(false),
            '&' => Self::Conjunction(HashMap::new()),
            _ => Self::Broadcast,
        }
    }
}

fn conjunction_pulse_check(map: &mut HashMap<String, bool>) -> Pulse {
    for item in map.iter() {
        if !item.1 {
            return Pulse::High;
        }
    }
    Pulse::Low
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl From<String> for Module {
    fn from(value: String) -> Self {
        let first_char = value.chars().next().unwrap();

        let module_type: ModuleType = first_char.into();

        let mut split = value.split([' ', ',']);

        let name = match module_type {
            ModuleType::Broadcast => split.next().unwrap(),
            _ => split.next().unwrap()[1..].into(),
        }
        .into();

        let mut destinations = Vec::new();

        while let Some(value) = split.nth(1) {
            destinations.push(value.into());
        }

        Self {
            name,
            module_type,
            destinations,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    #[default]
    Low,
    High,
}
impl Pulse {
    fn to_bool(&self) -> bool {
        match self {
            Pulse::Low => false,
            Pulse::High => true,
        }
    }
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        if value {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Debug)]
struct ModuleState {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Default for ModuleState {
    fn default() -> Self {
        Self {
            to: "broadcaster".into(),
            pulse: Default::default(),
            from: "Button".into(),
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const PRESSES: usize = 1000;
    // Parse
    let values: Vec<Module> = parse_file_into(input_file);

    let mut module_map: HashMap<String, Module> = HashMap::new();

    let conjunction_list: Vec<String> = values
        .iter()
        .filter(|i| matches!(i.module_type, ModuleType::Conjunction(_)))
        .map(|i| i.name.clone())
        .collect();

    let mut conjunction_memory: HashMap<String, HashMap<String, bool>> = conjunction_list
        .iter()
        .map(|c| (c.clone(), HashMap::new()))
        .collect();

    for value in values {
        let connections: Vec<String> = value
            .destinations
            .iter()
            .filter(|d| conjunction_list.contains(d))
            .map(|d| d.clone())
            .collect();

        for connection in connections {
            conjunction_memory
                .get_mut(&connection)
                .unwrap()
                .insert(value.name.clone(), false);
        }

        module_map.insert(value.name.clone(), value);
    }

    for conjunction in conjunction_list {
        let module = module_map.get_mut(&conjunction).unwrap();
        match &mut module.module_type {
            ModuleType::Conjunction(map) => *map = conjunction_memory.remove(&conjunction).unwrap(),
            _ => panic!("Should not be here!"),
        }
    }

    // Solve
    let mut high_pulses: usize = 0;
    let mut low_pulses: usize = 0;
    for _ in 0..PRESSES {
        let mut queue = VecDeque::from([ModuleState::default()]);

        while let Some(mut state) = queue.pop_back() {
            if state.pulse == Pulse::High {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            // println!("From: {}, To: {}, Pulse {:?}", state.from, state.to, state.pulse);
            if let Some(module) = module_map.get_mut(&state.to) {
                match &mut module.module_type {
                    ModuleType::FlipFlop(value) => {
                        if state.pulse == Pulse::Low {
                            *value = !*value;
                            state.pulse = (*value).into();
                        } else {
                            continue;
                        }
                    }
                    ModuleType::Broadcast => {}
                    ModuleType::Conjunction(map) => {
                        let memory = map.get_mut(&state.from).unwrap();
                        *memory = state.pulse.to_bool();
                        state.pulse = conjunction_pulse_check(map);
                    }
                }
                for next in module.destinations.iter() {
                    queue.push_front(ModuleState {
                        to: next.clone(),
                        pulse: state.pulse,
                        from: module.name.clone(),
                    });
                }
            }
        }
    }
    let result = low_pulses * high_pulses;

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let special_module: String = "rx".into();

    // Parse
    let values: Vec<Module> = parse_file_into(input_file);

    let mut module_map: HashMap<String, Module> = HashMap::new();

    let conjunction_list: Vec<String> = values
        .iter()
        .filter(|i| matches!(i.module_type, ModuleType::Conjunction(_)))
        .map(|i| i.name.clone())
        .collect();

    let mut conjunction_memory: HashMap<String, HashMap<String, bool>> = conjunction_list
        .iter()
        .map(|c| (c.clone(), HashMap::new()))
        .collect();

    let mut special_conjunction: String = "".into();

    for value in values {
        if value.destinations.contains(&special_module) {
            special_conjunction = value.name.clone();
        }

        let connections: Vec<String> = value
            .destinations
            .iter()
            .filter(|d| conjunction_list.contains(d))
            .map(|d| d.clone())
            .collect();

        for connection in connections {
            conjunction_memory
                .get_mut(&connection)
                .unwrap()
                .insert(value.name.clone(), false);
        }

        module_map.insert(value.name.clone(), value);
    }

    for conjunction in conjunction_list {
        let module = module_map.get_mut(&conjunction).unwrap();
        match &mut module.module_type {
            ModuleType::Conjunction(map) => *map = conjunction_memory.remove(&conjunction).unwrap(),
            _ => panic!("Should not be here!"),
        }
    }

    //  Prepare
    let mut sc_tracker: HashMap<String, usize> =
        match &module_map.get(&special_conjunction).unwrap().module_type {
            ModuleType::Conjunction(map) => map.iter().map(|i| (i.0.clone(), 0)).collect(),
            _ => panic!("Should not be here!"),
        };

    // Solve
    let mut count = 0;
    loop {
        count += 1;
        let mut queue = VecDeque::from([ModuleState::default()]);

        while let Some(mut state) = queue.pop_back() {
            // println!("From: {}, To: {}, Pulse {:?}", state.from, state.to, state.pulse);
            if let Some(module) = module_map.get_mut(&state.to) {
                match &mut module.module_type {
                    ModuleType::FlipFlop(value) => {
                        if state.pulse == Pulse::Low {
                            *value = !*value;
                            state.pulse = (*value).into();
                        } else {
                            continue;
                        }
                    }
                    ModuleType::Broadcast => {}
                    ModuleType::Conjunction(map) => {
                        if state.to == special_conjunction && state.pulse == Pulse::High {
                            if sc_tracker.get(&state.from).is_some_and(|i| *i == 0) {
                                *sc_tracker.get_mut(&state.from).unwrap() = count;
                            }
                        }

                        let memory = map.get_mut(&state.from).unwrap();
                        *memory = state.pulse.to_bool();
                        state.pulse = conjunction_pulse_check(map);
                    }
                }
                for next in module.destinations.iter() {
                    queue.push_front(ModuleState {
                        to: next.clone(),
                        pulse: state.pulse,
                        from: module.name.clone(),
                    });
                }
            }
        }

        if is_finished(&sc_tracker) {
            break;
        }
    }

    let mut result = 0;
    for (_, i) in sc_tracker {
        result = if result == 0 { i } else { lcm(result, i) }
    }

    println!("Result of part 2 is {}", result);
}

fn is_finished(sc_tracker: &HashMap<String, usize>) -> bool {
    sc_tracker.iter().find(|i| *i.1 == 0).is_none()
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_2_path;
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_1_2() {
        let input_path = get_test_input_2_path(file!());
        run(input_path.to_str().unwrap());
    }

    // #[test]
    // fn test_input_part_2() {
    //     let input_path = get_test_input_path(file!());
    //     run2(input_path.to_str().unwrap());
    // }
}
