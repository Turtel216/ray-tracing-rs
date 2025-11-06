# Rust Raytracer

A high-performance CPU-based raytracer implementation in Rust, featuring parallel rendering and optimized vector operations.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

This raytracer is a from-scratch implementation that leverages modern CPU capabilities to generate photorealistic images through ray tracing techniques. Built with performance in mind, it demonstrates the power of Rust's safety guarantees combined with low-level optimization strategies.

## Key Features

### **Parallel Rendering with Rayon**
- Multi-threaded ray tracing utilizing all available CPU cores
- Automatic work distribution for optimal load balancing
- Near-linear performance scaling with core count

### **SIMD Vector Operations**
- Hardware-accelerated vector mathematics using SIMD instructions
- Optimized dot products, cross products, and vector arithmetic
- Significant performance improvements for ray-geometry intersection tests

### **Positionable Camera System**
- Flexible camera positioning and orientation in 3D space
- Configurable field of view and aspect ratio
- Look-at functionality for intuitive scene composition

### **Core Raytracing Features**
- Sphere and plane primitive support
- Diffuse, reflective, and refractive materials
- Soft shadows and ambient lighting
- Configurable anti-aliasing through supersampling

## Installation

### Prerequisites

- Rust 1.93.0-nightly or higher
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Turtel216/raytracer.git
cd raytracer

# Build in release mode (recommended for performance)
cargo build --release

# Run the raytracer
cargo run --release
```
<!--
## Usage

### Basic Example

```rust
use raytracer::{Camera, Scene, Sphere, Vector3, Material};

fn main() {
    // Create a scene
    let mut scene = Scene::new();
    
    // Add objects
    scene.add(Sphere::new(
        Vector3::new(0.0, 0.0, -5.0),
        1.0,
        Material::diffuse(Vector3::new(0.8, 0.3, 0.3))
    ));
    
    // Configure camera
    let camera = Camera::new()
        .position(Vector3::new(0.0, 0.0, 0.0))
        .look_at(Vector3::new(0.0, 0.0, -1.0))
        .field_of_view(60.0)
        .aspect_ratio(16.0 / 9.0);
    
    // Render the scene
    let image = camera.render(&scene, 1920, 1080);
    image.save("output.png").unwrap();
}
```

### Configuration

Adjust rendering parameters in your code or configuration file:

```rust
let config = RenderConfig {
    width: 1920,
    height: 1080,
    samples_per_pixel: 64,
    max_ray_depth: 8,
    num_threads: None, // Auto-detect CPU cores
};
```
-->

## Performance: TODO
<!--
Benchmark results on an AMD Ryzen 9 5900X (12 cores / 24 threads):

| Resolution | Samples | Render Time |
|------------|---------|-------------|
| 1920×1080  | 16 SPP  | ~8.2s       |
| 1920×1080  | 64 SPP  | ~31.5s      |
| 3840×2160  | 16 SPP  | ~33.1s      |

*Performance may vary based on scene complexity and hardware*

### Optimization Techniques

- **Rayon**: Parallelizes pixel rendering across all CPU cores
- **SIMD**: Vectorizes mathematical operations for 4× throughput improvement
- **Rust Zero-Cost Abstractions**: Clean code without runtime overhead
-->

<!--
## Architecture

```
src/
├── main.rs           # Entry point
├── camera.rs         # Camera system with positioning
├── ray.rs            # Ray definition and methods
├── vector.rs         # SIMD-optimized vector operations
├── scene.rs          # Scene management
├── geometry/         # Geometric primitives
│   ├── sphere.rs
│   └── plane.rs
├── material.rs       # Material definitions
└── renderer.rs       # Parallel rendering engine
```
-->

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Ray Tracing in One Weekend](https://raytracing.github.io/) by Peter Shirley
- [Rayon](https://github.com/rayon-rs/rayon) for effortless parallelism
- The Rust community for excellent documentation and support
