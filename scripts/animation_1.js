const fs = require('fs');

const scene = require('../scenes/5.json');

let vh = 0.1;
let vy = 0.0;

const CAMERA_PAN_SPEED = 0.01;
const VY_COLLISION_MULTIPLIER = -0.87;
const VH_COLLISION_MULTIPLIER = 0.8;

const GRAVITY = -0.01881;

const ANIMATION_SECONDS = 3;
const FPS = 24;
const RESOLUTION = FPS * 50;

let t = 0;
let frames_rendered = 0;
while (t < RESOLUTION * FPS * ANIMATION_SECONDS) {
  scene.objects[0].center = JSON.parse(
    JSON.stringify(scene.objects[0].center2),
  );

  scene.objects[0].center2.x += vh / RESOLUTION;
  scene.objects[0].center2.y += vy / RESOLUTION;

  scene.camera.look_at.x += CAMERA_PAN_SPEED / RESOLUTION;

  vy += GRAVITY / RESOLUTION;

  if (scene.objects[0].center2.y <= 0) {
    scene.objects[0].center2.y = 0;
    vy *= VY_COLLISION_MULTIPLIER;
    vh *= VH_COLLISION_MULTIPLIER;
  }

  if (t % RESOLUTION === 0) {
    const bak = JSON.parse(JSON.stringify(scene.objects[0].center2));

    scene.objects[0].center2.x += vh;
    scene.objects[0].center2.y += vy;

    fs.writeFileSync(
      `./scenes/animation/${frames_rendered.toString().padStart(3, '0')}.json`,
      JSON.stringify(scene, null, 2),
    );

    frames_rendered += 1;
    scene.objects[0].center2 = bak;
  }

  t += 1;
}
