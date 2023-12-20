use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, digit1, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    IResult,
};
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow<'s> {
    name: &'s str,
    rules: Vec<Rule<'s>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Rule<'s> {
    check: Option<RuleCheck>,
    action: Action<'s>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RuleCheck {
    category: Category,
    condition: Condition,
    value: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Condition {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action<'s> {
    GotoWorkflow(&'s str),
    Reject,
    Accept,
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x={},m={},a={},s={}}}", self.x, self.m, self.a, self.s)
    }
}

fn parse<'s>(input: &'s str) -> (HashMap<&'s str, Workflow<'s>>, Vec<Part>) {
    let mut halves = input.split("\n\n");
    let workflows = halves
        .next()
        .unwrap()
        .lines()
        .map(parser::parse_workflow)
        .map(|w| (w.name, w))
        .collect();
    let parts = halves
        .next()
        .unwrap()
        .lines()
        .map(parser::parse_part)
        .collect();
    (workflows, parts)
}

mod parser {
    use nom::bytes::complete::tag;

    use super::*;

    fn workflow_name(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_ascii_lowercase())(input)
    }

    fn category(input: &str) -> IResult<&str, Category> {
        let (input, c) = one_of("xmas")(input)?;
        Ok((
            input,
            match c {
                'x' => Category::ExtremelyCoolLooking,
                'm' => Category::Musical,
                'a' => Category::Aerodynamic,
                's' => Category::Shiny,
                _ => unreachable!(),
            },
        ))
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        let (input, c) = one_of("<>")(input)?;
        Ok((
            input,
            match c {
                '<' => Condition::LessThan,
                '>' => Condition::GreaterThan,
                _ => unreachable!(),
            },
        ))
    }

    fn value(input: &str) -> IResult<&str, usize> {
        digit1(input).map(|(input, digits)| (input, digits.parse().unwrap()))
    }

    fn rule_check(input: &str) -> IResult<&str, RuleCheck> {
        let (input, category) = category(input)?;
        let (input, condition) = condition(input)?;
        let (input, value) = value(input)?;
        Ok((
            input,
            RuleCheck {
                category,
                condition,
                value,
            },
        ))
    }

    fn accept(input: &str) -> IResult<&str, Action> {
        char('A')(input).map(|(input, _)| (input, Action::Accept))
    }

    fn reject(input: &str) -> IResult<&str, Action> {
        char('R')(input).map(|(input, _)| (input, Action::Reject))
    }

    fn goto(input: &str) -> IResult<&str, Action> {
        workflow_name(input).map(|(input, workflow)| (input, Action::GotoWorkflow(workflow)))
    }

    fn action(input: &str) -> IResult<&str, Action> {
        preceded(opt(char(':')), alt((accept, reject, goto)))(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, check) = opt(rule_check)(input)?;
        let (input, action) = action(input)?;
        Ok((input, Rule { check, action }))
    }

    fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
        separated_list1(char(','), rule)(input)
    }

    pub fn parse_workflow(line: &str) -> Workflow {
        let workflow = pair(workflow_name, delimited(char('{'), rules, char('}')))(line);
        match workflow {
            Ok((_, (name, rules))) => Workflow { name, rules },
            Err(_) => panic!("Failed to parse workflow: '{line}'"),
        }
    }

    fn part_categories(input: &str) -> IResult<&str, Part> {
        let (input, _) = tag("x=")(input)?;
        let (input, x) = value(input)?;
        let (input, _) = tag(",m=")(input)?;
        let (input, m) = value(input)?;
        let (input, _) = tag(",a=")(input)?;
        let (input, a) = value(input)?;
        let (input, _) = tag(",s=")(input)?;
        let (input, s) = value(input)?;
        Ok((input, Part { x, m, a, s }))
    }

    pub fn parse_part(line: &str) -> Part {
        let part = delimited(char('{'), part_categories, char('}'))(line);
        match part {
            Ok((_, part)) => part,
            Err(_) => panic!("Failed to parse part: '{line}'"),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn can_parse_workflow_name() {
            let (input, name) = workflow_name("px{a<2006:qkq,m>2090:A,rfg}").unwrap();
            assert_eq!(input, "{a<2006:qkq,m>2090:A,rfg}");
            assert_eq!(name, "px");
        }

        #[test]
        fn can_parse_category() {
            let (input, cat) = category("x").unwrap();
            assert_eq!(input, "");
            assert_eq!(cat, Category::ExtremelyCoolLooking);
            let (input, cat) = category("m").unwrap();
            assert_eq!(input, "");
            assert_eq!(cat, Category::Musical);
            let (input, cat) = category("a").unwrap();
            assert_eq!(input, "");
            assert_eq!(cat, Category::Aerodynamic);
            let (input, cat) = category("s").unwrap();
            assert_eq!(input, "");
            assert_eq!(cat, Category::Shiny);
        }

        #[test]
        fn can_parse_condition() {
            let (input, cond) = condition("<").unwrap();
            assert_eq!(input, "");
            assert_eq!(cond, Condition::LessThan);
            let (input, cond) = condition(">").unwrap();
            assert_eq!(input, "");
            assert_eq!(cond, Condition::GreaterThan);
        }

        #[test]
        fn can_parse_value() {
            let (input, value) = value("123").unwrap();
            assert_eq!(input, "");
            assert_eq!(value, 123);
        }

        #[test]
        fn can_parse_rule_check() {
            let (input, check) = rule_check("a<2006").unwrap();
            assert_eq!(input, "");
            assert_eq!(check.category, Category::Aerodynamic);
            assert_eq!(check.condition, Condition::LessThan);
            assert_eq!(check.value, 2006);
        }

        #[test]
        fn can_parse_accept_action() {
            let (input, action) = accept("A").unwrap();
            assert_eq!(input, "");
            assert_eq!(action, Action::Accept);
        }

        #[test]
        fn can_parse_reject_action() {
            let (input, action) = reject("R").unwrap();
            assert_eq!(input, "");
            assert_eq!(action, Action::Reject);
        }

        #[test]
        fn can_parse_goto_action() {
            let (input, action) = goto("rfg").unwrap();
            assert_eq!(input, "");
            assert_eq!(action, Action::GotoWorkflow("rfg"));
        }

        #[test]
        fn can_parse_action() {
            let (input, act) = action("A").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::Accept);

            let (input, act) = action(":A").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::Accept);

            let (input, act) = action("R").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::Reject);

            let (input, act) = action(":R").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::Reject);

            let (input, act) = action("rfg").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::GotoWorkflow("rfg"));

            let (input, act) = action(":qkq").unwrap();
            assert_eq!(input, "");
            assert_eq!(act, Action::GotoWorkflow("qkq"));
        }

        #[test]
        fn can_parse_rule() {
            let (input, r) = rule("a<2006:qkq").unwrap();
            assert_eq!(input, "");
            assert_eq!(r.check.unwrap().category, Category::Aerodynamic);
            assert_eq!(r.check.unwrap().condition, Condition::LessThan);
            assert_eq!(r.check.unwrap().value, 2006);
            assert_eq!(r.action, Action::GotoWorkflow("qkq"));

            let (input, r) = rule("m>2090:A").unwrap();
            assert_eq!(input, "");
            assert_eq!(r.check.unwrap().category, Category::Musical);
            assert_eq!(r.check.unwrap().condition, Condition::GreaterThan);
            assert_eq!(r.check.unwrap().value, 2090);
            assert_eq!(r.action, Action::Accept);

            let (input, r) = rule("rfg").unwrap();
            assert_eq!(input, "");
            assert!(r.check.is_none());
            assert_eq!(r.action, Action::GotoWorkflow("rfg"));
        }

        #[test]
        fn can_parse_rules() {
            let (input, rules) = rules("a<2006:qkq,m>2090:A,rfg").unwrap();
            assert_eq!(input, "");
            assert_eq!(rules.len(), 3);
            assert_eq!(rules[0].check.unwrap().category, Category::Aerodynamic);
            assert_eq!(rules[0].check.unwrap().condition, Condition::LessThan);
            assert_eq!(rules[0].check.unwrap().value, 2006);
            assert_eq!(rules[0].action, Action::GotoWorkflow("qkq"));
            assert_eq!(rules[1].check.unwrap().category, Category::Musical);
            assert_eq!(rules[1].check.unwrap().condition, Condition::GreaterThan);
            assert_eq!(rules[1].check.unwrap().value, 2090);
            assert_eq!(rules[1].action, Action::Accept);
            assert!(rules[2].check.is_none());
            assert_eq!(rules[2].action, Action::GotoWorkflow("rfg"));
        }

        #[test]
        fn can_parse_workflow() {
            let workflow = "px{a<2006:qkq,m>2090:A,rfg}";
            let parsed = parse_workflow(workflow);
            assert_eq!(parsed.name, "px");
            assert_eq!(parsed.rules.len(), 3);
            assert_eq!(
                parsed.rules[0].check.unwrap().category,
                Category::Aerodynamic
            );
            assert_eq!(
                parsed.rules[0].check.unwrap().condition,
                Condition::LessThan
            );
            assert_eq!(parsed.rules[0].check.unwrap().value, 2006);
            assert_eq!(parsed.rules[0].action, Action::GotoWorkflow("qkq"));
            assert_eq!(parsed.rules[1].check.unwrap().category, Category::Musical);
            assert_eq!(
                parsed.rules[1].check.unwrap().condition,
                Condition::GreaterThan
            );
            assert_eq!(parsed.rules[1].check.unwrap().value, 2090);
            assert_eq!(parsed.rules[1].action, Action::Accept);
            assert!(parsed.rules[2].check.is_none());
            assert_eq!(parsed.rules[2].action, Action::GotoWorkflow("rfg"));
        }

        #[test]
        fn can_parse_part() {
            let part = "{x=787,m=2655,a=1222,s=2876}";
            let parsed = parse_part(part);
            assert_eq!(parsed.x, 787);
            assert_eq!(parsed.m, 2655);
            assert_eq!(parsed.a, 1222);
            assert_eq!(parsed.s, 2876);
        }
    }
}

