# 🌤️ Rust Skybox Converter

A high-performance CLI tool written in Rust to convert Equirectangular HDRI images into Cubemaps. It supports high-precision floating-point processing, multithreaded rendering, and automatic tone mapping for LDR outputs.

## ✨ Features

- **⚡ High Performance:** Uses `rayon` for parallel processing across all CPU cores.
- **📐 High Precision:** Internal processing uses 32-bit floating point (`f32`) to preserve HDR data.
- **🌈 Formats:**
  - **Input:** `.hdr` (Radiance), `.exr` (OpenEXR).
  - **Output:** `.png` (Tone Mapped LDR), `.exr` (Linear HDR).
- **📦 Layouts:**
  - `cross`: Standard unfolded cube cross.
  - `strip-h`: Horizontal strip ($6 \times 1$).
  - `strip-v`: Vertical strip ($1 \times 6$).
- **🔍 Quality:** Uses Bilinear Interpolation for smooth sampling.

---

## 🚀 Installation & Compilation

You need the Rust toolchain installed. If you don't have it, get it at [rustup.rs](https://rustup.rs).

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/SiputBiru/skybox_converter_rs
    cd skybox-converter_rs
    ```

2.  **Build for Release:**

    > **Note:** Always build in release mode! Debug builds are significantly larger and slower.
    -

    ```bash
    cargo build --release
    ```

    - Specific Build:

    ```bash
    cargo build --target x86_64-pc-windows-gnu --release
    ```

The executable will be located at `./target/release/skybox_converter`.

---

## 📖 Usage

You can run the tool directly via cargo or use the compiled binary.
if using the compiled binary use **./skybox_converter** or **cargo run --**

### Basic Conversion

Convert an HDR image to a standard PNG cubemap (Cross layout, 512px faces).

```bash
./skybox_converter -i input.hdr -o output.png
```

### Change Output Layout

Generate a Horizontal Strip (6 x 1) instead of cross.

```bash
./skybox_converter -i input.exr -o sky_strip.png --layout strip-h
```

Generate a Vertical Strip (1 x 6).

```bash
./skybox_converter -i input.exr -o strip_v.png --layout strip-v
```

### High-Res HDR Output

Keep the data in floating point (Linear HDR) and increase face resolution to 2048px.

```bash
./skybox_converter -i input.hdr -o sky_hq.exr --format exr --size 2048
```

### Tonemap Types

Change the tonemap output to your liking (ACES, Khronos PBR Neutral, Reinhard, AgX, Linear).

```bash
./skybox_converter -i input.hdr -o sky_hq.exr --format exr --t aces
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
│   └── strip.rs    # Strip (H/V) implementation
└── codecs/         # File Format encoders
    ├── mod.rs      # Encoder Factory
    ├── png.rs      # LDR Tone Mapping & PNG saving
    └── exr.rs      # HDR EXR saving
```

## 🗺️ Roadmap

- [x] Basic Equirectangular projection
- [x] Multithreaded processing
- [x] Bilinear Filtering
- [x] PNG (LDR) & EXR (HDR) support
- [ ] ASTC Compression (Mobile)
- [ ] BC6H Compression (DirectX/High-End)
- [ ] DDS Container support
- [ ] Ktx2 Container support
