impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let (balance, blanks) = moves.chars().fold((0, 0), |(bal, blank), c| {
            match c {
                'L' => (bal - 1, blank),
                'R' => (bal + 1, blank),
                '_' => (bal, blank + 1),
                _ => unreachable!(),
            }
        });

        (balance as i32).abs() + blanks
    }
}