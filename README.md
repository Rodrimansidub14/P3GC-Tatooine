# Tatooine Raytracer - Normal Mapping Implementation


<div align="center">
  <a href="https://github.com/user-attachments/assets/4016379c-6686-49e8-a700-0ffdcde18405">Click here to view</a>
</div>


## Overview

This project is a ray-traced rendering of a desert-like scene inspired by Tatooine from Star Wars, featuring two suns, sand dunes, and cubic structures. The scene incorporates **normal mapping** for more realistic surface details, such as simulating the appearance of sand dunes on a finite plane.

The raytracer uses Rust and external libraries such as `nalgebra` for vector and matrix operations, and `image` for handling textures. The project focuses on realistic lighting, shading, and surface details by using both diffuse textures and normal maps.

## Features

- **Normal Mapping**: Adds extra detail to flat surfaces by using normal maps to simulate small bumps and irregularities, such as dunes in the desert.
- **Finite Plane**: The ground is modeled as a finite plane of 10x10 units with texture and normal mapping applied.
- **Two Suns**: Simulates the Tatooine environment with two light sources casting realistic shadows and highlights.
- **Day Light Cycle**: You can change the time to resemble a tatooine sunset as lighting and object properties change.
- **Raytracing**: The entire scene is ray-traced, meaning all objects interact with light via reflection, refraction, shadows, and more.
- **Customizable Materials**: Each object in the scene has customizable material properties (color, albedo, specular, emissive, etc.) to simulate different materials like sand, metal, and clay.

## Controls

You can interact with the camera and environment using the following controls:

### Camera Controls
- **Mouse Scroll**: Zoom in/out by adjusting the camera's radius.
- **Left Mouse Button (Drag)**: Orbit the camera around the scene by dragging while holding the left mouse button.
- **W**: Move the camera up along the global Y-axis.
- **S**: Move the camera down along the global Y-axis.
- **A**: Move the camera left along the global X-axis.
- **D**: Move the camera right along the global X-axis.

### Time of Day Toggle
- **1**: Toggle between day and night modes for the skybox and lighting.

### Escape
- **Esc**: Exit the application.

The camera remains fixed at a certain point in space, but the controls allow you to move the camera around the scene for different viewpoints. The zoom and orbit features allow for smooth camera manipulation using the mouse, while WASD keys provide global X/Y axis movement.

### Prerequisites

Ensure you have the following installed:

- **Rust**: The project is written in Rust, so you'll need the latest version of Rust and Cargo.
- **Image Assets**: Download the necessary textures (albedo and normal maps) and place them in the appropriate folder.

### Project Structure

