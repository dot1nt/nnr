# nnr
Simple tool for removing noise from automatically recorded NOAA images 

## Usage
```
Usage:
Options:
-c         |    Crop noise (top/bottom of image)
-t <val>   |    Noise threshold; 0.0 to 1.0; Default: 0.5
-f         |    Filter noise with median filter
```

## Building
```
git clone https://github.com/dot1nt/nnr.git && cd nnr
cargo build --release
sudo cp target/release/nnr /usr/local/bin/
```
