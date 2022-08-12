/**
 * [1282] group the people given the group size they belong to
 * https://leetcode.cn/problems/group-the-people-given-the-group-size-they-belong-to/
 */
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut people_size_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (index, size) in group_sizes.iter().enumerate() {
            let index = index as i32;
            match people_size_map.get_mut(size) {
                None => {
                    let people = Vec::from([index]);
                    people_size_map.insert(*size, people);
                }
                Some(arr) => arr.push(index as i32),
            }
        }
        println!("{:?}", people_size_map);
        let mut groups: Vec<Vec<i32>> = Vec::new();
        for (size, people) in people_size_map {
            let groups_for_size: Vec<&[i32]> = people.chunks(size as usize).collect();
            groups_for_size
                .iter()
                .for_each(|v| groups.push((**v).to_vec()));

            // let group_count = people.len() as i32 / size;
            // for g in 0..group_count {
            //     let mut group = Vec::new();
            //     let mut start = g * size;
            //     for j in 0..size {
            //         let index = (start + j) as usize;
            //         group.push(people[index]);
            //     }
            //     groups.push(group);
            // }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1282() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        );
    }
}
