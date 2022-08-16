/**
 * [1656] design an ordered stream
 * https://leetcode.cn/problems/design-an-ordered-stream/
 */

struct OrderedStream {
    data: Vec<String>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            data: vec![String::with_capacity(5); n as usize],
            ptr: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let index = (id_key - 1) as usize;
        self.data[index] = value;

        let mut list: Vec<String> = vec![];
        if self.ptr != id_key as usize {
            return list;
        }

        let length = self.data.len();
        let mut position = self.ptr - 1;
        while position < length && self.data[position].len() == 5 {
            let item = self.data[position].clone();
            list.push(item);
            position += 1;
        }
        self.ptr = position + 1;
        list
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */

#[cfg(test)]
mod tests {
    use super::OrderedStream;

    #[test]
    fn test_1656() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "ccccc".to_string()), Vec::<String>::new());
        assert_eq!(os.insert(1, "aaaaa".to_string()), vec!["aaaaa"]);
        assert_eq!(os.insert(2, "bbbbb".to_string()), vec!["bbbbb", "ccccc"]);
        assert_eq!(os.insert(5, "eeeee".to_string()), Vec::<String>::new());
        assert_eq!(os.insert(4, "ddddd".to_string()), vec!["ddddd", "eeeee"]);
    }
}
