// https://adventofcode.com/2022/day/10

fn main() {
    let input = include_str!("../../input/day10.txt").trim();

    let (_, _, signal_strengths_sum, crt) =
        input.split('\n').fold(
            (0, 1, 0, [['.'; 40]; 6]),
            |(mut cycle, mut x, mut signal_strengths_sum, mut crt), line| {
                'instruction_loop: for k in 1..3 {
                    cycle += 1;
                    if (cycle + 20) % 40 == 0 {
                        signal_strengths_sum += cycle * x;
                    }
                    let pixel_idx = ((cycle - 1) % 40) as usize;
                    if [x - 1, x, x + 1].contains(&(pixel_idx as i32)) {
                        crt[((cycle - 1) / 40) as usize][pixel_idx] = '#';
                    }
                    if line.eq("noop") {
                        break 'instruction_loop;
                    } else if k == 2 {
                        x += line[5..].parse::<i32>().unwrap();
                    };
                }
                (cycle, x, signal_strengths_sum, crt)
            },
        );

    println!("P1: {}", signal_strengths_sum);
    println!("P2:");
    crt.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));
}
