use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, wireframe::WireframeConfig},
    color::palettes::css::*,
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection},
};
use std::f32::consts::*;

pub fn setup_lighting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn camera
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            aspect_ratio: 16.0 / 9.0,
            fov: std::f32::consts::FRAC_PI_3,
            near: 0.1,
            far: 1000.0,
            ..default()
        }),
        Camera {
            hdr : true,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 5.0).looking_at(Vec3::Y * 0.5, Vec3::Y),
        /*
        DistanceFog {
            color: Color::srgb_u8(43, 44, 47),
            falloff: FogFalloff::Linear {
                start: 1.0,
                end: 8.0,
            },
            ..default()
        },
        */
        EnvironmentMapLight {
        diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
        specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
        intensity: 2000.0,
        ..default()
    }
    ));

    // spawn gltf asset
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("FlightHelmet/FlightHelmet.gltf"),
        )),
        Transform {
            translation: Vec3::new(2.5, 1.0, 0.0), // position
            scale: Vec3::splat(2.0),              // uniform scale (downsize FlightHelmet)
            rotation: Quat::from_rotation_y(0.0),  // no rotation
        },
        GlobalTransform::default(),               // required by Bevy's transform system
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            ..default()
        })),
    ));


    //spawn cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DEEP_PINK.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )); 

    // directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
    ));
}

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_secs() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}

pub fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
