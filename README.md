# Rust Raytracer

A high-performance CPU-based raytracer implementation in Rust, featuring parallel rendering and optimized vector operations.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

This raytracer is a from-scratch implementation that leverages modern CPU capabilities to generate photorealistic images through ray tracing techniques. Built with performance in mind, it demonstrates the power of Rust's safety guarantees combined with low-level optimization strategies.

## Sample Scene 
![Example Scene](/assets/scene.png)

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
git clone https://github.com/Turtel216/raytracer-rs.git
cd raytracer-rs

# Build in release mode (recommended for performance)
cargo build --release

# Run the raytracer
cargo run --release > image.ppm
```

## Performance Benchmarks
Benchmark results on an AMD Ryzen 5 3600 (6 cores / 12 threads) using [hyperfine](https://github.com/sharkdp/hyperfine)

| Method | Render Time |
|------------|---------|---------|
| Singly Threaded  | TODO       |
| Singly Threaded + SIMD | TODO |
| Multi-threaded | TODO |
| Multi-threaded + SIMD | TODO |

*Performance may vary based on scene complexity and hardware*

### Optimization Techniques

- **Rayon**: Parallelizes pixel rendering across all CPU cores
- **SIMD**: Vectorizes mathematical operations for 4× throughput improvement

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
