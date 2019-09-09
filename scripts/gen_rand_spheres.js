let spheres = [];

for (let a = -11; a < 11; a++) {
  for (let b = -11; b < 11; b++) {
    let num = Math.random();
    let center = {
      x: a + 0.9 * Math.random(),
      y: 0.2,
      z: b + 0.9 * Math.random(),
    };

    let center_2 = {
      x: center.x - 4,
      y: center.y - 0.2,
      z: center.z,
    };

    if (
      Math.sqrt(
        center_2.x * center_2.x * center_2.y * center_2.y +
          center_2.z * center_2.z,
      ) <= 0.9
    ) {
      continue;
    }

    let to_push = {
      name: 'Sphere',
      center,
      radius: 0.2,
    };

    if (num < 0.8) {
      to_push.material = {
        name: 'Lambertian',
        albedo: {
          x: Math.random() * Math.random(),
          y: Math.random() * Math.random(),
          z: Math.random() * Math.random(),
        },
      };
    } else if (num < 0.95) {
      to_push.material = {
        name: 'Metal',
        albedo: {
          x: 0.5 * (1 + Math.random()),
          y: 0.5 * (1 + Math.random()),
          z: 0.5 * (1 + Math.random()),
        },
        fuzz: 0.5 * Math.random(),
      };
    } else {
      to_push.material = {
        name: 'Dielectric',
        ref_idx: 1.52,
      };
    }

    spheres.push(to_push);
  }
}

console.log(JSON.stringify(spheres, null, 2));
