#!/usr/bin/env python3

import numpy as np

EPSILON = 1e-4


# https://registry.khronos.org/OpenGL-Refpages/gl4/html/step.xhtml
def _step(edge, x):
    return 0.0 if x < edge else 1.0


step = np.vectorize(_step)


def intersect_aabb(ray_origin, ray_direction, aabb_min, aabb_max):
    ray_delta = 1.0 / ray_direction
    print("ray_delta", ray_delta)
    time_to_min = ray_delta * (aabb_min - ray_origin)
    print("time_to_min", time_to_min)
    time_to_max = ray_delta * (aabb_max - ray_origin)
    print("time_to_max", time_to_max)
    fastest = np.minimum(time_to_min, time_to_max)
    print("fastest", fastest)
    slowest = np.maximum(time_to_min, time_to_max)
    print("slowest", slowest)
    near = np.max(fastest)
    print("near", near)
    far = np.min(slowest)
    print("far", far)
    if far > 0.0 and near < far:
        distance = near - EPSILON
        print("distance", distance)
        normal = -np.sign(ray_direction) * step(near, fastest)
        print("normal", normal)
    else:
        print("no intersection")


def main():
    samples = (
        {
            "ray_origin": np.array((0.0, 0.0, 0.0)),
            "ray_direction": np.array((0.1, 0.2, 0.3)),
            "aabb_min": np.array((1.0, 1.0, 1.0)),
            "aabb_max": np.array((2.0, 3.0, 4.0)),
        },
        {
            "ray_origin": np.array((0.0, 0.0, 0.0)),
            "ray_direction": np.array((-0.1, 0.2, 0.3)),
            "aabb_min": np.array((1.0, 1.0, 1.0)),
            "aabb_max": np.array((2.0, 3.0, 4.0)),
        },
        {
            "ray_origin": np.array((0.0, 0.0, 0.0)),
            "ray_direction": np.array((0.1, 0.2, 0.3)),
            "aabb_min": np.array((4.0, 4.0, 4.0)),
            "aabb_max": np.array((5.0, 5.0, 5.0)),
        },
    )
    for sample in samples:
        ray_origin = sample["ray_origin"]
        print("ray_origin", ray_origin)
        ray_direction = sample["ray_direction"]
        print("ray_direction", ray_direction)
        ray_direction /= np.linalg.norm(ray_direction)
        print("ray_direction", ray_direction)
        aabb_min = sample["aabb_min"]
        print("aabb_min", aabb_min)
        aabb_max = sample["aabb_max"]
        print("aabb_max", aabb_max)
        intersect_aabb(ray_origin, ray_direction, aabb_min, aabb_max)
        print()


if __name__ == "__main__":
    main()
