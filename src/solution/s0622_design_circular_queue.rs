/**
 * [622] 设计循环队列
 * 
 * 设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于 FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。它也被称为“环形缓冲器”。
 * 循环队列的一个好处是我们可以利用这个队列之前用过的空间。在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。但是使用循环队列，我们能使用这些空间去存储新的值。
 *
 * 你的实现应该支持如下操作：
 *
 * MyCircularQueue(k): 构造器，设置队列长度为 k 。
 * Front: 从队首获取元素。如果队列为空，返回 -1 。
 * Rear: 获取队尾元素。如果队列为空，返回 -1 。
 * enQueue(value): 向循环队列插入一个元素。如果成功插入则返回真。
 * deQueue(): 从循环队列中删除一个元素。如果成功删除则返回真。
 * isEmpty(): 检查循环队列是否为空。
 * isFull(): 检查循环队列是否已满。
 *
 *
 * 示例：
 *
 * MyCircularQueue circularQueue = new MyCircularQueue(3); // 设置长度为 3
 * circularQueue.enQueue(1);  // 返回 true
 * circularQueue.enQueue(2);  // 返回 true
 * circularQueue.enQueue(3);  // 返回 true
 * circularQueue.enQueue(4);  // 返回 false，队列已满
 * circularQueue.Rear();  // 返回 3
 * circularQueue.isFull();  // 返回 true
 * circularQueue.deQueue();  // 返回 true
 * circularQueue.enQueue(4);  // 返回 true
 * vcircularQueue.Rear();  // 返回 4
 *
 * 提示：
 *
 * 所有的值都在 0 至 1000 的范围内；
 * 操作数将在 1 至 1000 的范围内；
 * 请不要使用内置的队列库。
 */

struct MyCircularQueue {
    data: Vec<i32>,
    capcity: usize,
    front: usize,
    rear: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        let k = k as usize + 1;
        MyCircularQueue { capcity: k, data: vec![0; k], front: 0, rear: 0 }
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.rear = (self.rear + 1) % self.capcity;
        self.data[self.rear] = value;
        true
    }
    
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capcity;
        true
    }
    
    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let head = (self.front + 1) % self.capcity;
        self.data[head]
    }
    
    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.rear]
    }
    
    fn is_empty(&self) -> bool {
        self.front == self.rear
    }
    
    fn is_full(&self) -> bool {
        self.front == (self.rear + 1) % self.capcity
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn test_622() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        assert!(!queue.en_queue(4));
        assert_eq!(queue.rear(), 3);
        assert!(queue.is_full());
        assert!(queue.de_queue());
        assert!(queue.en_queue(4));
        assert_eq!(queue.rear(), 4);
    }
 }