fn is_part_accepted<'s>(workflows: &'s HashMap<&'s str, Workflow<'s>>, part: Part) -> bool {
    let mut workflow = workflows.get("in").expect("in workflow exists");
    'workflows: loop {
        for rule in workflow.rules.iter() {
            let apply_rule = if let Some(check) = rule.check {
                let value = match check.category {
                    Category::ExtremelyCoolLooking => part.x,
                    Category::Musical => part.m,
                    Category::Aerodynamic => part.a,
                    Category::Shiny => part.s,
                };
                match check.condition {
                    Condition::LessThan => value < check.value,
                    Condition::GreaterThan => value > check.value,
                }
            } else {
                // always apply rule if there is no check
                // (this should only happen on the last rule)
                true
            };
            if apply_rule {
                match rule.action {
                    Action::Accept => return true,
                    Action::Reject => return false,
                    Action::GotoWorkflow(name) => {
                        workflow = workflows.get(name).expect("workflow exists");
                        continue 'workflows;
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    parts
        .into_par_iter()
        .filter(|part| is_part_accepted(&workflows, *part))
        .map(|part| part.rating())
        .sum()
}

pub fn part2(_input: &str) -> usize {
    0
}

pub fn run(input: &str) -> (Option<usize>, Option<usize>) {
    (Some(part1(input)), None)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = r#"px{a<2006:qkq,m>2090:A,rfg}
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

    #[test]
    fn day19_part1_sample() {
        assert_eq!(part1(SAMPLE), 19114);
    }

    #[test]
    #[ignore]
    fn day19_part2_sample() {
        assert_eq!(part2(SAMPLE), 167409079868000);
    }
}
