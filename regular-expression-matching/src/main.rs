use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Char {
    Letter(char),
    String(char),
    Any,
    Anies,
}

impl Char {
    fn from_queue(s: &mut VecDeque<char>) -> Option<Char> {
        let c = s.pop_front()?;

        if let Some(&c0) = s.get(0) {
            if c0 == '*' {
                s.pop_front();
                return Some(
                    match c {
                        'a'..='z' => Char::String(c),
                        '.' => Char::Anies,
                        _ => unreachable!(),
                    }
                );
            }
        }

        Some(match c {
            'a'..='z' => Char::Letter(c),
            '.' => Char::Any,
            _ => unreachable!(),
        })
    }

    fn get_match<'a>(&self, source: &'a Vec<char>, start: usize) -> Option<(&'a [char], usize)> {
        match *self {
            Char::Letter(l) => {
                if l == source[start] {
                    Some((&source[start..=start], 1))
                } else {
                    None
                }
            },
            Char::String(l) => {
                let mut end = 0;
                while start + end < source.len() && source[start + end] == l {
                    end += 1;
                }
                Some((&source[start..(start + end)], end))
            },
            Char::Any => {
                Some((&source[start..=start], 1))
            },
            _ => None,
        }
    }
}

fn is_match(source: Vec<char>, chars: &VecDeque<Char>) -> bool {
    let mut idx = 0;
    let mut found = VecDeque::new();
    let mut chars = chars.iter();

    while let Some(&c) = chars.next() {
        let mut delta = 1;

        let res = match c {
            Char::Anies => {
                if let Some(next) = chars.next() {
                    let mut idy = idx;
                    let mut res = None;
                    let mut n = 0;
                    while idy < source.len() {
                        if let Some(c) = next.get_match(&source, idy) {
                            res = Some((&source[idx..idy], c.0));
                            n = c.1;
                            break;
                        }
                        idy += 1;
                    }
                    delta = idy - idx + n;
                    res
                } else {
                    delta = source.len() - idx;
                    Some((&source[idx..], &source[0..0]))
                }
            },
            _ => {
                c.get_match(&source, idx).map(|s| {delta = s.1; (s.0, &source[0..0])})
            },
        };

        println!("{:?}", res);

        if let Some(ch) = res {
            found.append(&mut ch.0.into_iter().map(|c| *c).collect());
            found.append(&mut ch.1.into_iter().map(|c| *c).collect());
        } else {
            found = VecDeque::new();
        }

        idx += delta;
        if idx >= source.len() {
            break;
        }
    }

    if found.len() == 0 || idx != source.len() || chars.count() > 0 {
        false
    } else {
        true
    }
}

impl Solution {
    pub fn is_match(s: String, mut p: String) -> bool {
        let mut p = p.chars().collect();
        let mut chars = VecDeque::new();
        while let Some(ch) = Char::from_queue(&mut p) {
            chars.push_back(ch);
        }

        is_match(s.chars().collect(), &chars) ||
            is_match(s.chars().rev().collect(), &chars.into_iter().rev().collect())
    }
}

struct Solution();

fn main() {
    let s = "aaa";
    let p = "aaaa";

    let s = s.to_string();
    let p = p.to_string();

    println!("{}", Solution::is_match(s, p));
}