/**
 * [735] asteroid collision
 * https://leetcode.cn/problems/asteroid-collision/
 */

pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut statck: Vec<i32> = vec![];
        for (index, asteroid) in asteroids.iter().enumerate() {
            let mut alive = true;
            // 只有行星负方向移动，栈里还有正向移动的时候，才怕断那个会爆炸
            // 这一行代码基就是这个题的核心思路，栈是该题的核心数据结构
            while (alive && *asteroid < 0 && statck.len() > 0 && *statck.last().unwrap() > 0) {
                let lastest = *statck.last().unwrap();
                alive = lastest < -asteroid;
                if lastest <= -asteroid {
                    statck.pop();
                }
            }
            if alive {
                statck.push(*asteroid);
            }
        }
        statck
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0735() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }
}
