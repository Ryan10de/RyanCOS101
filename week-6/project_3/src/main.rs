fn main() {
    let n = 10;
    for i in 1..=n {
        for j in 1..=12 {
            if j == 1 {
                print!("{:>2} x{:>2} = {:>2}", i, j, i * j);
            } else {
                print!(" {:>2} x{:>2} = {:>2}", i, j, i * j);
            }
        }
        println!();
    }
}
