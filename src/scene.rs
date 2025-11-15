use crate::{
    ViewMode,
    color::Color,
    geometry::intersect::Intersect,
    geometry::intersection::Intersection,
    geometry::ray::Ray,
    math::vec3::{Vec3f, vec3},
    object::Object,
    palette::Palette,
    sky::Sky,
    trace_payload::TracePayload,
    trace_stats::TraceStats,
};

pub(crate) struct Scene {
    objects: Vec<Object>,
    sky: Sky,
}

impl Scene {
    pub(crate) fn new() -> Self {
        Self {
            objects: vec![],
            sky: Sky::default(),
        }
    }

    pub(crate) fn spawn(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub(crate) fn intersect(&self, ray: Ray) -> Option<(Intersection, Color)> {
        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .intersect(ray)
                    .map(|intersection| (intersection, object.color))
            })
            .min_by(|(a, _), (b, _)| a.distance.total_cmp(&b.distance))
    }

    pub(crate) fn trace(&self, ray: Ray, view_mode: &ViewMode) -> TracePayload {
        const REFLECTION_DEPTH: usize = 2;
        const REFLECTION_COUNT: usize = 2;
        const MAX_REFLECTION_COUNT: usize = const {
            let mut count = 1; // one is an initial ray
            let mut depth = 1;
            while depth <= REFLECTION_DEPTH {
                count += REFLECTION_COUNT.pow(depth as u32);
                depth += 1;
            }
            count
        };

        struct Incident {
            ray: Ray,
            depth: usize,
        }

        let mut incidents = vec![Incident { ray, depth: 0 }];
        let mut colors = vec![];
        let mut stats = TraceStats::default();
        while let Some(incident) = incidents.pop() {
            stats.traced += 1;
            let ray = incident.ray;
            let depth = incident.depth;
            let color = if let Some((intersection, color)) = self.intersect(ray) {
                stats.hit += 1;
                match view_mode {
                    ViewMode::Normal => {
                        let color = 0.5 * intersection.normal + 0.5;
                        colors.push(color);
                        break;
                    }
                    ViewMode::Depth => {
                        const FAR_DISTANCE: f32 = 4.0;
                        let ratio = intersection.distance / FAR_DISTANCE;
                        let color = vec3!(ratio);
                        colors.push(color);
                        break;
                    }
                    _ => (),
                }
                let hit_position = intersection.hit_position(ray);
                if incident.depth < REFLECTION_DEPTH {
                    for _ in 0..REFLECTION_COUNT {
                        let reflected_ray = Ray {
                            origin: hit_position,
                            direction: generate_diffuse_ray(
                                ray.direction,
                                intersection.normal,
                                0.1,
                            ),
                        };
                        incidents.push(Incident {
                            depth: depth + 1,
                            ray: reflected_ray,
                        });
                        stats.reflected += 1;
                    }
                }
                let shadow_ray = Ray {
                    origin: hit_position,
                    direction: -self.sky.sun_light_direction,
                };
                stats.shadow_traced += 1;
                // Let the sun to light with 1.0 intensity,
                // but leave some threshold for ambient light
                const AMBIENT_LIGHT_THRESHOLD: f32 = 0.2;
                if self.intersect(shadow_ray).is_some() {
                    stats.shadow_hit += 1;
                    AMBIENT_LIGHT_THRESHOLD * color.0
                } else {
                    let light_intensity = intersection
                        .normal
                        .dot(-self.sky.sun_light_direction)
                        .max(AMBIENT_LIGHT_THRESHOLD);
                    color.0 * light_intensity
                }
            } else {
                match view_mode {
                    ViewMode::Color => self.sky.get_color(ray.direction).0,
                    ViewMode::Normal => 0.5 * -ray.direction + 0.5,
                    ViewMode::Depth => vec3!(1.0),
                    ViewMode::Complexity => vec3!(0.0),
                }
            };
            let color_contribution = 0.1f32.powi(depth as i32);
            colors.push(color * color_contribution);
        }
        let color = if let ViewMode::Complexity = view_mode {
            let ratio = colors.len() as f32 / MAX_REFLECTION_COUNT as f32;
            Palette::TEMPERATURE.get_color(ratio)
        } else {
            Color(colors.into_iter().sum::<Vec3f>())
        };
        TracePayload { color, stats }
    }
}

fn generate_diffuse_ray(incident: Vec3f, normal: Vec3f, bias: f32) -> Vec3f {
    let bias_direction = loop {
        let random_unit = Vec3f::random_unit();
        if random_unit.dot(normal) > 0.0 {
            break random_unit.normalize();
        };
    };
    (incident.reflect(normal) + bias_direction * bias).normalize()
}
