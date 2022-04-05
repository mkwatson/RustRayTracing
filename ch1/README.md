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
