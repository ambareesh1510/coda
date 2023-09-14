mod parser;
mod gen;

fn main() {
    parser::parse_and_print();
    gen::write_wav();
}
