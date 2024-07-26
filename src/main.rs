mod utils;

fn main() {
    use utils::get_arguments;
    let args = get_arguments();
    println!("Searching for {} in {}", args.query(), args.filepath());
}
