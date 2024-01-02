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

    fn next(&self, rating_range: RatingRange) -> Vec<RatingRange> {
        let mut rtn = Vec::new();
        match self.rule_type {
            RuleType::AcceptAll => {
                rtn.push(RatingRange {
                    workflow: self.to.clone(),
                    rule: 0,
                    x: rating_range.x,
                    m: rating_range.m,
                    a: rating_range.a,
                    s: rating_range.s,
                });
            }
            RuleType::Smaller => {
                let mut rr: Vec<RatingRange> = self.split_smaller_range(rating_range);
                rtn.append(&mut rr);
            }
            RuleType::Greater => {
                let mut rr: Vec<RatingRange> = self.split_greater_range(rating_range);
                rtn.append(&mut rr);
            }
        }

        rtn
    }

    fn split_smaller_range(&self, rating_range: RatingRange) -> Vec<RatingRange> {
        let mut rtn = Vec::new();
        match self.left.unwrap() {
            'x' => {
                if rating_range.x.0 < self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: (
                            rating_range.x.0,
                            rating_range.x.1.min(self.right.unwrap() - 1),
                        ),
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                    if rating_range.x.1 >= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: (self.right.unwrap(), rating_range.x.1),
                            m: rating_range.m,
                            a: rating_range.a,
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            'm' => {
                if rating_range.m.0 < self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: (
                            rating_range.m.0,
                            rating_range.m.1.min(self.right.unwrap() - 1),
                        ),
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                    if rating_range.m.1 >= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: (self.right.unwrap(), rating_range.m.1),
                            a: rating_range.a,
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            'a' => {
                if rating_range.a.0 < self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: (
                            rating_range.a.0,
                            rating_range.a.1.min(self.right.unwrap() - 1),
                        ),
                        s: rating_range.s,
                    });
                    if rating_range.a.1 >= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: rating_range.m,
                            a: (self.right.unwrap(), rating_range.a.1),
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    })
                }
            }
            's' => {
                if rating_range.s.0 < self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: (
                            rating_range.s.0,
                            rating_range.s.1.min(self.right.unwrap() - 1),
                        ),
                    });
                    if rating_range.s.1 >= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: rating_range.m,
                            a: rating_range.a,
                            s: (self.right.unwrap(), rating_range.s.1),
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            _ => panic!("Should not be here"),
        };
        rtn
    }

    fn split_greater_range(&self, rating_range: RatingRange) -> Vec<RatingRange> {
        let mut rtn = Vec::new();
        match self.left.unwrap() {
            'x' => {
                if rating_range.x.1 > self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: (
                            rating_range.x.0.max(self.right.unwrap() + 1),
                            rating_range.x.1,
                        ),
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                    if rating_range.x.0 <= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: (rating_range.x.0, self.right.unwrap()),
                            m: rating_range.m,
                            a: rating_range.a,
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            'm' => {
                if rating_range.m.1 > self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: (
                            rating_range.m.0.max(self.right.unwrap() + 1),
                            rating_range.m.1,
                        ),
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                    if rating_range.m.0 <= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: (rating_range.m.0, self.right.unwrap()),
                            a: rating_range.a,
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            'a' => {
                if rating_range.a.1 > self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: (
                            rating_range.a.0.max(self.right.unwrap() + 1),
                            rating_range.a.1,
                        ),
                        s: rating_range.s,
                    });
                    if rating_range.a.0 <= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: rating_range.m,
                            a: (rating_range.a.0, self.right.unwrap()),
                            s: rating_range.s,
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            's' => {
                if rating_range.s.1 > self.right.unwrap() {
                    rtn.push(RatingRange {
                        workflow: self.to.clone(),
                        rule: 0,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: (
                            rating_range.s.0.max(self.right.unwrap() + 1),
                            rating_range.s.1,
                        ),
                    });
                    if rating_range.s.0 <= self.right.unwrap() {
                        rtn.push(RatingRange {
                            workflow: rating_range.workflow.clone(),
                            rule: rating_range.rule + 1,
                            x: rating_range.x,
                            m: rating_range.m,
                            a: rating_range.a,
                            s: (rating_range.s.0, self.right.unwrap()),
                        });
                    }
                } else {
                    rtn.push(RatingRange {
                        workflow: rating_range.workflow.clone(),
                        rule: rating_range.rule + 1,
                        x: rating_range.x,
                        m: rating_range.m,
                        a: rating_range.a,
                        s: rating_range.s,
                    });
                }
            }
            _ => panic!("Should not be here"),
        };
        rtn
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

#[derive(Debug, Clone)]
struct RatingRange {
    pub workflow: String,
    pub rule: usize,
    pub x: (usize, usize),
    pub m: (usize, usize),
    pub a: (usize, usize),
    pub s: (usize, usize),
}

impl RatingRange {
    pub fn init() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
            workflow: "in".into(),
            rule: 0,
        }
    }

    fn product(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
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
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut rating_ranges = vec![RatingRange::init()];

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            break;
        }

        let workflow: Workflow = line.into();
        workflows.insert(workflow.name.clone(), workflow);
    }
    let mut accepted = Vec::new();

    // Solve
    while let Some(rating_range) = rating_ranges.pop() {
        match rating_range.workflow.as_str() {
            "R" => {
                continue;
            }
            "A" => {
                accepted.push(rating_range);
                continue;
            }
            _ => {}
        }

        // println!("Use Workflow: {}, Rule: {} s: {}", &rating_range.workflow, rating_range.rule, rating_range.s.0);
        let workflow = workflows.get(&rating_range.workflow).unwrap();
        let rule = workflow.rules.get(rating_range.rule).unwrap();

        let mut next = rule.next(rating_range);
        rating_ranges.append(&mut next);
    }

    // Result
    let result: usize = accepted.iter().map(|a| a.product()).sum();

    println!("Result of part 2 is {}", result);
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
