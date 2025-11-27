# 🌤️ eq2c

[![CI](https://github.com/SiputBiru/eq2c-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/SiputBiru/eq2c-rs/actions)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/SiputBiru/eq2c-rs?style=flat-square)](https://github.com/SiputBiru/eq2c-rs/releases)

eq2c (Equi-to-Cube) is a high-performance CLI tool written in Rust to convert Equirectangular HDRI images into Cubemaps.
It supports high-precision floating-point processing, multithreaded rendering, and automatic tone mapping for LDR outputs.

## ✨ Features

- **🌈 Formats:**
  - **Input:** `.hdr` (Radiance), `.exr` (OpenEXR).
  - **Output:** `.png` (Tone Mapped LDR), `.exr` (Linear HDR).
- **📦 Layouts:**
  - `cross`: Standard unfolded cube cross.
  - `strip-h`: Horizontal strip ($6 \times 1$).
  - `strip-v`: Vertical strip ($1 \times 6$).
  - `Separate`: output 6 file for each faces.
- **🔍 Quality:** Uses Bilinear Interpolation for smooth sampling.
- **🎨 Tonemap:** Various tonemap options for the output file.
  - `Linear` or `None` Simple clamping system.
  - `ACES` Source: http://www.oscars.org/science-technology/sci-tech-projects/aces
  - `Khronos PBR Neutral` Source: https://github.com/KhronosGroup/ToneMapping/blob/main/PBR_Neutral/pbrNeutral.glsl
  - `Reinhard` Source: https://www-old.cs.utah.edu/docs/techreports/2002/pdf/UUCS-02-001.pdf
  - `AgX` Still in testing, need more research still.

---

## 🚀 Installation & Compilation

You need the Rust toolchain installed. If you don't have it, get it at [rustup.rs](https://rustup.rs).

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/SiputBiru/eq2c-rs
    cd eq2c-rs
    ```

2.  **Build for Release:**

    > **Note:** Always build in release mode! Debug builds are significantly larger and slower.
    - default command for cargo build with release profile

    ```bash
    cargo build --release
    ```

    - Specific Build:

    ```bash
    cargo build --target x86_64-pc-windows-gnu --release
    ```

The executable will be located at `./target/release/eq2c`

---

## 📖 Usage

You can run the tool directly via cargo or use the compiled binary.
if using the compiled binary use **eq2c** or **cargo run --**

### Basic Conversion

Convert an HDR image to a standard PNG cubemap (Cross layout, 512px faces).

```bash
eq2c -i input.hdr -o output.png
```

### Change Output Layout

- Generate a Horizontal Strip (6 x 1) instead of cross.

```bash
eq2c -i input.exr -o sky_strip.png --layout strip-h
```

- Generate a Vertical Strip (1 x 6).

```bash
eq2c -i input.exr -o strip_v.png --layout strip-v
```

- Generate Separate faces in 6 files.

```bash
eq2c -i input.exr -o my_sky.png --layout separate
```

output:

| Face   | File Name     | Meaning    |
| ------ | ------------- | ---------- |
| Right  | my_sky_px.png | Positive X |
| Left   | my_sky_nx.png | Negative X |
| Top    | my_sky_py.png | Positive Y |
| Bottom | my_sky_ny.png | Negative Y |
| Front  | my_sky_pz.png | Positive Z |
| Back   | my_sky_nz.png | Negative Z |

### High-Res HDR Output

Keep the data in floating point (Linear HDR) and increase face resolution to 2048px.

```bash
./eq2c -i input.hdr -o sky_hq.exr --format exr --size 2048
```

### Tonemap Types

Change the tonemap output to your liking (ACES, Khronos PBR Neutral, Reinhard, AgX, Linear).

```bash
./eq2c -i input.hdr -o sky_hq.exr --format exr --t aces
```

## 🏗️ Project Structure

```
src/
├── main.rs         # CLI Entry point & Argument parsing
├── lib.rs          # Library interface
├── math.rs         # Core 3D vector math & UV projection
├── layouts/        # Geometry logic
│   ├── mod.rs      # Layout Factory
│   ├── cross.rs    # Cross layout implementation
│   ├── separate.rs # Separate layout implementation
│   └── strip.rs    # Strip (H/V) implementation
└── codecs/         # File Format encoders
    ├── mod.rs      # Encoder Factory
    ├── png.rs      # LDR Tone Mapping & PNG saving
    ├── tonemap.rs  # tonemaps implementation
    └── exr.rs      # HDR EXR saving
```

## 🗺️ Roadmap

- [x] Basic Equirectangular projection
- [x] Multithreaded processing
- [x] Bilinear Filtering
- [x] PNG (LDR) & EXR (HDR) support
- [x] Adding Separate layout outputing 6 faces
- [ ] Better Agx implementation
- [ ] ASTC Compression (Mobile)
- [ ] DWAA/DWAB Compression
- [ ] BC6H Compression (DirectX/High-End)
- [ ] DDS Container support
- [ ] Ktx2 Container support
