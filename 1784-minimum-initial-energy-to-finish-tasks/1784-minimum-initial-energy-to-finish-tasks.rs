impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_by(|a, b| {
            (b[1] - b[0]).cmp(&(a[1] - a[0]))
        });

        let mut energy = 0;
        let mut current = 0;

        for task in tasks {
            let actual = task[0];
            let minimum = task[1];

            if current < minimum {
                energy += minimum - current;
                current = minimum;
            }

            current -= actual;
        }

        energy
    }
}