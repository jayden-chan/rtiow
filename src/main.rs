mod vector3;
mod util;

use vector3::Vector3;

fn main() {
    let v = Vector3::new(3.0, 3.0, 3.0);

    let v_norm = v.normalize();

    println!("{:?}", v_norm);
}
