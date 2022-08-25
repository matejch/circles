# Collisions

How to handle collisions?

The general algorightm is this:

1. get pairs of objects that can collide. This is sometimes called Broad Collision Detection.
2. pairwise detect collision. 
3. resolve collisions


100 balls => 10k detections per frame => 600k detections per second
1000 balls => 1M detections per frame => 60M detections per second

1. naive

O(n^2)

2. Space partitioning

2.1 QuadTree

O(n log(n))

2.2 QuadTree in Rust

self-referencing data structures in Rust.

Three options
a. Box<Option<<QuadTreeNode >> (safe, slower, compiler helps, cumbersome code unwrap fest, safe)
b. replace pointers with array indexes (safe, fast, hard to reason about = error prone)
c. unsafe (unsafe, fast, easy to reason about)

I wanted to write idiomatic Rust so I chose the first approach. It was as expected, cumbersome, but I learned a few
tricks.


usually quadtree 

quadtree needs to be updated every frame



3. detection

Cost of detecting whether two objects collide depends on object's shapes.
To simplify calculations you can detect collisions between objects bounding boxes.
  


4. resolution

Depending on gameplay constrains and realism goals, collision resolution is 

In case of SpaceBalls, when balls collide they pass through each other.
When a ball collides with a wall it bounces of the wall without loss of energy.

If a ball collides with an "active ball" it becomes active itself, stops, and begins expanding.



5. improvements

circle-to-circle detection

k-d space partitioning


6. resources: 

recursive structures in rust
yt collisions
book

