use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const PLAYER_SPEED: f32 = 300.0;
const AIR_CONTROL: f32 = 1.0; // 1.0 = full control in air, 0.5 = half control, etc.
const JUMP_SPEED: f32 = 700.0; // Increased from 500.0 for higher jumps
const GRAVITY: f32 = 2000.0;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Platform {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Fruit;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Grounded(bool);

// Game state resources
#[derive(Resource)]
struct GameState {
    lives: u32,
    level: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            lives: 3,
            level: 1,
        }
    }
}

// UI Components
#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct LevelText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_systems(Startup, (
            setup_camera, 
            setup_player, 
            setup_platforms, 
            setup_fruits.after(setup_platforms), 
            setup_ui
        ))
        .add_systems(Update, (
            player_movement,
            apply_gravity,
            apply_velocity,
            check_collisions,
            check_fruit_collection,
            check_player_death,
            update_ui,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ui(mut commands: Commands) {
    // Lives text as 2D world text (top left)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Lives: 3",
                TextStyle {
                    font_size: 50.0,
                    color: Color::srgb(1.0, 1.0, 0.0), // Bright yellow
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 150.0, WINDOW_HEIGHT / 2.0 - 50.0, 10.0)),
            ..default()
        },
        LivesText,
    ));

    // Level text as 2D world text (top right)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Level: 1",
                TextStyle {
                    font_size: 50.0,
                    color: Color::srgb(0.0, 1.0, 1.0), // Bright cyan
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 - 150.0, WINDOW_HEIGHT / 2.0 - 50.0, 10.0)),
            ..default()
        },
        LevelText,
    ));

    // Game title in center top
    commands.spawn(
        Text2dBundle {
            text: Text::from_section(
                "BEVY PLATFORMER",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 0.5, 0.0), // Orange
                    ..default()
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT / 2.0 - 50.0, 10.0)),
            ..default()
        }
    );
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        },
        Player,
        Velocity { x: 0.0, y: 0.0 },
        Grounded(false),
    ));
}

fn setup_platforms(mut commands: Commands) {
    // Use current time for initial random seed
    let initial_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
        
    generate_random_platforms_with_seed(&mut commands, initial_seed);
}

fn generate_random_platforms_with_seed(commands: &mut Commands, seed: u64) {
    use bevy::math::Vec3;
    
    const MIN_PLATFORM_DISTANCE: f32 = 80.0; // Minimum distance between platform edges
    const PLAYER_SIZE: f32 = 50.0; // Player is 50x50
    const MIN_GAP_FOR_PLAYER: f32 = PLAYER_SIZE + 30.0; // Extra space for comfortable movement
    const MIN_VERTICAL_GAP: f32 = 60.0; // Minimum vertical space for jumping
    
    // Always ensure there's a starting platform near the player first
    let starting_platform = (0.0, 100.0, 200.0); // x, y, width
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(starting_platform.2, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(starting_platform.0, starting_platform.1, 0.0)),
            ..default()
        },
        Platform { width: starting_platform.2, height: 20.0 },
    ));
    
    // Keep track of all platforms (including starting platform)
    let mut platforms = vec![starting_platform];
    
    // Simple linear congruential generator for pseudo-random numbers
    let mut rng_state = seed;
    let mut next_rand = || {
        rng_state = (rng_state.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        rng_state
    };
    
    // Generate 6-10 random platforms with proper spacing
    let num_platforms = 6 + (next_rand() % 5) as usize;
    let mut attempts = 0;
    let max_attempts = num_platforms * 10; // Limit attempts to prevent infinite loops
    
    while platforms.len() < num_platforms + 1 && attempts < max_attempts {
        attempts += 1;
        
        // Generate random position and size
        let width = 120.0 + ((next_rand() % 1000) as f32 / 1000.0) * 100.0; // Width between 120-220
        let x = ((next_rand() % 1000) as f32 / 1000.0 - 0.5) * (WINDOW_WIDTH - width - 100.0);
        let y = ((next_rand() % 1000) as f32 / 1000.0 - 0.5) * (WINDOW_HEIGHT - 150.0);
        
        // Check if this position is valid (enough space from other platforms)
        let mut valid_position = true;
        
        for &(existing_x, existing_y, existing_width) in &platforms {
            let distance_x = (x - existing_x).abs();
            let distance_y = (y - existing_y).abs();
            
            // Calculate required horizontal spacing
            let required_horizontal_gap = (width / 2.0) + (existing_width / 2.0) + MIN_GAP_FOR_PLAYER;
            
            // Check horizontal overlap/proximity
            if distance_x < required_horizontal_gap {
                // If horizontally close, need enough vertical separation
                if distance_y < MIN_VERTICAL_GAP {
                    valid_position = false;
                    break;
                }
            }
            
            // Check if platforms are too close in general
            let total_distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
            if total_distance < MIN_PLATFORM_DISTANCE {
                valid_position = false;
                break;
            }
        }
        
        // Don't place platforms too close to starting area
        if x.abs() < 120.0 && (y - 100.0).abs() < 70.0 {
            valid_position = false;
        }
        
        // Keep platforms reasonably within bounds
        if x.abs() > WINDOW_WIDTH / 2.0 - width / 2.0 - 50.0 || 
           y.abs() > WINDOW_HEIGHT / 2.0 - 100.0 {
            valid_position = false;
        }
        
        if valid_position {
            // Add platform to our tracking list
            platforms.push((x, y, width));
            
            // Spawn the platform
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.5, 0.5, 0.5),
                        custom_size: Some(Vec2::new(width, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..default()
                },
                Platform { width, height: 20.0 },
            ));
        }
    }
}

