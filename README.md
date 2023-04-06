# RUST GL

A really simple CPU graphical library in Rust!

### Inspired by
[Tsoding's Olive.c](https://github.com/tsoding/olive.c)

### Work in Progress

You might not believe it, but this is still under construction! Just a disclaimer.

### Usage

You can edit the main function in [main.rs](./src/main.rs) with your styles, or use some of the example functions in there. Then simply:

```sh
$ cargo run
```

To transform the PPM images to PNGs, go to the outputs folder and run:
```sh
$ grep ppm | sed 's/.ppm//' | xargs -Ixx convert xx.ppm xx.png
```

## Gallery

![logo](./outputs/logo.png)

![checker](./outputs/checker_example.png)

![japan](./outputs/japan_flag_example.png)

![lines](./outputs/lines_example.png)

![triangles](./outputs/triangles_example.png)

