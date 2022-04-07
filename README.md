# Rust Ray Tracing

The goal of this repo is to build a play ray tracer using rust.

Heavily inspired by [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

### Why?

- I've never used Rust before
- I've never built a ray tracer from scratch
- Hopefully can be used as a playground to investigate other fun things like WASM, WebGL, etc

### Generate first PPM image file

```bash
# Generate ppm
cargo run > image.ppm

# convert to png (need ffmpeg installed)
# so that a browser will display it (like in this README)
ffmpeg -i image.ppm image.png
```

### Output

![First Ouput Image](image.png)