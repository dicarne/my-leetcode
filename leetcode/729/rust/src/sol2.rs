use std::collections::HashSet;

struct MyCalendar {
    pub tree: HashSet<i64>,
    pub lazy: HashSet<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            tree: HashSet::new(),
            lazy: HashSet::new(),
        }
    }

    fn query(&self, start: i64, end: i64, l: i64, r: i64, idx: i64) -> bool {
        if r < start || l > end {
            return false;
        }
        if self.lazy.contains(&idx) {
            return true;
        }
        if start <= l && r <= end {
            return self.tree.contains(&idx);
        }
        let mid = (l + r) / 2;
        return self.query(start, end, l, mid, 2 * idx)
            || self.query(start, end, mid + 1, r, 2 * idx + 1);
    }

    fn update(&mut self, start: i64, end: i64, l: i64, r: i64, idx: i64) {
        if r < start || end < l {
            return;
        }
        if start <= l && r <= end {
            self.tree.insert(idx);
            self.lazy.insert(idx);
        } else {
            let mid = (l + r) / 2;
            self.update(start, end, l, mid, 2 * idx);
            self.update(start, end, mid + 1, r, 2 * idx + 1);
            self.tree.insert(idx);
            if self.lazy.contains(&(2 * idx)) && self.lazy.contains(&(2 * idx + 1)) {
                self.lazy.insert(idx);
            }
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
       if self.query(start as i64, end as i64 - 1, 0, 90000000000, 1) {
           return false
       }
       self.update(start as i64, end as i64 - 1, 0, 90000000000, 1);

        return true;
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn runall() {
        let mut obj = MyCalendar::new();
        assert_eq!(true, obj.book(10, 20));
        assert_eq!(false, obj.book(15, 25));
        assert_eq!(true, obj.book(20, 30));
    }

    #[test]
    fn run79() {
        let mut obj = MyCalendar::new();
        let q = [
            [47, 50],
            [33, 41],
            [39, 45],
            [33, 42],
            [25, 32],
            [26, 35],
            [19, 25],
            [3, 8],
            [8, 13],
            [18, 27],
        ];
        let a = [
            true, true, false, false, true, false, true, true, true, false,
        ];
        for i in 0..q.len() {
            assert_eq!(a[i], obj.book(q[i][0], q[i][1]))
        }
    }
}
