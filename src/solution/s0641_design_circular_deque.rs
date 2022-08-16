/**
* [641] design circular deque
* https://leetcode.cn/problems/design-circular-deque/
*
* 设计实现双端队列。
*
* 实现 MyCircularDeque 类:
*
* MyCircularDeque(int k) ：构造函数,双端队列最大为 k 。
* boolean insertFront()：将一个元素添加到双端队列头部。 如果操作成功返回 true ，否则返回 false 。
* boolean insertLast() ：将一个元素添加到双端队列尾部。如果操作成功返回 true ，否则返回 false 。
* boolean deleteFront() ：从双端队列头部删除一个元素。 如果操作成功返回 true ，否则返回 false 。
* boolean deleteLast() ：从双端队列尾部删除一个元素。如果操作成功返回 true ，否则返回 false 。
* int getFront() )：从双端队列头部获得一个元素。如果双端队列为空，返回 -1 。
* int getRear() ：获得双端队列的最后一个元素。 如果双端队列为空，返回 -1 。
* boolean isEmpty() ：若双端队列为空，则返回 true ，否则返回 false  。
* boolean isFull() ：若双端队列满了，则返回 true ，否则返回 false 。
*
*
* 示例 1：
*
* 输入
* ["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
* [[3], [1], [2], [3], [4], [], [], [], [4], []]
* 输出
* [null, true, true, true, false, 2, true, true, true, 4]
*
* 解释
* MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
* circularDeque.insertLast(1);			        // 返回 true
* circularDeque.insertLast(2);			        // 返回 true
* circularDeque.insertFront(3);			        // 返回 true
* circularDeque.insertFront(4);			        // 已经满了，返回 false
* circularDeque.getRear();  				// 返回 2
* circularDeque.isFull();				        // 返回 true
* circularDeque.deleteLast();			        // 返回 true
* circularDeque.insertFront(4);			        // 返回 true
* circularDeque.getFront();				// 返回 4

* 提示：
*
* 1 <= k <= 1000
* 0 <= value <= 1000
* insertFront, insertLast, deleteFront, deleteLast, getFront, getRear, isEmpty, isFull  调用次数不大于 2000 次
*/
struct MyCircularDeque {
    data: Vec<i32>,
    capacity: usize,
    front: usize,
    rear: usize,
}

/**
 * 1. 队列空一位可以用来判断队列满、空
 * 2. 队列头指针和尾指针的移动是不同的，这里是根据题意头指针左移动后赋值，尾指针赋值后右移
 * 3. 防止指针溢出，要对 self.capacity 取模，在 rust 里 (self.rear - 1 + self.capacity) % self.capacity 编译会报错
 * 要写成 (self.rear + self.capacity + 1) % self.capacity 才行
 * 4. Vec::with_capacity(capacity) 是有容量，但是 length 为 0，这时候用 data[0] 是赋值不了的，编译报错，一定要用 vec![0; capacity]
 */

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let capacity = (k + 1) as usize;
        MyCircularDeque {
            data: vec![0; capacity],
            capacity,
            front: 0,
            rear: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.data[self.front] = value;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.capacity - 1) % self.capacity;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let rear = (self.rear + self.capacity - 1) % self.capacity;
        self.data[rear]
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn is_full(&self) -> bool {
        self.front == (self.rear + 1) % self.capacity
    }
}
