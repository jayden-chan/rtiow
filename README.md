# Ray Tracing in One Weekend

A parallel ray tracer written in Rust. Based on Peter Shirley's ["Ray Tracing in One
Weekend"](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf)
book series. This implementation more or less covers all three books, with some
exceptions.

## Progress Pictures

Rendering settings:
- Resolution: 1920 x 1080
- 5000 samples per pixel
- Machine: GCP n1-highcpu-96 (96 vCPUs, 86.4 GB memory)

### Translations & Rotations - 2019-09-13
Implemented the Translate and Rotate wrapper objects that can translate and rotate
other objects (as the names suggest). The `Rotate` object uses `const generics` in a
similar way to the Rectangle object.
- Scene: [`scenes/9.json`](/scenes/9.json)
- Render time: 00:06:00

<p align="center">
<img width="650" height="650" src="/img/cornell_with_blocks.png"/>
</p>

### Remaining Rectangle Planes - 2019-09-12
Implemented the remaining rectangular plane objects using Rust's new `const generics`
feature. The new rectangle types are demonstrated using the famous Cornell Box.
- Scene: [`scenes/8.json`](/scenes/8.json)
- Render time: 00:01:00

<p align="center">
<img width="650" height="650" src="/img/cornell.png"/>
</p>

### Lights & Rectangles - 2019-09-09
Implemented a simple diffuse lighting material that can emit its own rays. Also
implemented an axis-aligned rectangle object type.
- Scene: [`scenes/7.json`](/scenes/7.json)
- Render time: 00:00:25

![](/img/lights_2.png "Lights and Mirrors")

- Scene: [`scenes/6.json`](/scenes/6.json)
- Render time: 00:00:17

![](/img/spheres_6.png "Diffuse Light Material")

### Bounding Volume Hierarchy - 2019-09-09
Implemented a basic bounding volume hierarchy to organize objects into groups and reduce
ray-object intersection computations. This results in a significant speedup in rendering
for scenes that have many objects.

![](/img/spheres_3_bvh.png "BVH")

### Motion Blur - 2019-07-11
Implemented camera motion blur. This feature is more interesting for animations as
opposed to still images.
- Scene: [`scenes/5.json`](/scenes/5.json)
- Physics script: [`scripts/animation_1.js`](/scripts/animation_1.js)
- Render time: 00:08:03

![](/img/animation_1.gif "Bouncing Ball Animation")

- Scene: [`scenes/4.json`](/scenes/4.json)
- Render time: 00:00:42

![](/img/motion_blur.png "Motion Blur")

### Camera Lens Effects - 2019-07-05
Implemented camera aperture width and focal length effects. This is the end of the "Ray
Tracing in One Weekend" book. The render below includes a bunch of randomly generated
spheres and showcases the depth of field effect given by the camera lens parameters.
- Scene: [`scenes/3.json`](/scenes/3.json)
- Render time: 00:26:38

![](/img/spheres_3.png "Camera Lens Effects")

### Positionable Camera - 2019-07-03
An alternate angle of the previous scene using a custom camera position and FOV
- Scene: [`scenes/2_alt.json`](/scenes/2_alt.json)
- Render time: 00:01:48

![](/img/spheres_2_alt.png "Positionable Camera")

### Dielectric Materials - 2019-07-01
A simple scene with two metal spheres and a hollow glass sphere, showcasing the
reflective / refractive properties of dielectric materials
- Scene: [`scenes/2.json`](/scenes/2.json)
- Render time: 00:01:19

![](/img/spheres_2.png "Hollow Dielectric Sphere")

### Lambertian & Metal Materials - 2019-07-01
A simple scene with some metal and matte spheres, showcasing the scattering and
reflection properties of Lambertian and Metal materials.
- Scene: [`scenes/1.json`](/scenes/1.json)
- Render time: 00:01:22

![](/img/spheres.png "Lambertian & Metal spheres")