fn setup_fruits_with_seed(mut commands: Commands, query: Query<(Entity, &Transform), (With<Platform>, Without<Player>)>, seed: u64) {
    use bevy::math::Vec3;
    
    // Collect all platform positions (excluding the starting platform at y=100.0 where player spawns)
    let mut platform_positions: Vec<Vec3> = query
        .iter()
        .map(|(_, transform)| transform.translation)
        .filter(|pos| pos.y != 100.0) // Exclude starting platform
        .collect();
    
    if platform_positions.is_empty() {
        return; // No platforms available for fruit placement
    }
    
    // Simple LCG for random selection
    let mut rng_state = seed.wrapping_mul(73);
    rng_state = (rng_state.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
    
    // Select a random platform
    let index = (rng_state as usize) % platform_positions.len();
    let platform_pos = platform_positions[index];
    
    // Place fruit on top of the selected platform (platform height is 20.0, fruit height is 25.0)
    let fruit_position = Vec3::new(platform_pos.x, platform_pos.y + 10.0 + 12.5, 0.0);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.0), // Orange color for fruit
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(fruit_position),
            ..default()
        },
        Fruit,
    ));
}

fn generate_random_platforms(commands: &mut Commands) {
    use bevy::math::Vec3;
    use std::collections::HashSet;
    
    // Always ensure there's a starting platform near the player first
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(200.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..default()
        },
        Platform { width: 200.0, height: 20.0 },
    ));
    
    // Generate 6-10 random platforms
    let num_platforms = 7 + (std::ptr::addr_of!(commands) as usize % 4); // Pseudo-random 7-10
    let mut used_positions = HashSet::new();
    
    for i in 0..num_platforms {
        // Create pseudo-random values based on current state
        let seed1 = (i * 73 + std::ptr::addr_of!(commands) as usize) % 1000;
        let seed2 = (i * 137 + std::ptr::addr_of!(commands) as usize * 2) % 1000;
        let seed3 = (i * 211 + std::ptr::addr_of!(commands) as usize * 3) % 1000;
        
        // Generate random position
        let x = (seed1 as f32 / 1000.0 - 0.5) * (WINDOW_WIDTH - 300.0);
        let y = (seed2 as f32 / 1000.0 - 0.5) * (WINDOW_HEIGHT - 200.0);
        let width = 120.0 + (seed3 as f32 / 1000.0) * 100.0; // Width between 120-220
        
        // Skip if too close to starting area
        if x.abs() < 150.0 && (y - 100.0).abs() < 80.0 {
            continue;
        }
        
        // Convert to grid position to avoid overlaps
        let grid_x = (x / 100.0).round() as i32;
        let grid_y = (y / 100.0).round() as i32;
        
        if used_positions.contains(&(grid_x, grid_y)) {
            continue;
        }
        used_positions.insert((grid_x, grid_y));
        
        let final_x = grid_x as f32 * 100.0;
        let final_y = grid_y as f32 * 100.0;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(width, 20.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(final_x, final_y, 0.0)),
                ..default()
            },
            Platform { width, height: 20.0 },
        ));
    }
}

