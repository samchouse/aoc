fn main() {
    let input = include_str!("../data/input.txt");

    let sum = input
        .lines()
        .into_iter()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();

            if digits.len() == 1 {
                return format!("{}{}", digits.first().unwrap(), digits.first().unwrap())
                    .parse::<i32>()
                    .unwrap();
            }

            return format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<i32>()
                .unwrap();
        })
        .sum::<i32>();

    println!("{:#?}", sum)
}
