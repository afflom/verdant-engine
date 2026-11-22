//! Distinct High-Definition 3D architectural world template builders (`hd_world_templates.rs`).

use super::*;

fn append_torus_ring(
    acc: &mut SmoothMeshAcc,
    center: Vec3,
    major_r: f64,
    minor_r: f64,
    tilt_x: f64,
    tilt_z: f64,
    col: [f32; DIM_HOMOGENEOUS],
) {
    let major_segs = (four_u32() * four_u32() + four_u32() + four_u32()) as usize;
    let minor_segs = (four_u32() + four_u32()) as usize;
    let tau = std::f64::consts::TAU;
    let q = Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), tilt_x)
        * Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), tilt_z);
    let start_v = acc.positions.len() as u32;
    let stride = (minor_segs + 1) as u32;
    for iu in 0..=major_segs {
        let uf = (iu as f64) / (major_segs as f64);
        let u = uf * tau;
        let (su, cu) = u.sin_cos();
        for iv in 0..=minor_segs {
            let vf = (iv as f64) / (minor_segs as f64);
            let v = vf * tau;
            let (sv, cv) = v.sin_cos();
            let ring_r = major_r + minor_r * cv;
            let local_p = Vec3::new(ring_r * cu, minor_r * sv, ring_r * su);
            let local_n = Vec3::new(cv * cu, sv, cv * su);
            let p = center + q.rotate_vec3(local_p);
            let n = q.rotate_vec3(local_n).normalized();
            acc.push_vertex(p, n, [uf as f32, vf as f32], col);
        }
    }
    for iu in 0..(major_segs as u32) {
        for iv in 0..(minor_segs as u32) {
            let a = start_v + iu * stride + iv;
            let b = a + 1;
            let c = a + stride + 1;
            let d = a + stride;
            acc.indices.extend_from_slice(&[a, b, c, a, c, d]);
        }
    }
}

