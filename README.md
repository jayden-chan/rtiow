# raytracer

A parallel ray tracer written in Rust. Based on Peter Shirley's ["Ray Tracing in One
Weekend"](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf)

## Progress Pictures

Rendering settings:
- Resolution: 1920 x 1080
- 5000 samples per pixel
- Machine: GCP n1-highcpu-96

### Dielectrics: Hollow Glass Sphere - 2019-07-01
- Scene: [`scenes/spheres_2.json`](/scenes/spheres_2.json)
- Render time: 00:01:19

![](/img/spheres_2.png "Hollow Dielectric Sphere")

### Lambertian & Metal spheres - 2019-07-01
- Scene: [`scenes/spheres.json`](/scenes/spheres.json)
- Render time: 00:01:22

![](/img/spheres.png "Lambertian & Metal spheres")
