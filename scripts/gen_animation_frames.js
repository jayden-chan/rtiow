const fs = require('fs');

const scene = require('../scenes/5.json');

let vh = 0.1;
let vy = 0.0;

const GRAVITY = -0.01881;

const ANIMATION_SECONDS = 3;
const FPS = 25;

let t = 0;
while (t < FPS * ANIMATION_SECONDS) {
  scene.objects[0].center = JSON.parse(
    JSON.stringify(scene.objects[0].center2),
  );
  scene.objects[0].center2.x += vh;
  scene.objects[0].center2.y += vy;

  scene.camera.look_at.x += 0.009;

  vy += GRAVITY;

  if (scene.objects[0].center2.y <= 0) {
    scene.objects[0].center2.y = 0;
    vy *= -0.68;
    vh *= 0.75;
  }

  fs.writeFileSync(
    `./scenes/animation/${t.toString().padStart(3, '0')}.json`,
    JSON.stringify(scene, null, 2),
  );

  t += 1;
}