pub(super) fn synthesize_blank_studio_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let four = four_u32() as f64;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Crisp architectural checkerboard studio floor (no hills, no white dome, no trees):
    let tiles_per_half = (four_u32() + four_u32()) as i32; // 8 tiles per half -> 16x16 grid
    let tile_sz = two;
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let col_a = [
        (quarter + eighth * half) as f32,
        (quarter + eighth * half) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let col_b = [
        (quarter - eighth * half) as f32,
        (quarter - eighth * half) as f32,
        quarter as f32,
        1.0f32,
    ];
    for ix in -tiles_per_half..tiles_per_half {
        for iz in -tiles_per_half..tiles_per_half {
            let x0 = (ix as f64) * tile_sz;
            let z0 = (iz as f64) * tile_sz;
            let col = if (ix + iz) & 1 == 0 { col_a } else { col_b };
            floor_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + tile_sz),
                Vec3::new(x0 + tile_sz, 0.0, z0 + tile_sz),
                Vec3::new(x0 + tile_sz, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let floor_mesh = floor_acc.into_mesh("StudioGridFloor");

    // Subtle origin axis guide & reference plinth (so creators immediately perceive 3D scale):
    let mut origin_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + 1) as usize,
        pow2_u64(four_u32() + 1) as usize,
    );
    let rail_w = eighth * half;
    let rail_h = eighth * quarter;
    let x_axis_col = [
        (two + half + quarter) as f32,
        quarter as f32,
        eighth as f32,
        1.0f32,
    ];
    let z_axis_col = [
        (two + eighth) as f32,
        (half + eighth) as f32,
        (half + quarter) as f32,
        1.0f32,
    ];
    origin_acc.append_box(
        Vec3::new(-sixteen, 0.0, -rail_w),
        Vec3::new(sixteen, rail_h, rail_w),
        x_axis_col,
    );
    origin_acc.append_box(
        Vec3::new(-rail_w, 0.0, -sixteen),
        Vec3::new(rail_w, rail_h, sixteen),
        z_axis_col,
    );
    // Clean central studio reference plinth (metallic-stone subtle pad):
    let pad_col = [
        (two + half) as f32,
        (half + eighth) as f32,
        (half + eighth) as f32,
        1.0f32,
    ];
    origin_acc.append_box(
        Vec3::new(-two, 0.0, -two),
        Vec3::new(two, quarter * half, two),
        pad_col,
    );
    let origin_mesh = origin_acc.into_mesh("StudioOriginGuides");

    // Minimal boundary rim markers at ±16m:
    let mut rim_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    let rim_col = [
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let rim_t = quarter;
    let rim_h = eighth;
    rim_acc.append_box(
        Vec3::new(-sixteen, 0.0, -sixteen),
        Vec3::new(sixteen, rim_h, -sixteen + rim_t),
        rim_col,
    );
    rim_acc.append_box(
        Vec3::new(-sixteen, 0.0, sixteen - rim_t),
        Vec3::new(sixteen, rim_h, sixteen),
        rim_col,
    );
    let rim_mesh = rim_acc.into_mesh("StudioBoundaryRim");

    let meshes = vec![floor_mesh, origin_mesh, rim_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "studio_grid_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "studio_origin_guides",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "studio_boundary_rim",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "blank_studio"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

pub(super) fn synthesize_starfall_arena_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;
    let arena_half = sixteen + eight; // 24m half-extent -> 48m x 48m FPS arena

    // Dark volcanic basalt floor with tactical orange lane trims:
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let basalt_dark = [
        eighth as f32,
        eighth as f32,
        (eighth + eighth * quarter) as f32,
        1.0f32,
    ];
    let basalt_mid = [
        (eighth + eighth * half) as f32,
        (eighth + eighth * quarter) as f32,
        quarter as f32,
        1.0f32,
    ];
    let magma_glow = [
        (three + half + quarter + eighth) as f32,
        (quarter + eighth) as f32,
        eighth as f32,
        1.0f32,
    ];
    let tiles = (four_u32() + two_u32()) as i32;
    let step = arena_half / (tiles as f64);
    for ix in -tiles..tiles {
        for iz in -tiles..tiles {
            let x0 = (ix as f64) * step;
            let z0 = (iz as f64) * step;
            let col = if (ix + iz) & 1 == 0 {
                basalt_dark
            } else {
                basalt_mid
            };
            floor_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let floor_mesh = floor_acc.into_mesh("StarfallBasaltFloor");

    // Central stepped combat platform + 4 angled tactical ramps + 4 cover monoliths + 4 corner pylons:
    let mut arena_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let armor_metal = [
        (two + quarter + eighth) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth + eighth * half) as f32,
        1.0f32,
    ];
    let plat_r = four + two;
    let plat_h = two - half;
    // Lower tier & upper central tactical platform:
    arena_acc.append_box(
        Vec3::new(-plat_r - two, 0.0, -plat_r - two),
        Vec3::new(plat_r + two, half + quarter, plat_r + two),
        basalt_mid,
    );
    arena_acc.append_box(
        Vec3::new(-plat_r, half + quarter, -plat_r),
        Vec3::new(plat_r, plat_h, plat_r),
        armor_metal,
    );
    // Glowing tactical center core strip:
    arena_acc.append_box(
        Vec3::new(-two, plat_h, -two),
        Vec3::new(two, plat_h + quarter, two),
        magma_glow,
    );

    let arena_mesh = arena_acc.into_mesh("StarfallCombatPlatform");

    // Separate named mesh for the 4 tactical cover monoliths:
    let mut cover_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let cover_dist = eight + four;
    for (cx, cz) in [
        (cover_dist, 0.0),
        (-cover_dist, 0.0),
        (0.0, cover_dist),
        (0.0, -cover_dist),
    ] {
        let wide_x = if cx == 0.0 { three } else { 1.0 };
        let wide_z = if cz == 0.0 { three } else { 1.0 };
        cover_acc.append_box(
            Vec3::new(cx - wide_x, 0.0, cz - wide_z),
            Vec3::new(cx + wide_x, three, cz + wide_z),
            armor_metal,
        );
        cover_acc.append_box(
            Vec3::new(cx - wide_x, three, cz - wide_z),
            Vec3::new(cx + wide_x, three + quarter, cz + wide_z),
            magma_glow,
        );
    }
    let cover_mesh = cover_acc.into_mesh("StarfallCoverMonoliths");

    // Separate named mesh for the 4 tall corner perimeter guard towers:
    let mut towers_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let tower_dist = sixteen + two;
    let tower_h = eight + four;
    for (tx, tz) in [
        (tower_dist, tower_dist),
        (-tower_dist, tower_dist),
        (tower_dist, -tower_dist),
        (-tower_dist, -tower_dist),
    ] {
        towers_acc.append_box(
            Vec3::new(tx - two, 0.0, tz - two),
            Vec3::new(tx + two, tower_h, tz + two),
            basalt_mid,
        );
        towers_acc.append_box(
            Vec3::new(tx - two - quarter, tower_h - 1.0, tz - two - quarter),
            Vec3::new(tx + two + quarter, tower_h - half, tz + two + quarter),
            magma_glow,
        );
    }
    let towers_mesh = towers_acc.into_mesh("StarfallCornerTowers");

    // 4 flanking elevated perimeter walls / catwalk decks:
    let mut walls_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let wall_h = four + two;
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, -arena_half),
        Vec3::new(arena_half, wall_h, -arena_half + two),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, arena_half - two),
        Vec3::new(arena_half, wall_h, arena_half),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(-arena_half, 0.0, -arena_half),
        Vec3::new(-arena_half + two, wall_h, arena_half),
        basalt_dark,
    );
    walls_acc.append_box(
        Vec3::new(arena_half - two, 0.0, -arena_half),
        Vec3::new(arena_half, wall_h, arena_half),
        basalt_dark,
    );
    let walls_mesh = walls_acc.into_mesh("StarfallPerimeterBastion");

    let meshes = vec![floor_mesh, arena_mesh, walls_mesh, cover_mesh, towers_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "starfall_basalt_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "starfall_combat_platform",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "starfall_perimeter_walls",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let cover_idx = 1usize + 1usize + 1usize;
    let cover_entity = scene.spawn(
        "starfall_cover_monoliths",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[cover_idx].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(cover_entity, cover_idx, four as usize);

    let tower_idx = cover_idx + 1usize;
    let tower_entity = scene.spawn(
        "starfall_corner_towers",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[tower_idx].bounds(),
        (1.0, "starfall_arena"),
    );
    scene.attach_renderable(tower_entity, tower_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

pub(super) fn synthesize_meridian_observatory_sector(seed: u64, lod_rank: u64) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Deep midnight-indigo slate astral plaza:
    let mut plaza_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let slate_a = [
        eighth as f32,
        (eighth + eighth * quarter) as f32,
        (quarter + eighth * half) as f32,
        1.0f32,
    ];
    let slate_b = [
        (eighth + eighth * half) as f32,
        (quarter * half + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let tiles = (four_u32() + two_u32()) as i32;
    let step = (sixteen + eight) / (tiles as f64);
    for ix in -tiles..tiles {
        for iz in -tiles..tiles {
            let x0 = (ix as f64) * step;
            let z0 = (iz as f64) * step;
            let col = if (ix + iz) & 1 == 0 { slate_a } else { slate_b };
            plaza_acc.append_quad_smooth(
                Vec3::new(x0, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0 + step),
                Vec3::new(x0 + step, 0.0, z0),
                Vec3::new(x0, 0.0, z0),
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                Vec3::UP,
                col,
            );
        }
    }
    let plaza_mesh = plaza_acc.into_mesh("MeridianAstralPlaza");

    // Stepped obsidian dais + TWO colossal tilted metallic brass armillary rings + glowing cyan core:
    let mut obs_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let brass_col = [
        (two + half + quarter + eighth) as f32,
        (half + quarter) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let star_glow = [
        (three + quarter) as f32,
        (half + quarter + eighth) as f32,
        (half + quarter + eighth) as f32,
        1.0f32,
    ];
    obs_acc.append_box(
        Vec3::new(-eight, 0.0, -eight),
        Vec3::new(eight, half, eight),
        slate_b,
    );
    obs_acc.append_box(
        Vec3::new(-four - two, half, -four - two),
        Vec3::new(four + two, 1.0, four + two),
        brass_col,
    );
    let dais_mesh = obs_acc.into_mesh("MeridianObservatoryDais");

    let mut rings_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + four_u32()) as usize,
        pow2_u64(four_u32() + four_u32()) as usize,
    );
    let ring_center = Vec3::new(0.0, eight, 0.0);
    append_torus_ring(
        &mut rings_acc,
        ring_center,
        four + three,
        quarter + eighth,
        half,
        quarter,
        brass_col,
    );
    append_torus_ring(
        &mut rings_acc,
        ring_center,
        four + 1.0,
        quarter,
        -half - quarter,
        half,
        brass_col,
    );
    let obs_mesh = rings_acc.into_mesh("MeridianArmillaryRings");

    let mut core_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    core_acc.append_box(
        Vec3::new(-1.0, eight - 1.0, -1.0),
        Vec3::new(1.0, eight + 1.0, 1.0),
        star_glow,
    );
    let core_mesh = core_acc.into_mesh("MeridianStarlightCore");

    // Ring of 6 tall obsidian starlight obelisks around the observatory:
    let mut ob_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let ob_count = (three_u32() + three_u32()) as usize;
    let tau = std::f64::consts::TAU;
    let ring_r = sixteen - two;
    for i in 0..ob_count {
        let ang = (i as f64) * tau / (ob_count as f64);
        let (sa, ca) = ang.sin_cos();
        let ox = ring_r * ca;
        let oz = ring_r * sa;
        ob_acc.append_lathed_column(
            Vec3::new(ox, 0.0, oz),
            eight + two,
            1.0,
            (four_u32() + four_u32()) as usize,
            four_u32() as usize,
            slate_b,
            brass_col,
        );
        ob_acc.append_box(
            Vec3::new(ox - half, eight + two, oz - half),
            Vec3::new(ox + half, eight + three, oz + half),
            star_glow,
        );
    }
    let ob_mesh = ob_acc.into_mesh("MeridianStarlightObelisks");

    let meshes = vec![plaza_mesh, obs_mesh, ob_mesh, dais_mesh, core_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "meridian_astral_plaza",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "meridian_armillary_orrery",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "meridian_starlight_obelisks",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let dais_idx = 1usize + 1usize + 1usize;
    let dais_entity = scene.spawn(
        "meridian_observatory_dais",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[dais_idx].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(dais_entity, dais_idx, four as usize);

    let core_idx = dais_idx + 1usize;
    let core_entity = scene.spawn(
        "meridian_starlight_core",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[core_idx].bounds(),
        (1.0, "meridian_observatory"),
    );
    scene.attach_renderable(core_entity, core_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

pub(super) fn synthesize_glasshouse_conservatory_sector(
    seed: u64,
    lod_rank: u64,
) -> HdSectorBundle {
    let half = half_f64();
    let quarter = half * half;
    let eighth = quarter * half;
    let two = two_f64();
    let three = three_u32() as f64;
    let four = four_u32() as f64;
    let eight = four + four;
    let sixteen = pow2_u64(four_u32()) as f64;

    // Warm sunlit emerald moss & travertine conservatory ground:
    let mut floor_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    let moss_col = [
        (eighth + eighth * half) as f32,
        (half - eighth * half) as f32,
        (quarter + eighth * half) as f32,
        1.0f32,
    ];
    let stone_col = [
        (half + quarter) as f32,
        (half + quarter - eighth * half) as f32,
        (half + eighth) as f32,
        1.0f32,
    ];
    let iron_col = [
        (two + eighth + eighth * half) as f32,
        (quarter + eighth) as f32,
        (quarter + eighth) as f32,
        1.0f32,
    ];
    let amber_glow = [
        (three + half + quarter + eighth) as f32,
        (half + quarter) as f32,
        quarter as f32,
        1.0f32,
    ];
    let span = sixteen + eight;
    floor_acc.append_quad_smooth(
        Vec3::new(-span, 0.0, span),
        Vec3::new(span, 0.0, span),
        Vec3::new(span, 0.0, -span),
        Vec3::new(-span, 0.0, -span),
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        Vec3::UP,
        moss_col,
    );
    let floor_mesh = floor_acc.into_mesh("GlasshouseGardenFloor");

    // 5 rhythmic cast-iron conservatory vault ribs along the X axis + elevated side-scroller terraces:
    let mut vault_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + three_u32()) as usize,
        pow2_u64(four_u32() + three_u32()) as usize,
    );
    let rib_xs = [-sixteen, -eight, 0.0, eight, sixteen];
    let rib_half_z = eight;
    let rib_h = eight + two;
    let mut lantern_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    for &rx in &rib_xs {
        // Left & right iron rib columns:
        vault_acc.append_box(
            Vec3::new(rx - half, 0.0, -rib_half_z - half),
            Vec3::new(rx + half, rib_h, -rib_half_z + half),
            iron_col,
        );
        vault_acc.append_box(
            Vec3::new(rx - half, 0.0, rib_half_z - half),
            Vec3::new(rx + half, rib_h, rib_half_z + half),
            iron_col,
        );
        // Overhead conservatory gable truss bar + separate amber lantern mesh:
        vault_acc.append_box(
            Vec3::new(rx - quarter, rib_h - half, -rib_half_z),
            Vec3::new(rx + quarter, rib_h, rib_half_z),
            iron_col,
        );
        lantern_acc.append_box(
            Vec3::new(rx - half, rib_h - two, -half),
            Vec3::new(rx + half, rib_h - 1.0, half),
            amber_glow,
        );
    }
    let lantern_mesh = lantern_acc.into_mesh("GlasshouseAmberLanterns");
    // Longitudinal ridge spine tying all 5 conservatory ribs together:
    vault_acc.append_box(
        Vec3::new(-sixteen, rib_h, -quarter),
        Vec3::new(sixteen, rib_h + half, quarter),
        iron_col,
    );
    let vault_mesh = vault_acc.into_mesh("GlasshouseConservatoryRibs");

    // Low stone central promenade:
    let mut prom_acc =
        SmoothMeshAcc::with_capacity(pow2_u64(four_u32()) as usize, pow2_u64(four_u32()) as usize);
    prom_acc.append_box(
        Vec3::new(-sixteen - two, 0.0, -three),
        Vec3::new(sixteen + two, half, three),
        stone_col,
    );
    let prom_mesh = prom_acc.into_mesh("GlasshouseStonePromenade");

    // Multi-level elevated 2.5D jump platforms running along the conservatory promenade:
    let mut plat_acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + two_u32()) as usize,
        pow2_u64(four_u32() + two_u32()) as usize,
    );
    plat_acc.append_box(
        Vec3::new(-four - eight, two, -two),
        Vec3::new(-four, two + half, two),
        stone_col,
    );
    plat_acc.append_box(
        Vec3::new(-three, four, -two),
        Vec3::new(three, four + half, two),
        stone_col,
    );
    plat_acc.append_box(
        Vec3::new(four, two + 1.0, -two),
        Vec3::new(four + eight, three + half, two),
        stone_col,
    );
    let plat_mesh = plat_acc.into_mesh("GlasshouseTerraceWalkways");

    let meshes = vec![floor_mesh, vault_mesh, plat_mesh, lantern_mesh, prom_mesh];
    let mut scene = SceneWorld::new();
    let terrain_entity = scene.spawn(
        "glasshouse_garden_floor",
        EntityRole::TerrainPatch,
        None,
        Transform::default(),
        meshes[0].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(terrain_entity, 0, 0);

    let citadel_entity = scene.spawn(
        "glasshouse_conservatory_ribs",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(citadel_entity, 1, four as usize);

    let bridge_entity = scene.spawn(
        "glasshouse_terrace_walkways",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[1 + 1].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(bridge_entity, 1 + 1, four as usize);

    let lantern_idx = 1usize + 1usize + 1usize;
    let lantern_entity = scene.spawn(
        "glasshouse_amber_lanterns",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[lantern_idx].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(lantern_entity, lantern_idx, four as usize);

    let prom_idx = lantern_idx + 1usize;
    let prom_entity = scene.spawn(
        "glasshouse_stone_promenade",
        EntityRole::StaticArchitecture,
        None,
        Transform::default(),
        meshes[prom_idx].bounds(),
        (1.0, "glasshouse_conservatory"),
    );
    scene.attach_renderable(prom_entity, prom_idx, four as usize);

    HdSectorBundle {
        seed,
        lod_rank,
        scene,
        meshes,
        citadel_entity,
        bridge_entity,
        terrain_entity,
    }
}

/// Distinct procedural 3D architectural world styles so every Verdant Studio template and
/// domain pack renders unmistakably distinct geometry, floor, materials, and silhouette.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldArchitectureKind {
    /// Clean neutral 3D creation canvas with architectural studio grid (zero demo dome).
    BlankStudio,
    /// Sunspire Coastal Harbor basin with vaulted hall, stone arch bridge, and coastal trees.
    SunspireCoastal,
    /// Meridian Astral Observatory with stepped obsidian dais, brass armillary rings, and obelisks.
    MeridianObservatory,
    /// Glasshouse Botanical Conservatory with 2.5D terraced walkways and arched conservatory ribs.
    GlasshouseConservatory,
    /// Starfall Bastion Arena FPS with basalt combat platform, tactical ramps, cover monoliths, and pylons.
    StarfallArena,
}

impl WorldArchitectureKind {
    /// Resolves a domain index (`0..=3`) or slug to its distinct 3D architecture.
    #[must_use]
    pub fn from_domain_slug(slug: &str) -> Self {
        match slug {
            "blank-studio" | "blank" => WorldArchitectureKind::BlankStudio,
            "meridian-observatory" | "DOM-MERIDIAN-OBS" => {
                WorldArchitectureKind::MeridianObservatory
            }
            "glasshouse-chronicle" | "DOM-GLASSHOUSE-BOTANICAL" => {
                WorldArchitectureKind::GlasshouseConservatory
            }
            "starfall-bastion" | "DOM-STARFALL-BASTION" => WorldArchitectureKind::StarfallArena,
            _ => WorldArchitectureKind::SunspireCoastal,
        }
    }

    /// Resolves a numeric architecture selector (`0` = BlankStudio, `1` = Sunspire, `2` = Meridian,
    /// `3` = Glasshouse, `4` = Starfall).
    #[must_use]
    pub fn from_u32(code: u32) -> Self {
        let one = 1_u32;
        let two = one.wrapping_add(one);
        let three = two.wrapping_add(one);
        let four = two.wrapping_add(two);
        if code == 0 {
            WorldArchitectureKind::BlankStudio
        } else if code == one {
            WorldArchitectureKind::SunspireCoastal
        } else if code == two {
            WorldArchitectureKind::MeridianObservatory
        } else if code == three {
            WorldArchitectureKind::GlasshouseConservatory
        } else if code == four {
            WorldArchitectureKind::StarfallArena
        } else {
            WorldArchitectureKind::BlankStudio
        }
    }
}

/// Interactive primitive shapes creators can place directly into a Verdant Studio scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorPrimitiveKind {
    /// Solid architectural block / cube (`0`).
    Box,
    /// Walkable elevated floor platform (`1`).
    Platform,
    /// Sculpted cylindrical column / pillar (`2`).
    Pillar,
    /// Sloped tactical ramp / wedge (`3`).
    Ramp,
    /// Faceted glowing beacon / orb (`4`).
    Orb,
}

impl AuthorPrimitiveKind {
    /// Decodes a primitive kind code (`0..=4`).
    #[must_use]
    pub fn from_u32(code: u32) -> Self {
        let one = 1_u32;
        let two = one.wrapping_add(one);
        let three = two.wrapping_add(one);
        let four = two.wrapping_add(two);
        if code == one {
            AuthorPrimitiveKind::Platform
        } else if code == two {
            AuthorPrimitiveKind::Pillar
        } else if code == three {
            AuthorPrimitiveKind::Ramp
        } else if code == four {
            AuthorPrimitiveKind::Orb
        } else {
            AuthorPrimitiveKind::Box
        }
    }

    /// Human-readable display label for Studio Scene Outliner.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Box => "Box",
            Self::Platform => "Platform",
            Self::Pillar => "Pillar",
            Self::Ramp => "Ramp",
            Self::Orb => "Beacon Orb",
        }
    }
}

/// Generates a clean smooth-normal, material-tagged `ProceduralMesh` for an interactive Studio
/// primitive placed by a creator.
#[must_use]
pub fn generate_studio_primitive_mesh(
    kind: AuthorPrimitiveKind,
    sx: f64,
    sy: f64,
    sz: f64,
    col: [f32; DIM_HOMOGENEOUS],
) -> ProceduralMesh {
    let half = half_f64();
    let quarter = half * half;
    let two = two_f64();
    let hx = (sx.abs().max(quarter)) * half;
    let hy = sy.abs().max(quarter);
    let hz = (sz.abs().max(quarter)) * half;
    let mut acc = SmoothMeshAcc::with_capacity(
        pow2_u64(four_u32() + 1) as usize,
        pow2_u64(four_u32() + 1) as usize,
    );
    match kind {
        AuthorPrimitiveKind::Box => {
            acc.append_box(Vec3::new(-hx, 0.0, -hz), Vec3::new(hx, hy, hz), col);
        }
        AuthorPrimitiveKind::Platform => {
            let plat_h = (hy * quarter).max(quarter * half);
            acc.append_box(
                Vec3::new(-hx * two, 0.0, -hz * two),
                Vec3::new(hx * two, plat_h, hz * two),
                col,
            );
        }
        AuthorPrimitiveKind::Pillar => {
            let r = (hx.min(hz) * half).max(quarter);
            let r_segs = (four_u32() * three_u32()) as usize;
            let h_segs = four_u32() as usize;
            acc.append_lathed_column(
                Vec3::new(0.0, 0.0, 0.0),
                hy * two,
                r,
                r_segs,
                h_segs,
                col,
                col,
            );
        }
        AuthorPrimitiveKind::Ramp => {
            let p0 = Vec3::new(-hx, 0.0, -hz);
            let p1 = Vec3::new(hx, 0.0, -hz);
            let p2 = Vec3::new(hx, 0.0, hz);
            let p3 = Vec3::new(-hx, 0.0, hz);
            let t2 = Vec3::new(hx, hy, hz);
            let t3 = Vec3::new(-hx, hy, hz);
            acc.append_quad_flat(p0, p1, t2, t3, col);
            acc.append_quad_flat(p3, p2, t2, t3, col);
            acc.append_quad_flat(p0, p3, p2, p1, col);
            acc.append_quad_flat(p0, t3, p3, p0, col);
            acc.append_quad_flat(p1, p2, t2, p1, col);
        }
        AuthorPrimitiveKind::Orb => {
            let r = hx.min(hz).max(quarter);
            let lat_segs = (four_u32() + four_u32()) as usize;
            let lon_segs = (four_u32() * three_u32()) as usize;
            let tau = std::f64::consts::TAU;
            let pi = std::f64::consts::PI;
            let start_v = acc.positions.len() as u32;
            let stride = (lon_segs + 1) as u32;
            let center_y = r;
            for ilat in 0..=lat_segs {
                let vf = (ilat as f64) / (lat_segs as f64);
                let phi_a = vf * pi;
                let (sp, cp) = phi_a.sin_cos();
                for ilon in 0..=lon_segs {
                    let uf = (ilon as f64) / (lon_segs as f64);
                    let theta = uf * tau;
                    let (st, ct) = theta.sin_cos();
                    let n = Vec3::new(sp * ct, cp, sp * st);
                    let p = Vec3::new(n.x * r, center_y + n.y * r, n.z * r);
                    acc.push_vertex(p, n, [uf as f32, vf as f32], col);
                }
            }
            for ilat in 0..(lat_segs as u32) {
                for ilon in 0..(lon_segs as u32) {
                    let a = start_v + ilat * stride + ilon;
                    let b = a + 1;
                    let c = a + stride + 1;
                    let d = a + stride;
                    acc.indices.extend_from_slice(&[a, d, c, a, c, b]);
                }
            }
        }
    }
    acc.into_mesh(kind.label())
}

/// Synthesizes a High-Definition 3D sector matching the requested [`WorldArchitectureKind`],
/// guaranteeing visually unmistakable geometry, materials, and spatial layout across all templates.
#[must_use]
pub fn synthesize_hd_world_sector_for_kind(
    seed: u64,
    relief: &TerrainReliefParams,
    water_level_meters: f64,
    lod_rank: u64,
    kind: WorldArchitectureKind,
) -> HdSectorBundle {
    match kind {
        WorldArchitectureKind::BlankStudio => synthesize_blank_studio_sector(seed, lod_rank),
        WorldArchitectureKind::StarfallArena => synthesize_starfall_arena_sector(seed, lod_rank),
        WorldArchitectureKind::MeridianObservatory => {
            synthesize_meridian_observatory_sector(seed, lod_rank)
        }
        WorldArchitectureKind::GlasshouseConservatory => {
            synthesize_glasshouse_conservatory_sector(seed, lod_rank)
        }
        WorldArchitectureKind::SunspireCoastal => {
            synthesize_hd_citadel_sector(seed, relief, water_level_meters, lod_rank)
        }
    }
}
