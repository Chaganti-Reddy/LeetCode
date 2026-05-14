impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let a = s1.as_bytes();
        let b = s2.as_bytes();

        let even_match =
            (a[0] == b[0] && a[2] == b[2]) ||
            (a[0] == b[2] && a[2] == b[0]);

        let odd_match =
            (a[1] == b[1] && a[3] == b[3]) ||
            (a[1] == b[3] && a[3] == b[1]);

        even_match && odd_match
    }
}