//! Slope-filtered walkable `NavMesh` extraction, bounded `A*` polygon search, and corridor steering.
//!
//! Walkable triangle slope filtering (`navTriangleWalkable`), admissible `f = g + h` priority
//! scoring (`astarPriorityMilli`), and search node caps (`navMaxSearchNodes`) execute through
//! the Lean-verified `crate::prism_kernel` emitted by Prism.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};
use verdant_tensor::WordBitset;

use crate::error::NavigationError;
use crate::geometry::ProceduralMesh;
use crate::math::{geometric_epsilon, three_u32, Vec3, DIM_PLANAR, DIM_SPATIAL};

/// Single walkable navigation polygon extracted from scene or terrain geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavPolygon {
    /// Contiguous polygon index in `NavMesh::polygons`.
    pub index: usize,
    /// World-space triangle vertices.
    pub vertices: [Vec3; DIM_SPATIAL],
    /// Geometric centroid `(v0 + v1 + v2) / 3`.
    pub centroid: Vec3,
    /// Upward normal `n_y` in permille (`0..=1000`).
    pub normal_y_milli: u64,
    /// Indices of edge-adjacent walkable polygons.
    pub neighbors: Vec<usize>,
}

/// Walkable polygon navigation mesh for autonomous agent pathfinding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavMesh {
    /// Walkable navigation polygons with shared-edge neighbor lists.
    pub polygons: Vec<NavPolygon>,
}

/// Planned multi-polygon navigation corridor returned by `NavMesh::find_path_astar`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavCorridorPath {
    /// Ordered polygon indices from `start` to `goal`.
    pub polygon_indices: Vec<usize>,
    /// Centroid/portal waypoints along the corridor in world meters.
    pub waypoints: Vec<Vec3>,
    /// Cumulative Euclidean travel cost in meters.
    pub total_cost_meters: f64,
}

