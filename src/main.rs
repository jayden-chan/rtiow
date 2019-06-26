mod util;
mod vector3;

use vector3::Vector;

fn main() {
    let v = Vector::new(3.0, 3.0, 3.0);
    let v_norm = v.normalize();

    println!("{:?}", v_norm);
}
