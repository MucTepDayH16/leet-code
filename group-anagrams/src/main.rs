    use std::{
        collections::{BTreeMap, BTreeSet},
        cmp::*,
        iter::*,
    };

    fn ga(s: &String) -> String {
        let mut chars = s.clone().chars().collect::<Vec<char>>();
        chars.sort();
        chars.into_iter().collect()
    }

    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut groups = BTreeMap::<String, Vec<String>>::new();

            for s in strs {
                let a = ga(&s);
                match groups.get_mut(&a) {
                    Some(g) => g.push(s),
                    None => { groups.insert(a, vec![s]); },
                }
            }

            groups.into_values().collect()
        }
    }

struct Solution();

fn main() {
    let strs = ["ddddddddddg","dgggggggggg"];
    let strs = strs.into_iter().map(|s| s.to_string()).collect();
    let result = Solution::group_anagrams(strs);

    println!("{:?}", result);
}
