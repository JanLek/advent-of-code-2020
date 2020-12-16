#![deny(clippy::all, clippy::pedantic)]

use regex::Regex;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    field: String,
    valid_ranges: Vec<RangeInclusive<usize>>,
}

impl Rule {
    fn field_value_is_valid(&self, field_value: usize) -> bool {
        self.valid_ranges
            .iter()
            .any(|range| range.contains(&field_value))
    }
}

#[derive(Clone, Debug)]
struct Ticket {
    field_values: Vec<usize>,
}

impl Ticket {
    fn invalid_field_values(&self, rules: &Vec<Rule>) -> Vec<usize> {
        self.field_values
            .iter()
            .filter(|field_value| {
                !rules.iter().any(|rule| {
                    rule.valid_ranges
                        .iter()
                        .any(|range| range.contains(field_value))
                })
            })
            .map(|&field_value| field_value)
            .collect()
    }

    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        self.invalid_field_values(rules).is_empty()
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn main() {
    let input = parse_input(INPUT);

    let ticket_scanning_error_rate: usize = input
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.invalid_field_values(&input.rules))
        .sum();
    println!(
        "Part 1 - Ticket scanning error rate = {}",
        ticket_scanning_error_rate
    );

    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.is_valid(&input.rules))
        .collect();

    let field_order = ticket_field_order(&valid_tickets, &input.rules);
    let result: usize = field_order
        .iter()
        .enumerate()
        .filter(|(_index, field)| field.starts_with("departure"))
        .map(|(index, _field)| input.your_ticket.field_values[index])
        .product();

    println!("Part 2 - result = {}", result);
}

fn parse_input(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let rules_part = parts.next().unwrap();
    let your_ticket_part = parts.next().unwrap();
    let nearby_tickets_part = parts.next().unwrap();

    let rules = rules_part.split("\n").map(|line| {
        let rule_pattern = Regex::new(r"(?P<field>[a-z ]+): (?P<range1_start>\d+)-(?P<range1_end>\d+) or (?P<range2_start>\d+)-(?P<range2_end>\d+)").unwrap();
        let captures = rule_pattern.captures(line).unwrap();
        let field = captures.name("field").unwrap().as_str().to_string();
        let range1_start = captures.name("range1_start").unwrap().as_str().parse().unwrap();
        let range1_end = captures.name("range1_end").unwrap().as_str().parse().unwrap();
        let range2_start = captures.name("range2_start").unwrap().as_str().parse().unwrap();
        let range2_end = captures.name("range2_end").unwrap().as_str().parse().unwrap();
        let valid_ranges = vec![range1_start..=range1_end, range2_start..=range2_end];
        Rule { field, valid_ranges }
    }).collect();

    let your_ticket_field_values = your_ticket_part
        .split("\n")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|field_value| field_value.parse().unwrap())
        .collect();
    let your_ticket = Ticket {
        field_values: your_ticket_field_values,
    };

    let nearby_tickets = nearby_tickets_part
        .split("\n")
        .skip(1)
        .map(|line| {
            let field_values = line
                .split(",")
                .map(|field_value| field_value.parse().unwrap())
                .collect();
            Ticket { field_values }
        })
        .collect();

    Input {
        rules,
        your_ticket,
        nearby_tickets,
    }
}

fn ticket_field_order(tickets: &Vec<&Ticket>, rules: &Vec<Rule>) -> Vec<String> {
    let mut rules_by_index: Vec<Vec<Rule>> = (0..rules.len()).map(|_| rules.clone()).collect();

    for ticket in tickets {
        for (field_index, field_value) in ticket.field_values.iter().enumerate() {
            rules_by_index[field_index].retain(|rule| rule.field_value_is_valid(*field_value));
        }
    }

    loop {
        let matched_rules: Vec<Rule> = rules_by_index
            .iter()
            .filter(|rules| rules.len() == 1)
            .map(|rules| rules[0].clone())
            .collect();

        for rules in rules_by_index.iter_mut() {
            if rules.len() > 1 {
                rules.retain(|rule| !matched_rules.contains(rule))
            }
        }

        if rules_by_index.iter().all(|rules| rules.len() <= 1) {
            break;
        }
    }

    rules_by_index
        .iter()
        .map(|rules| rules[0].field.clone())
        .collect()
}
