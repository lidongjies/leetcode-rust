use core::time;

/**
 * [636] exclusive time of function
 * https://leetcode.cn/problems/exclusive-time-of-functions/
 */

pub struct Solution {}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut times: Vec<i32> = vec![0; n as usize];
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(n as usize);
        logs.iter().for_each(|log| {
            let mut iterator = log.split(":");
            let id = iterator.next().unwrap().parse::<usize>().unwrap();
            let command = iterator.next().unwrap();
            let timestamp = iterator.next().unwrap().parse::<i32>().unwrap();

            if command.eq("start") {
                if !stack.is_empty() {
                    let (id, end_timestamp) = stack.last().unwrap();
                    times[*id] += timestamp - end_timestamp;
                    let top = stack.len() - 1;
                    stack[top].1 = timestamp;
                }
                stack.push((id, timestamp))
            } else {
                let (id, end_timestamp) = stack.pop().unwrap();
                times[id] += timestamp - end_timestamp + 1;
                if !stack.is_empty() {
                    let top = stack.len() - 1;
                    stack[top].1 = timestamp + 1;
                }
            }
        });
        times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_636() {
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string()
                ]
            ),
            [3, 4]
        )
    }
}
