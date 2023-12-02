# Overview
This project is an example of using rust to generate simple image patens and noise source images.

## Implementation
Rendering is achieved by implementing a fragment shader on the CPU in parallel across multiple discreet chucks to reduce render time.

Points are scattered semi randomly around the center of the image and along the edges to create a seamless output texture.

This data is then distributed to a thread pool and the resulting chucks are recombined to create the output image.

![Output frame example](/frame.jpg)