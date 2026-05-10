# 🎨 2D Drawing Library: A Rust Deep Dive

This project is a 2D rendering engine built in Rust. It generates high-quality raster images (`image.png`) by calculating and plotting individual pixels for various geometric shapes onto a coordinate grid. Rather than using high-level drawing commands, this engine builds every shape from scratch using integer-based mathematical algorithms.

---

## 🦀 Rust for Beginners: Language Overview

Since this project is built in Rust, it utilizes several core language features that ensure memory safety and high performance.

### 1. The Syntax: `struct` and `impl`

Rust uses **Structs** to define data and **Impl blocks** to define behavior.

* **Structs**: Think of a `struct` as a blueprint. For example, our `Point` struct holds two pieces of data: an $x$ coordinate and a $y$ coordinate.
* **Impl**: The `impl` block is where we write functions associated with that blueprint, such as `Point::new()`.

### 2. Traits: The Power of Shared Behavior

A **Trait** (like our `Drawable`) is a "contract." It tells the compiler: "I don't care what shape this is (Circle, Line, or Cube); as long as it implements `Drawable`, I know I can call the `.draw()` method on it." This allows our `main.rs` to loop through different shapes and treat them all the same way.

### 3. Ownership and References (`&`)

You will notice many functions take arguments like `&Point`. The `&` symbol means we are "borrowing" the data rather than taking full ownership of it. This makes the code faster because we aren't constantly making expensive copies of data in memory.

---

## 🛠️ Detailed Codeblock Analysis

### 1. `main.rs`: The Orchestrator

The `main.rs` file serves as the entry point for the application.

* **Image Initialization**: It starts by creating a `blank` canvas of 1000x1000 pixels.
* **Trait Implementation**: It implements `Displayable` for the `Image` struct. This is a crucial bridge that allows our custom `geometrical_shapes` module to "talk" to the external `raster` library to set individual pixels.
* **Drawing Sequence**: It executes a series of commands to draw points, lines, and exactly 49 random circles.

### 2. `geometrical_shapes.rs`: The Mathematical Core

This file contains the heavy lifting.

* **`stroke_line`**: This is the engine's most important function. It uses **Bresenham's Line Algorithm** to decide exactly which pixels to turn on between two points to create a straight line.
* **Rectangle Normalization**: A key feature in our `Rectangle::new()` function is normalization. It uses `min()` and `max()` to ensure that even if you provide the "bottom-right" corner first, the engine correctly identifies the `top_left` so the drawing logic doesn't break.
* **Midpoint Circle Algorithm**: Used for the `Circle` struct, this algorithm plots points along a curve using only integer addition and subtraction, which is much faster for a computer than using trigonometry.

---

## 💎 Bonus: Advanced Geometry

We pushed the boundaries of 2D drawing with two bonus implementations:

* **The Pentagon**: This uses trigonometry ($sin$ and $cos$) to calculate five vertices equally spaced around a center point.
* **The Cube**: This uses a wireframe isometric projection. It maps 3D "model coordinates" into 2D "screen coordinates" to create the illusion of depth on a flat image.

---

## ✅ Quality Assurance (Testing Suite)

To ensure our logic remains sound as we add features, we utilize Rust's built-in testing framework.

* **`check_random.rs`**: Ensures that random generation never produces a point outside the image boundaries (e.g., a pixel at $x=1001$ on a 1000-wide image).
* **`check_reverse_order_points.rs`**: Verifies that our Rectangle normalization correctly swaps points to maintain a valid `top_left` corner.
* **`check_edge_cases.rs`**: Confirms that "zero-length lines" (where start and end are the same) are drawn as a single point without crashing the program.

**To execute tests:**

```bash
cargo test

```

---

## 🌟 Closing Note

Building this engine has been a journey through the fundamentals of computer graphics and the rigors of Rust's safety systems. What began as a simple pixel-plotting exercise evolved into a robust library capable of rendering complex 3D wireframes and mathematically perfect curves. We hope this code serves as a solid foundation for anyone looking to explore the beautiful intersection of math and code!

---

## 👥 The Development Team

| 🧑‍💻 Name | 💬 Discord Profile |
| --- | --- |
| **Georgia Marouli** | [@gmarouli](https://discordapp.com/users/1277216244910522371) |
| **Andriana Stas** | [@astas](https://www.google.com/search?q=https://discordapp.com/users/780150798927134740) |
| **Stamatios Manousis** | [@smanousi](https://discordapp.com/users/350760622180270090) |
| **Christos Markos** | [@cmarkos](https://www.google.com/search?q=https://discordapp.com/users/284929126278627328) |

