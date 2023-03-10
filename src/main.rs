use crate::cgol::CGOL;

mod cgol;

fn main() {
    let mut cgol = CGOL::new(5,5);
    cgol.randomize();
    println!("{}",&cgol.get_cmat_as_ascii());
    println!("");
    cgol.step();
    println!("{}",&cgol.get_cmat_as_ascii());
    println!("");
}
