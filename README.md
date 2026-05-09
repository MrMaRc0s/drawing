# 2D Drawing Library - Project Summary

This project is a 2D rendering engine built in Rust that generates raster images by plotting geometric shapes onto a pixel grid. It leverages the `raster` crate for image manipulation and uses custom mathematical algorithms to draw shapes pixel-by-pixel.

## How the Files Work

### 1. `main.rs`
This is the entry point of the application. It performs the following:
- **Initialization**: Creates a blank 1000x1000 canvas. [cite: main.rs]
- **Coordination**: Calls the drawing methods for various shapes (Points, Lines, Rectangles, Triangles, and Circles). [cite: main.rs]
- **Display Implementation**: Implements the `Displayable` trait for the `Image` struct, allowing the shapes to communicate directly with the pixel grid. [cite: main.rs]

### 2. `geometrical_shapes.rs`
This module contains the "brains" of the project. It is organized into three layers:
- **The Traits**: `Drawable` and `Displayable` act as contracts. Any shape we build must follow these rules so `main.rs` can draw them consistently. [cite: geometrical_shapes.rs]
- **The Core Logic**: This file contains the implementation of **Bresenham's Line Algorithm** (for straight lines) and the **Midpoint Circle Algorithm** (for curves). These algorithms calculate exactly which pixels to color based on integer math. [cite: geometrical_shapes.rs]
- **The Shapes**: 
    - `Point`: The base coordinate system. [cite: geometrical_shapes.rs]
    - `Line`: Connects two points using Bresenham's math. [cite: geometrical_shapes.rs]
    - `Rectangle`: Automatically normalizes coordinates so that it draws correctly even if the user provides the "bottom-right" point before the "top-left." [cite: geometrical_shapes.rs]
    - `Triangle`: A composite shape that uses three lines to close a polygon. [cite: geometrical_shapes.rs]
    - `Circle`: Uses the Midpoint algorithm to stamp symmetrical pixels around a center. [cite: geometrical_shapes.rs]

## Previous Work (Classmate Implementation)
A previous classmate provided the initial structural definitions for the project [cite: geometrical_shapes_og.rs]:
- Defined the basic `Drawable` and `Displayable` trait signatures. [cite: geometrical_shapes_og.rs]
- Outlined the original `Point`, `Line`, and `Circle` structs. [cite: geometrical_shapes_og.rs]
- Provided the initial random coordinate generation logic for the `Point` and `Line`. [cite: geometrical_shapes_og.rs]