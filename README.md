# raytracer

A parallel raytracer written in Rust. Based on Peter Shirley's ["Ray Tracing in One
Weekend"](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf)

## Progress Pictures

### Dielectrics: Hollow Glass Sphere - 2019-07-01
- Scene: `scenes/spheres_2.json`
- 1920 x 1080
- 5000 samples per pixel
- Machine: GCP n1-highcpu-96
- Render time: 00:01:20

![](/img/spheres_2.png "Hollow Dielectric Sphere")

### Lambertian & Metal spheres - 2019-07-01
- Scene: `scenes/spheres.json`
- 1920 x 1080
- 4000 samples per pixel
- Machine: GCP n1-highcpu-24
- Render time: 00:03:56

![](/img/spheres.png "Lambertian & Metal spheres")
