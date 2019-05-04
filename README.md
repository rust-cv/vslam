# vslam

An attempt to create a full abstraction of visual simultaneous localization and mapping in Rust

## Unstable

If you are reading this, this is still an unstable API. Expect things to break regularly as we find that our abstractions aren't sufficient.

## Architecture

### Minimize Allocations

The most important thing that should be noted is that we should never require non-essential allocations on a per-frame basis. This does not mean that we can perform all operations on `#![no_std]`, but rather that we can, if the user desires, reuse memory from previous photogrammetric operations where possible. For instance, if the user would like to always extract `5000` features from images, then they should be able to statically statically allocate a `5000` feature array and populate it with the features. Alternatively, a user could also create a vector and populate it with features, but they should be able to clear the vector and repopulate it without having to reallocate. This will help support a `#![no_std]` feature flag in the future, while also removing any overhead from system allocation.

Initially, not every library used will support this paradigm. Many will perform additional allocations, such as every time an image is loaded into memory, when features are extracted, etc. These abstractions should make it possible to do downstream and the libraries should strive to reduce allocations where possible by reusing buffers as necessary and allowing the user to supply the buffers.

If it comes down to a trade between allocation and latency, throughput, or quality, we should prefer allocating, but avoid it where possible. For instance, if we realize that iterators are prohibitive to SIMD since we need all the memory to be linear, we should create a new abstraction to send the SIMD primitives themselves over the iterator so we can achieve speed and zero-alocations simultaneously.

### Prioritization and trade-offs

We can choose to prioritize these things in any order:

- Lowest latency from frame-in to pose-estimation and mapping output
  - We want to know the position of the sensor and what is sensed as quickly as possible.
- Highest frame/sec throughput to pose-estimation and mapping output
  - We want to process as many video feeds at the same time as possible.
  - We want to process as high resolution of a video feed in near-realtime as possible.
- Fastest/most accurate localization/loop-closure
  - We want to know the global position of something accurately as soon as possible.

If tradeoffs are necessary for any one of these that change the way a user interacts with the APIs then we should support abstractions for each of these paradigms.

## Avoid

Please avoid using any novel/patentable processes in "ORB-SLAM2: an Open-Source SLAM System for Monocular, Stereo and RGB-D Cameras". Also avoid using the codebase of ORB-SLAM2 as a reference, as it is GPL 3 Affero licensed. Please develop any vSLAM algorithms using other alternatives. I intend for this crate to be usable by industry without any royalty payments to anyone, thus any code found to introduce patented or paid and copyrighted material will be removed from the code base.

## Credits

[TheiaSfM](https://github.com/sweeneychris/TheiaSfM) and all of its authors can be thanked as their abstractions are direct inspiration for this crate. In some cases, the names of some abstractions may be borrowed directly if they are appropriate for SLAM usage. You can find the TheiaSfM documentation [here](http://www.theia-sfm.org/api.html).

"[Past, Present, and Future of Simultaneous Localization And Mapping: Towards the Robust-Perception Age](https://arxiv.org/pdf/1606.05830.pdf)" is an excellent paper that compiles information about modern SLAM algorithms and papers.