impl NavMesh {
    /// Counts triangles satisfying `prism_kernel::navTriangleWalkable` without allocating full edge adjacency.
    #[must_use]
    pub fn count_walkable_triangles(mesh: &ProceduralMesh) -> usize {
        let scale = crate::prism_kernel::skinWeightScale().unwrap_or(1).max(1) as f64;
        let mut count = 0usize;
        for tri in mesh.indices.chunks_exact(DIM_SPATIAL) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[1 + 1] as usize;
            if i0 >= mesh.positions.len()
                || i1 >= mesh.positions.len()
                || i2 >= mesh.positions.len()
            {
                continue;
            }
            let p0 = mesh.positions[i0];
            let p1 = mesh.positions[i1];
            let p2 = mesh.positions[i2];
            let v0 = Vec3::new(f64::from(p0[0]), f64::from(p0[1]), f64::from(p0[1 + 1]));
            let v1 = Vec3::new(f64::from(p1[0]), f64::from(p1[1]), f64::from(p1[1 + 1]));
            let v2 = Vec3::new(f64::from(p2[0]), f64::from(p2[1]), f64::from(p2[1 + 1]));
            let cross = (v1 - v0).cross(v2 - v0);
            if cross.length() <= geometric_epsilon() {
                continue;
            }
            let normal = cross.normalized();
            if normal.y <= 0.0 {
                continue;
            }
            let normal_y_milli = (normal.y.min(1.0) * scale).round() as u64;
            if crate::prism_kernel::navTriangleWalkable(normal_y_milli).unwrap_or(false) {
                count += 1;
            }
        }
        count
    }

    /// Extracts walkable triangles via `prism_kernel::navTriangleWalkable` and builds edge adjacency.
    pub fn from_mesh(mesh: &ProceduralMesh) -> Result<Self, NavigationError> {
        let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;
        let inv_three = 1.0 / (three_u32() as f64);

        let mut polygons = Vec::new();
        for tri in mesh.indices.chunks_exact(DIM_SPATIAL) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[1 + 1] as usize;
            if i0 >= mesh.positions.len()
                || i1 >= mesh.positions.len()
                || i2 >= mesh.positions.len()
            {
                continue;
            }
            let p0 = mesh.positions[i0];
            let p1 = mesh.positions[i1];
            let p2 = mesh.positions[i2];
            let v0 = Vec3::new(f64::from(p0[0]), f64::from(p0[1]), f64::from(p0[1 + 1]));
            let v1 = Vec3::new(f64::from(p1[0]), f64::from(p1[1]), f64::from(p1[1 + 1]));
            let v2 = Vec3::new(f64::from(p2[0]), f64::from(p2[1]), f64::from(p2[1 + 1]));

            let cross = (v1 - v0).cross(v2 - v0);
            if cross.length() <= geometric_epsilon() {
                continue;
            }
            let normal = cross.normalized();
            if normal.y <= 0.0 {
                continue;
            }
            let normal_y_milli = (normal.y.min(1.0) * scale).round() as u64;
            let walkable = crate::prism_kernel::navTriangleWalkable(normal_y_milli).expect("prism");
            if !walkable {
                continue;
            }

            let idx = polygons.len();
            polygons.push(NavPolygon {
                index: idx,
                vertices: [v0, v1, v2],
                centroid: (v0 + v1 + v2).scale(inv_three),
                normal_y_milli,
                neighbors: Vec::new(),
            });
        }

        if polygons.is_empty() {
            return Err(NavigationError::EmptyWalkableNavMesh);
        }

        let qkey = |v: Vec3| {
            (
                (v.x * scale).round() as i64,
                (v.y * scale).round() as i64,
                (v.z * scale).round() as i64,
            )
        };
        let keys: Vec<[(i64, i64, i64); DIM_SPATIAL]> = polygons
            .iter()
            .map(|poly| {
                [
                    qkey(poly.vertices[0]),
                    qkey(poly.vertices[1]),
                    qkey(poly.vertices[1 + 1]),
                ]
            })
            .collect();

        let count = polygons.len();
        type QuantKey = (i64, i64, i64);
        let mut edge_owners: HashMap<(QuantKey, QuantKey), Vec<usize>> =
            HashMap::with_capacity(count * DIM_SPATIAL);
        let two_idx = 1usize + 1usize;
        for (poly_idx, tri_keys) in keys.iter().enumerate() {
            let pairs = [
                (tri_keys[0], tri_keys[1]),
                (tri_keys[1], tri_keys[two_idx]),
                (tri_keys[two_idx], tri_keys[0]),
            ];
            for (k0, k1) in pairs {
                if k0 == k1 {
                    continue;
                }
                let canonical = if k0 <= k1 { (k0, k1) } else { (k1, k0) };
                edge_owners.entry(canonical).or_default().push(poly_idx);
            }
        }
        for owners in edge_owners.values() {
            for i in 0..owners.len() {
                for j in (i + 1)..owners.len() {
                    let a = owners[i];
                    let b = owners[j];
                    if a != b {
                        if !polygons[a].neighbors.contains(&b) {
                            polygons[a].neighbors.push(b);
                        }
                        if !polygons[b].neighbors.contains(&a) {
                            polygons[b].neighbors.push(a);
                        }
                    }
                }
            }
        }
        for poly in &mut polygons {
            poly.neighbors.sort_unstable();
        }

        Ok(Self { polygons })
    }

    /// Finds the shortest walkable polygon corridor between `start_polygon` and `goal_polygon`
    /// using `O(E log V)` `BinaryHeap` Euclidean `A*` bounded by `prism_kernel::navMaxSearchNodes`.
    pub fn find_path_astar(
        &self,
        start_polygon: usize,
        goal_polygon: usize,
    ) -> Result<NavCorridorPath, NavigationError> {
        let count = self.polygons.len();
        if start_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: start_polygon,
                polygon_count: count,
            });
        }
        if goal_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: goal_polygon,
                polygon_count: count,
            });
        }

        if start_polygon == goal_polygon {
            let c = self.polygons[start_polygon].centroid;
            return Ok(NavCorridorPath {
                polygon_indices: vec![start_polygon],
                waypoints: vec![c],
                total_cost_meters: 0.0,
            });
        }

        let scale = verdant_tensor::kernel_consts().skin_weight_scale_f64;
        let max_nodes = crate::prism_kernel::navMaxSearchNodes().expect("prism") as usize;
        let goal_pos = self.polygons[goal_polygon].centroid;

        let mut g_milli = vec![u64::MAX; count];
        let mut came_from: Vec<Option<usize>> = vec![None; count];
        let mut closed = WordBitset::with_len(count);
        let mut open: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

        g_milli[start_polygon] = 0;
        let h0 =
            ((self.polygons[start_polygon].centroid - goal_pos).length() * scale).round() as u64;
        let f0 = crate::prism_kernel::astarPriorityMilli(0, h0).expect("prism");
        open.push(Reverse((f0, start_polygon)));

        let mut expanded = 0usize;
        while let Some(Reverse((_, current))) = open.pop() {
            if expanded >= max_nodes {
                break;
            }
            if closed.test(current) {
                continue;
            }
            if current == goal_polygon {
                let mut poly_path = vec![goal_polygon];
                let mut cursor = goal_polygon;
                while let Some(prev) = came_from[cursor] {
                    poly_path.push(prev);
                    cursor = prev;
                }
                poly_path.reverse();
                let waypoints: Vec<Vec3> = poly_path
                    .iter()
                    .map(|&idx| self.polygons[idx].centroid)
                    .collect();
                let mut total_cost_meters = 0.0;
                for pair in waypoints.windows(DIM_PLANAR) {
                    total_cost_meters += (pair[1] - pair[0]).length();
                }
                return Ok(NavCorridorPath {
                    polygon_indices: poly_path,
                    waypoints,
                    total_cost_meters,
                });
            }

            closed.set(current);
            expanded += 1;

            let cur_pos = self.polygons[current].centroid;
            for &nbr in &self.polygons[current].neighbors {
                if closed.test(nbr) {
                    continue;
                }
                let step_milli =
                    (((self.polygons[nbr].centroid - cur_pos).length() * scale).round() as u64)
                        .max(1);
                let tentative_g = g_milli[current].saturating_add(step_milli);
                if tentative_g < g_milli[nbr] {
                    came_from[nbr] = Some(current);
                    g_milli[nbr] = tentative_g;
                    let h =
                        ((self.polygons[nbr].centroid - goal_pos).length() * scale).round() as u64;
                    let f_score =
                        crate::prism_kernel::astarPriorityMilli(tentative_g, h).expect("prism");
                    open.push(Reverse((f_score, nbr)));
                }
            }
        }

        Err(NavigationError::UnreachableGoal {
            start_polygon,
            goal_polygon,
        })
    }

    /// Computes exact `(min, +)` all-pairs shortest-path distances in millimeters across the
    /// walkable polygon adjacency matrix via `verdant_tensor::tropical_min_plus_all_pairs_closure`.
    pub fn all_pairs_shortest_paths_tropical(&self) -> Result<Vec<Option<u64>>, NavigationError> {
        let count = self.polygons.len();
        if count == 0 {
            return Err(NavigationError::EmptyWalkableNavMesh);
        }
        let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;
        let mut edge_costs = vec![None; count * count];
        for (idx, poly) in self.polygons.iter().enumerate() {
            edge_costs[idx * count + idx] = Some(0);
            for &nbr in &poly.neighbors {
                if nbr < count {
                    let dist_milli = (((self.polygons[nbr].centroid - poly.centroid).length()
                        * scale)
                        .round() as u32)
                        .max(1);
                    edge_costs[idx * count + nbr] = Some(dist_milli);
                }
            }
        }
        verdant_tensor::tropical_min_plus_all_pairs_closure(&edge_costs, count)
            .map_err(|_| NavigationError::EmptyWalkableNavMesh)
    }

    /// Evaluates the exact tropical `(min, +)` shortest corridor distance in meters between
    /// `start_polygon` and `goal_polygon`.
    pub fn shortest_corridor_cost_tropical(
        &self,
        start_polygon: usize,
        goal_polygon: usize,
    ) -> Result<f64, NavigationError> {
        let count = self.polygons.len();
        if start_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: start_polygon,
                polygon_count: count,
            });
        }
        if goal_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: goal_polygon,
                polygon_count: count,
            });
        }
        let closure = self.all_pairs_shortest_paths_tropical()?;
        let scale = crate::prism_kernel::skinWeightScale().expect("prism") as f64;
        let cost_milli = closure[start_polygon * count + goal_polygon].ok_or(
            NavigationError::UnreachableGoal {
                start_polygon,
                goal_polygon,
            },
        )?;
        Ok((cost_milli as f64) / scale)
    }

    /// Computes both the exact shortest corridor path and relaxation count via in-place tropical
    /// `(min, +)` closure with `next_hop` witness recovery (`verdant-aqm` `tropicalMinPlusClosureWithPaths`).
    pub fn shortest_corridor_with_path_tropical(
        &self,
        start_polygon: usize,
        goal_polygon: usize,
    ) -> Result<(NavCorridorPath, usize), NavigationError> {
        let count = self.polygons.len();
        if start_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: start_polygon,
                polygon_count: count,
            });
        }
        if goal_polygon >= count {
            return Err(NavigationError::InvalidPolygonIndex {
                polygon_index: goal_polygon,
                polygon_count: count,
            });
        }
        let inf = verdant_tensor::tropical_inf_sentinel();
        let scale = verdant_tensor::kernel_consts().skin_weight_scale_f64;
        let mut dist = vec![inf; count * count];
        let mut next_hop = vec![-1_i32; count * count];
        for (idx, poly) in self.polygons.iter().enumerate() {
            dist[idx * count + idx] = 0;
            next_hop[idx * count + idx] = idx as i32;
            for &nbr in &poly.neighbors {
                if nbr < count {
                    let step_milli = (((self.polygons[nbr].centroid - poly.centroid).length()
                        * scale)
                        .round() as i32)
                        .max(1)
                        .min(inf - 1);
                    if step_milli < dist[idx * count + nbr] {
                        dist[idx * count + nbr] = step_milli;
                        next_hop[idx * count + nbr] = nbr as i32;
                    }
                }
            }
        }
        let relaxations =
            verdant_tensor::tropical_min_plus_closure_in_place(&mut dist, &mut next_hop, count)
                .map_err(|_| NavigationError::EmptyWalkableNavMesh)?;
        let poly_path = verdant_tensor::reconstruct_tropical_next_hop_path(
            &next_hop,
            count,
            start_polygon,
            goal_polygon,
        )
        .ok_or(NavigationError::UnreachableGoal {
            start_polygon,
            goal_polygon,
        })?;
        let waypoints: Vec<Vec3> = poly_path
            .iter()
            .map(|&idx| self.polygons[idx].centroid)
            .collect();
        let mut total_cost_meters = 0.0;
        for pair in waypoints.windows(DIM_PLANAR) {
            total_cost_meters += (pair[1] - pair[0]).length();
        }
        Ok((
            NavCorridorPath {
                polygon_indices: poly_path,
                waypoints,
                total_cost_meters,
            },
            relaxations,
        ))
    }
}

/// Computes a normalized steering direction from `position` toward the furthest corridor waypoint
/// within `lookahead_meters` (or the next unvisited waypoint).
pub fn steer_along_corridor(
    position: Vec3,
    corridor: &NavCorridorPath,
    lookahead_meters: f64,
) -> Result<Vec3, NavigationError> {
    if corridor.waypoints.is_empty() {
        return Err(NavigationError::EmptyWalkableNavMesh);
    }
    let radius = lookahead_meters.max(geometric_epsilon());
    let mut target = corridor.waypoints[0];
    for &wp in &corridor.waypoints {
        target = wp;
        if (wp - position).length() >= radius {
            break;
        }
    }
    let delta = target - position;
    if delta.length() <= geometric_epsilon() {
        Ok(Vec3::ZERO)
    } else {
        Ok(delta.normalized())
    }
}
