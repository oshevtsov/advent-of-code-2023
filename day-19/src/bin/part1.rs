use anyhow::{anyhow, bail, Error, Ok, Result};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let mut lines: VecDeque<&str> = input.lines().map(|l| l.trim()).collect();

    // get rid of any empty lines in the beginning
    while let Some(l) = lines.front() {
        if !l.is_empty() {
            break;
        }
        lines.pop_front();
    }

    // parse input
    let workflows = parse_workflows(&mut lines).unwrap_or_else(|err| panic!("{err}"));
    let ratings = parse_ratings(&mut lines).unwrap_or_else(|err| panic!("{err}"));

    ratings
        .iter()
        .filter_map(|rating| {
            if apply_workflows(rating, &workflows) {
                return Some(rating.values().sum::<usize>());
            }
            None
        })
        .sum()
}

fn parse_ratings(lines: &mut VecDeque<&str>) -> Result<Vec<HashMap<Category, usize>>> {
    let mut ratings: Vec<HashMap<Category, usize>> = Vec::new();
    while let Some(l) = lines.pop_front() {
        if !l.is_empty() && l.len() > 2 {
            let mut rating = HashMap::new();
            for rating_str in l.trim_matches(|c| c == '{' || c == '}').split(',') {
                let cat: Category = rating_str.chars().next().unwrap().into();
                let value: usize = rating_str[2..].parse()?;

                rating.insert(cat, value);
            }
            ratings.push(rating);
        }
    }
    Ok(ratings)
}

fn parse_workflows(lines: &mut VecDeque<&str>) -> Result<HashMap<String, Vec<Rule>>> {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    while let Some(l) = lines.pop_front() {
        if l.is_empty() {
            break;
        }

        if let Some(start_idx) = l.chars().position(|c| c == '{') {
            let name = l[..start_idx].to_owned();
            let mut rules = Vec::new();
            for rule_str in l[start_idx..]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
            {
                let rule = rule_str.parse()?;
                rules.push(rule);
            }

            workflows.insert(name, rules);
        }
    }
    Ok(workflows)
}

fn apply_workflows(
    rating: &HashMap<Category, usize>,
    workflows: &HashMap<String, Vec<Rule>>,
) -> bool {
    let accept = String::from("A");
    let reject = String::from("R");
    let mut curr_wf = String::from("in");

    loop {
        if curr_wf == reject || curr_wf == accept {
            break;
        }

        for rule in workflows[&curr_wf].iter() {
            if rule.condition.is_none() {
                curr_wf = rule.next_workflow_name.clone();
                continue;
            }

            let condition = rule.condition.unwrap();
            if match condition.op {
                Operation::Greater => rating[&condition.cat] > condition.value,
                Operation::LessThan => rating[&condition.cat] < condition.value,
            } {
                curr_wf = rule.next_workflow_name.clone();
                break;
            }
        }
    }

    if curr_wf == accept {
        return true;
    }
    false
}

#[derive(Debug)]
struct Rule {
    next_workflow_name: String,
    condition: Option<Condition>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(':') {
            return Ok(Self {
                next_workflow_name: s.to_owned(),
                condition: None,
            });
        }

        if let Some((condition_str, next_workflow)) = s.split_once(':') {
            return Ok(Self {
                next_workflow_name: next_workflow.to_owned(),
                condition: Some(condition_str.parse()?),
            });
        }

        Err(anyhow!("Failed to parse the rule: {s}"))
    }
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    cat: Category,
    op: Operation,
    value: usize,
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            bail!("Condition must be at least 3-character long");
        }
        let cat: Category = s.chars().nth(0).unwrap().into();
        let op: Operation = s.chars().nth(1).unwrap().into();
        let value: usize = s[2..].parse()?;
        Ok(Self { cat, op, value })
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Greater,
    LessThan,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Greater,
            '<' => Self::LessThan,
            o => panic!("unknown operation: {o}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            c => panic!("unknown category: {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
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
        "#;
        assert_eq!(19114, process(input));
    }
}
