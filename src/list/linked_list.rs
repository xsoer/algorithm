type Link<T> = Option<Box<Node<T>>>;

// 定义节点
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

#[derive(Debug, Clone)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T>
where
    T: std::fmt::Debug,
{
    /// 新建节点初始化
    pub fn new() -> Self {
        Self { head: None }
    }

    /// 遍历所有节点
    pub fn traverse(&self) {
        if self.head.is_none() {
            println!("List is empty");
            // self.head = Some(Box::new(node));
        } else {
            let mut t = self.head.as_ref();
            while t.is_some() {
                let a = t.unwrap();
                println!("{:#?}", a.data);
                t = a.next.as_ref();
            }
            // t.as_mut().unwrap().next = Some(Box::new(node));
        }
        // print!("{},", self.head);
        // if let Some(node) = self.head {
        //     if let Some(next) = node.next {
        //         next.traverse();
        //     }
        // }
        // let mut t = self.next.as_ref();
        // while t.is_some() {
        //     print!("{},", t.unwrap().data);
        //     t = t.unwrap().next.as_ref();
        // }
        println!("");
    }

    /// 递归遍历查找最后一个node
    // fn get_last_node(&mut self) -> &mut Self {
    //     if let Some(ref mut t) = self.next {
    //         t.get_last_node()
    //     } else {
    //         self
    //     }
    // }

    /// 插入节点
    /// 循环遍历链表，找到最后一个节点，把新节点添加到最后一个节点的next上
    /// 1. 声明临时变量存放当前节点 t
    /// 2. while 判断t的next是否为Some
    ///       t = t.next;
    /// 4. 把新节点添加到t.next = Some(Box::new(node))
    pub fn append(&mut self, data: T) {
        // let node = Node { data, next: None };
        // if self.head.is_none() {
        //     self.head = Some(Box::new(node));
        // } else {
        //     let mut t = self.head.as_mut();
        //     while t.is_some() {
        //         t = t.unwrap().next.as_mut();
        //     }
        //     t.as_mut() = Some(Box::new(node));
        // }
        // let node = LinkNode::new(data);
        // self.get_last_node().next = Some(Box::new(node));
    }

    /// 添加一个list
    // pub fn append_list(&mut self, data: Vec<usize>) {
    //     if data.is_empty() {
    //         return;
    //     }
    //     let mut node = LinkNode::new(data[0]);
    //     data[1..]
    //         .into_iter()
    //         .for_each(|x| node.append(x.to_owned()));
    //     self.get_last_node().next = Some(Box::new(node));
    // }

    /// 遍历查找
    // pub fn search(&self, data: usize) -> Option<usize> {
    //     if self.data == data {
    //         return Some(data);
    //     }
    //     let mut t = self.next.as_ref();
    //     while t.is_some() {
    //         // println!("{}", t.unwrap().data);
    //         t = t.unwrap().next.as_ref();
    //         if t.is_some() && t.unwrap().data == data {
    //             return Some(data);
    //         }
    //     }
    //     None
    // }

    /// 删除某个节点
    // pub fn delete_pos(&mut self, pos: usize) {
    //     // 查找对应位置的node，然后删除，链接两端的node
    //     match self.get_pos_node(pos, 0) {
    //         Ok(v) => {
    //             if v.next.is_some() {
    //                 v.next = v.next.clone().unwrap().next.clone();
    //             }
    //         }
    //         Err(e) => println!("{}", e),
    //     }
    // }

    /// 插入到某个位置
    // pub fn insert(&mut self, data: usize, pos: usize) {
    //     // 查找对应的位置，然后重新连接上
    //     let mut n = LinkNode::new(data);
    //     match self.get_pos_node(pos, 0) {
    //         Ok(v) => {
    //             n.next = v.next.clone();
    //             v.next = Some(Box::new(n));
    //         }
    //         Err(e) => println!("{}", e),
    //     }
    // }

    // /// 查询给定位置的node
    // fn get_pos_node(&mut self, pos: usize, mut i: usize) -> Result<&mut Self, String> {
    //     if i == pos {
    //         return Ok(self);
    //     }
    //     i += 1;
    //     if let Some(ref mut t) = self.next {
    //         return t.get_pos_node(pos, i);
    //     }
    //     return Err("未找到位置".into());
    // }

    // fn get_len(&self) -> usize {
    //     let mut i = 1;
    //     let mut t = self.next.as_ref();
    //     while t.is_some() {
    //         i += 1;
    //         t = t.unwrap().next.as_ref();
    //     }
    //     i
    // }

    /*
     *  翻转链表
     *
     *  思路：递归和迭代两种方法
     *  调用自身的就是递归，带有while的就是迭代
     *
     *  1.递归(recursive)
     *
     *  2.迭代(iteration)
     *      新声明一个null值作为新
     */

    // 迭代翻转
    // pub fn reverse_iteration(&mut self) {
    //     let mut cur = self.next.take();
    //     let mut pre = Box::new(self.clone());
    //     pre.next = None;

    //     while cur.is_some() {
    //         let ref mut v = cur.unwrap();
    //         cur = v.next.take();
    //         v.next = Some(pre);
    //         pre = v.clone();
    //     }
    // }

    // 递归翻转
    pub fn reverse_recursive(&mut self) {}
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn t_node() {
        let mut n = List::new();
        println!("append");
        for i in 10..15 {
            n.append(i)
        }
        n.traverse();
        // let now = Instant::now();
        // println!("search");
        // let t = n.search(0);
        // println!("{:?}, {}ms", t, now.elapsed().as_millis());
        // let data = vec![30, 31, 32, 33, 34, 35];
        // println!("append_list");
        // n.append_list(data);
        // n.traverse();
        // println!("insert");
        // n.insert(500, 0);
        // n.traverse();
        // println!("delete");
        // n.delete_pos(4);
        // n.traverse();
        // println!("reverse_iteration");
        // // n.reverse_iteration();
        // n.traverse();
    }
}
