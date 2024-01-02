use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, PartialEq, Eq)]
enum RuleType {
    AcceptAll,
    Smaller,
    Greater,
}

#[derive(Debug)]
struct Rule {
    rule_type: RuleType,
    left: Option<char>,
    right: Option<usize>,
    to: String,
}
impl Rule {
    fn check(&self, rating: &Rating) -> bool {
        match self.rule_type {
            RuleType::AcceptAll => true,
            RuleType::Smaller => self.smaller(rating),
            RuleType::Greater => self.greater(rating),
        }
    }

    fn smaller(&self, rating: &Rating) -> bool {
        match self.left.unwrap() {
            'x' => rating.x < self.right.unwrap(),
            'm' => rating.m < self.right.unwrap(),
            'a' => rating.a < self.right.unwrap(),
            's' => rating.s < self.right.unwrap(),
            _ => panic!("Should not be here"),
        }
    }

    fn greater(&self, rating: &Rating) -> bool {
        match self.left.unwrap() {
            'x' => rating.x > self.right.unwrap(),
            'm' => rating.m > self.right.unwrap(),
            'a' => rating.a > self.right.unwrap(),
            's' => rating.s > self.right.unwrap(),
            _ => panic!("Should not be here"),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<String> for Workflow {
    fn from(value: String) -> Self {
        let mut split = value.split('{');
        let name = split.next().unwrap().into();
        let mut rules_part = split.next().unwrap();
        rules_part = &rules_part[0..rules_part.len() - 1];
        let rule_split = rules_part.split(',');

        let mut rules = Vec::new();

        for rule_str in rule_split {
            let mut char_itr = rule_str.chars();
            let first_char: char = char_itr.next().unwrap();
            if first_char == 'R' || first_char == 'A' {
                let to = first_char.into();

                let rule = Rule {
                    rule_type: RuleType::AcceptAll,
                    left: None,
                    right: None,
                    to,
                };
                rules.push(rule);
                continue;
            }

            let second_char = char_itr.next();

            let rule_type = match second_char {
                Some('<') => RuleType::Smaller,
                Some('>') => RuleType::Greater,
                Some(_) => RuleType::AcceptAll,
                _ => panic!("Should not be here."),
            };

            if rule_type == RuleType::AcceptAll {
                let rule = Rule {
                    rule_type: RuleType::AcceptAll,
                    left: None,
                    right: None,
                    to: rule_str.into(),
                };
                rules.push(rule);
                continue;
            }

            let mut rule_parts = rule_str.split(['<', '>', ':']);
            let left: Option<char> = Some(first_char);
            let right: Option<usize> = Some(rule_parts.nth(1).unwrap().parse().unwrap());
            let to = rule_parts.next().unwrap().into();

            let rule = Rule {
                rule_type,
                left,
                right,
                to,
            };
            rules.push(rule);
        }

        Self { name, rules }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rating {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}
impl Rating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<String> for Rating {
    fn from(value: String) -> Self {
        let value_part = &value[1..value.len() - 1];
        let mut split = value_part.split([',', '=']);
        let x = split.nth(1).unwrap().parse().unwrap();
        let m = split.nth(1).unwrap().parse().unwrap();
        let a = split.nth(1).unwrap().parse().unwrap();
        let s = split.nth(1).unwrap().parse().unwrap();

        Self { x, m, a, s }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut parse_ratings = false;
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut ratings: Vec<Rating> = Vec::new();
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            parse_ratings = true;
            continue;
        }

        if parse_ratings {
            let rating: Rating = line.into();
            ratings.push(rating);
        } else {
            let workflow: Workflow = line.into();
            workflows.insert(workflow.name.clone(), workflow);
        }
    }

    // Solve
    'ratings: for rating in ratings {
        let mut current_workflow = "in";

        'workflow: loop {
            let workflow = workflows.get(current_workflow).unwrap();

            for rule in workflow.rules.iter() {
                if rule.check(&rating) {
                    current_workflow = &rule.to;
                    match current_workflow {
                        "R" => {
                            continue 'ratings;
                        }
                        "A" => {
                            result += rating.sum();
                            continue 'ratings;
                        }
                        _ => {
                            continue 'workflow;
                        }
                    }
                }
            }
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
       // Preamble
       let mut parse_ratings = false;
       let mut workflows: HashMap<String, Workflow> = HashMap::new();
       let mut ratings: Vec<Rating> = Vec::new();
       let mut rejected = Vec::new();
       let mut accepted = Vec::new();
   
       // Parse
       let file = File::open(input_file).unwrap();
       let reader = BufReader::new(file);
   
       for line in reader.lines() {
           let line = line.unwrap().trim().to_string();
   
           if line.is_empty() {
               parse_ratings = true;
               continue;
           }
   
           if parse_ratings {
               let rating: Rating = line.into();
               ratings.push(rating);
           } else {
               let workflow: Workflow = line.into();
               workflows.insert(workflow.name.clone(), workflow);
           }
       }
   
       // Solve
       'ratings: for rating in ratings {
           let mut current_workflow = "in";
   
           'workflow: loop {
               let workflow = workflows.get(current_workflow).unwrap();
   
               for rule in workflow.rules.iter() {
                   if rule.check(&rating) {
                       current_workflow = &rule.to;
                       match current_workflow {
                           "R" => {
                               rejected.push(rating);
                               continue 'ratings;
                           }
                           "A" => {
                               accepted.push(rating);
                               continue 'ratings;
                           }
                           _ => {
                               continue 'workflow;
                           }
                       }
                   }
               }
           }
       }
   
       // Result
       let result:usize = accepted.iter().map(|a| a.sum()).sum();
   
       println!("Result of part 1 is {}", result);
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
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
