const fs = require('fs');

const scene = require('../scenes/5.json');

let vh = 0.1;
let vy = 0.0;

const CAMERA_PAN_SPEED = 0.01;
const VY_COLLISION_MULTIPLIER = -0.68;
const VH_COLLISION_MULTIPLIER = 0.75;

const GRAVITY = -0.01881;

const ANIMATION_SECONDS = 3;
const RESOLUTION = 50;
const FPS = 25;

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

  if (t % 50 === 0) {
    fs.writeFileSync(
      `./scenes/animation/${frames_rendered.toString().padStart(3, '0')}.json`,
      JSON.stringify(scene, null, 2),
    );

    frames_rendered += 1;
  }

  t += 1;
}
