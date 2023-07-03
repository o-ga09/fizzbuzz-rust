mod fizzbuzz;

fn main() {
    for i in 0..100 {
        let res = fizzbuzz::convert(i+1);
        println!("{} : {}",i+1,res);
    }
}