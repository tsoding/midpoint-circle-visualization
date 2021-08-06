# Another attempt to implement the algorithm in 2021

No dependencies. Single self-contained rust file. Visualization is done via output [PPM images](https://en.wikipedia.org/wiki/Netpbm)

## Quick Start

```console
$ rustc main.rs
$ ./main
```

The program will generate a bunch of PPM files with different patterns including solid and hollow circles. You will need an image viewer that supports the PPM format. I used [feh](https://feh.finalrewind.org/) and it worked well enough. Also [ImageMagick](https://imagemagick.org/index.php) can convert PPM files.

## Screenshots

![stripes](screenshots/stripes.png) ![checker](screenshots/checker.png)
