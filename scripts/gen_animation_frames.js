const fs = require('fs');

const scene = require('../scenes/4.json');

const curr_pos = scene.objects[0].center;
const next_pos = scene.objects[0].center2;
const vh = 0.01;

const ANIMATION_SECONDS = 1;
const FPS = 15;

let t = 0;
while (t < FPS * ANIMATION_SECONDS) {
  curr_pos.x += vh;
  next_pos.x += vh;

  scene.objects[0].center = curr_pos;

  fs.writeFileSync(
    `./scenes/animation_1/${t.toString().padStart(3, '0')}.json`,
    JSON.stringify(scene, null, 2),
  );

  t += 1;
}
