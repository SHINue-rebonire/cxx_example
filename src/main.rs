mod cxx;

fn main() {
    let result = cxx::add(1, 2);
    println!("Result: {}", result);
}
