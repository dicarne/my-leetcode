struct MyCalendar {
    pub db: Vec<[i32; 2]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        MyCalendar { db: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for ia in self.db.iter() {
            let [bg, ed] = ia;
            if !(start >= *ed || end <= *bg) {
                return false;
            }
        }
        self.db.push([start, end]);
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
