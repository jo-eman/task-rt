# Task "rt"
grit:lab Åland Islands 2023

## Build
```
./do
```

## Run
```
./we [camera file] [light file] [objects file]
```
One camera file, one light file and one objects file are required.  
The order of the files is important.  

## Example
```
./we use/camera1 use/light1 use/objects1
```

## Camera file example . The comments after `#` are ignored
```
# result image width in pixels, height in pixels, output file name without extension
800 600 output_file
view 90
from 0 0 0
to 0 0 1
up 0 1 0
```

## Light file example . The comments after `#` are ignored
```
# a point light custom implementation of brightness control
power 255
color 255 255 255
from 0 0 0
```

## Objects file example . The comments after `#` are ignored
```
# sphere: color coordinates radius
# cube: color coordinates size
# roll: color coordinates radius height
# flat: color coordinates normal
255 255 0 sphere 0 0 5 2
0 255 255 cube 0 0 0 2
255 0 255 roll 0 0 0 2 2
0 255 0 flat 0 0 0 0 0 1
```

## Description
https://github.com/01-edu/public/tree/master/subjects/rt

## Authors
- [healingdrawing](https://healingdrawing.github.io)