fn setup_fruits(mut commands: Commands, query: Query<(Entity, &Transform), (With<Platform>, Without<Player>)>) {
    // Use current time for initial random seed
    let initial_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
        
    setup_fruits_with_seed(commands, query, initial_seed + 99);
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &Grounded), With<Player>>,
) {
    if let Ok((mut velocity, grounded)) = player_query.get_single_mut() {
        // Horizontal movement - works in air and on ground
        let mut horizontal_input = 0.0;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            horizontal_input -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            horizontal_input += 1.0;
        }
        
        // Apply horizontal movement with air control
        let movement_multiplier = if grounded.0 { 1.0 } else { AIR_CONTROL };
        velocity.x = horizontal_input * PLAYER_SPEED * movement_multiplier;

        // Jumping - only when grounded
        if (keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW)) && grounded.0 {
            velocity.y = JUMP_SPEED;
        }
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y -= GRAVITY * time.delta_seconds();
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_collisions(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    platform_query: Query<(&Transform, &Platform), Without<Player>>,
) {
    if let Ok((mut player_transform, mut velocity, mut grounded)) = player_query.get_single_mut() {
        grounded.0 = false;
        const GROUNDED_TOLERANCE: f32 = 5.0;
        
        for (platform_transform, platform) in platform_query.iter() {
            let player_pos = player_transform.translation;
            let platform_pos = platform_transform.translation;
            
            // Player bounds (50x50 sprite)
            let player_left = player_pos.x - 25.0;
            let player_right = player_pos.x + 25.0;
            let player_bottom = player_pos.y - 25.0;
            let player_top = player_pos.y + 25.0;
            
            // Platform bounds - use the actual platform size
            let platform_width = platform.width;
            
            let platform_left = platform_pos.x - platform_width / 2.0;
            let platform_right = platform_pos.x + platform_width / 2.0;
            let platform_bottom = platform_pos.y - 10.0;
            let platform_top = platform_pos.y + 10.0;
            
            // Check for collision
            if player_right > platform_left &&
               player_left < platform_right &&
               player_top > platform_bottom &&
               player_bottom < platform_top {
                
                // Determine collision direction and resolve
                let overlap_x = f32::min(player_right - platform_left, platform_right - player_left);
                let overlap_y = f32::min(player_top - platform_bottom, platform_top - player_bottom);
                
                if overlap_x < overlap_y {
                    // Horizontal collision
                    if player_pos.x < platform_pos.x {
                        // Player is on the left
                        player_transform.translation.x = platform_left - 25.0;
                    } else {
                        // Player is on the right
                        player_transform.translation.x = platform_right + 25.0;
                    }
                    velocity.x = 0.0;
                } else {
                    // Vertical collision
                    if player_pos.y < platform_pos.y {
                        // Player is below platform (hitting from below)
                        player_transform.translation.y = platform_bottom - 25.0;
                        velocity.y = 0.0;
                    } else {
                        // Player is above platform (landing on top)
                        player_transform.translation.y = platform_top + 25.0;
                        if velocity.y <= 0.0 { // Only stop downward velocity
                            velocity.y = 0.0;
                        }
                        grounded.0 = true;
                    }
                }
            }
            
            // Additional grounded check - more lenient for jumping
            if player_right > platform_left &&
               player_left < platform_right &&
               player_bottom <= platform_top + GROUNDED_TOLERANCE &&
               player_bottom >= platform_top - GROUNDED_TOLERANCE &&
               velocity.y <= 0.0 {
                grounded.0 = true;
            }
        }
        
        // Keep player within window bounds
        let half_width = WINDOW_WIDTH / 2.0;
        if player_transform.translation.x < -half_width + 25.0 {
            player_transform.translation.x = -half_width + 25.0;
        } else if player_transform.translation.x > half_width - 25.0 {
            player_transform.translation.x = half_width - 25.0;
        }
        
        // Reset if player falls too far
        if player_transform.translation.y < -WINDOW_HEIGHT {
            player_transform.translation = Vec3::new(0.0, 200.0, 0.0);
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
    }
}

fn check_fruit_collection(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    fruit_query: Query<(Entity, &Transform), (With<Fruit>, Without<Player>)>,
    platform_query: Query<(Entity, &Transform), (With<Platform>, Without<Player>)>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok((mut player_transform, mut velocity)) = player_query.get_single_mut() {
        for (fruit_entity, fruit_transform) in fruit_query.iter() {
            let distance = player_transform.translation.distance(fruit_transform.translation);
            
            // Check if player is close enough to collect the fruit (collision detection)
            if distance < 30.0 {
                // Remove the fruit
                commands.entity(fruit_entity).despawn();
                
                // Increase level
                game_state.level += 1;
                
                // Remove all existing platforms
                for (platform_entity, _) in platform_query.iter() {
                    commands.entity(platform_entity).despawn();
                }
                
                // Reset player position and velocity
                player_transform.translation = Vec3::new(0.0, 200.0, 0.0);
                velocity.x = 0.0;
                velocity.y = 0.0;
                
                // Generate new random platforms using current time + level for true randomness
                let random_seed = (std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64) 
                    + (game_state.level as u64 * 1000);
                    
                generate_random_platforms_with_seed(&mut commands, random_seed);
                
                // Spawn new fruit  
                setup_fruits_with_seed(commands, platform_query, random_seed + 42);
                break; // Only collect one fruit per frame
            }
        }
    }
}

fn update_ui(
    game_state: Res<GameState>,
    mut lives_query: Query<&mut Text, (With<LivesText>, Without<LevelText>)>,
    mut level_query: Query<&mut Text, (With<LevelText>, Without<LivesText>)>,
) {
    // Only update if the game state has changed
    if game_state.is_changed() {
        // Update lives text
        if let Ok(mut text) = lives_query.get_single_mut() {
            text.sections[0].value = format!("Lives: {}", game_state.lives);
        }

        // Update level text
        if let Ok(mut text) = level_query.get_single_mut() {
            text.sections[0].value = format!("Level: {}", game_state.level);
        }
    }
}

fn check_player_death(
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut commands: Commands,
    platform_query: Query<(Entity, &Transform), (With<Platform>, Without<Player>)>,
    fruit_query: Query<Entity, With<Fruit>>,
) {
    if let Ok((mut player_transform, mut velocity)) = player_query.get_single_mut() {
        // Check if player fell below screen (more generous threshold)
        if player_transform.translation.y < -WINDOW_HEIGHT / 2.0 {
            // Decrease lives
            if game_state.lives > 0 {
                game_state.lives -= 1;
            }
            
            // Reset player position
            player_transform.translation = Vec3::new(0.0, 200.0, 0.0);
            velocity.x = 0.0;
            velocity.y = 0.0;
            
            // If no lives left, reset the game
            if game_state.lives == 0 {
                // Reset game state
                game_state.lives = 3;
                game_state.level = 1;
                
                // Remove all platforms and fruits, then regenerate
                for (platform_entity, _) in platform_query.iter() {
                    commands.entity(platform_entity).despawn();
                }
                for fruit_entity in fruit_query.iter() {
                    commands.entity(fruit_entity).despawn();
                }
                
                // Generate new level with random layout
                let random_seed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64;
                    
                generate_random_platforms_with_seed(&mut commands, random_seed);
                setup_fruits_with_seed(commands, platform_query, random_seed + 42);
            }
        }
    }
}