use bevy::prelude::*;

// Marker to find the Player sprite
#[derive(Component)]
pub struct Player;

// Marker to find the text at the cursor
#[derive(Component)]
pub struct CursorText;

// Marker to find the text that displays resizing
#[derive(Component)]
pub struct ResizeText;

#[derive(Component)]
pub struct IsInText;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
        app.add_plugins(bevy::prelude::WindowPlugin::default());
    }

    app.add_systems(Startup, (add_camera, add_player, add_text));
    app.add_systems(
        Update,
        (
            respond_to_keyboard,
            show_mouse_and_player_position,
            show_sizes,
            show_is_in,
        ),
    );

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

fn add_text(mut commands: Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(String::new(), TextStyle { color: Color::srgb(1.0, 0.0, 0.0), ..default() }),
            transform: Transform {
                translation: Vec3::new(-100.0, 300.0, 0.0),
                ..default()
            },
            ..default()
        },
        CursorText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(String::new(), TextStyle { color: Color::srgb(0.0, 1.0, 0.0), ..default() }),
            transform: Transform {
                translation: Vec3::new(-50.0, 100.0, 0.0),
                ..default()
            },
            ..default()
        },
        ResizeText,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(String::new(), TextStyle { color: Color::srgb(0.0, 0.0, 1.0), ..default() }),
            transform: Transform {
                translation: Vec3::new(-0.0, -100.0, 0.0),
                ..default()
            },
            ..default()
        },
        IsInText,
    ));
}

fn coordinate_to_str(coordinat: Vec2) -> String {
    format!("({}, {})", coordinat[0], coordinat[1])
}

fn ucoordinate_to_str(coordinat: UVec2) -> String {
    format!("({}, {})", coordinat[0], coordinat[1])
}

#[cfg(test)]
fn count_n_camers(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Camera>();
    return query.iter(app.world_mut()).len();
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world_mut()).len()
}

fn rect_to_str(r: Rect) -> String {
    format!("{}-{}", coordinate_to_str(r.min), coordinate_to_str(r.max))
}

fn urect_to_str(r: URect) -> String {
    format!("{}-{}", ucoordinate_to_str(r.min), ucoordinate_to_str(r.max))
}


#[cfg(test)]
fn get_camera_scale(app: &mut App) -> f32 {
    let mut query = app.world_mut().query::<&OrthographicProjection>();
    let projection = query.single(app.world());
    projection.scale
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

fn is_position_visible_in_projection_area(
    position: Vec2,
    projection: &OrthographicProjection,
) -> bool {
    projection.area.contains(position)
}

#[cfg(test)]
fn is_position_visible(app: &mut App, position: Vec2) -> bool {
    let mut camera_query = app.world_mut().query::<&OrthographicProjection>();
    let projection = camera_query.single(app.world());
    is_position_visible_in_projection_area(position, projection)
}

#[cfg(test)]
fn is_player_visible(app: &mut App) -> bool {
    let position = get_player_position(app);
    is_position_visible(app, position)
}

fn maybe_cursor_pos_to_str(maybe_cursor_pos: Option<Vec2>) -> String {
    if maybe_cursor_pos.is_some() {
        format!(
            "cursor_pos: {}",
            coordinate_to_str(maybe_cursor_pos.unwrap())
        ).to_string()
    } else {
        "cursor_pos: outside window".to_string()
    }
}

fn maybe_logical_viewport_rect_to_str(maybe_logical_viewport_rect: Option<Rect>) -> String {
    if maybe_logical_viewport_rect.is_some() {
        format!(
            "logical_viewport_rect: {}",
            rect_to_str(maybe_logical_viewport_rect.unwrap())
        )
    } else {
        "No logical_viewport_rect".to_string()
    }
}

fn maybe_physical_viewport_rect_to_str(maybe_physical_viewport_rect: Option<URect>) -> String {
    if maybe_physical_viewport_rect.is_some() {
        format!(
            "physical_viewport_rect: {}",
            urect_to_str(maybe_physical_viewport_rect.unwrap())
        )
    } else {
        "No physical_viewport_rect".to_string()
    }
}

fn respond_to_keyboard(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, _) = query.single_mut();
    use bevy::input::keyboard::KeyCode;
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 10.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 10.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 10.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 10.0;
    }
}

fn show_mouse_and_player_position(
    mut text_query: Query<(&mut Text, &CursorText)>,
    player_query: Query<(&Transform, &Player)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    let (camera, _, camera_transform) = camera_query.single();
    let mut text = text_query.single_mut().0;
    let maybe_cursor_pos = window_query.single().cursor_position();
    let line_cursor_pos = maybe_cursor_pos_to_str(maybe_cursor_pos);
    // cursor_pos in world
    let line_cursor_world_pos: String = if maybe_cursor_pos.is_some() {
        let cursor_world_pos = camera
            .viewport_to_world_2d(camera_transform, maybe_cursor_pos.unwrap())
            .unwrap();
        format!("cursor_world_pos: {}", coordinate_to_str(cursor_world_pos)).to_string()
    } else {
        "cursor_world_pos: outside window".to_string()
    };
    // player
    let player_pos = player_query.single().0.translation.xy();
    let line_player_pos: String = format!("player_pos: {}, {}", player_pos.x, player_pos.y);

    text.sections[0].value = format!(
        "{}\n{}\n{}",
        line_cursor_pos, line_cursor_world_pos, line_player_pos
    );
}

fn show_is_in(
    mut text_query: Query<(&mut Text, &Transform, &IsInText), Without<Player>>,
    player_query: Query<(&Transform, &Player)>,
    camera_query: Query<(&Camera, &OrthographicProjection)>,
) {
    let (mut text, _, _) = text_query.single_mut();
    let (_, projection) = camera_query.single();
    let player_pos = player_query.single().0.translation.xy();
    text.sections[0].value = format!(
        "is player visible: {}",
        is_position_visible_in_projection_area(player_pos, projection)
    );
}

fn show_sizes(
    mut text_query: Query<(&mut Text, &Transform, &ResizeText)>,
    camera_query: Query<(&Camera, &OrthographicProjection)>,
) {
    let (mut text, _, _) = text_query.single_mut();
    let (camera, projection) = camera_query.single();
    let maybe_logical_viewport_rect = camera.logical_viewport_rect();
    let maybe_physical_viewport_rect = camera.physical_viewport_rect();
    let projection_area = projection.area;
    text.sections[0].value = format!(
        "{}\n{}\n{}",
        maybe_logical_viewport_rect_to_str(maybe_logical_viewport_rect),
        maybe_physical_viewport_rect_to_str(maybe_physical_viewport_rect),
        format!("projection_area: {}", rect_to_str(projection_area)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_empty_app_has_no_camers() {
        let mut app = App::new();
        assert_eq!(count_n_camers(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_app_has_a_camera() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_camers(&mut app), 1);
    }

    #[test]
    fn test_get_camera_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_camera_scale(&mut app), 1.0);
    }

    #[test]
    fn test_player_is_visible_at_the_start() {
        let mut app = create_app();
        app.update();
        assert!(is_player_visible(&mut app));
    }

    #[test]
    fn test_origin_is_visible_at_the_start() {
        let mut app = create_app();
        app.update();
        assert!(is_position_visible(&mut app, Vec2::new(0.0,0.0)));
    }
    #[test]
    fn test_far_away_position_is_not_visible_at_the_start() {
        let mut app = create_app();
        app.update();
        assert!(!is_position_visible(&mut app, Vec2::new(10000000.0,0.0)));
    }

    #[test]
    fn test_coordinate_to_str() {
        assert_eq!(coordinate_to_str(Vec2::new(1.2, 3.4)), String::from("(1.2, 3.4)"))
    }
    #[test]
    fn test_ucoordinate_to_str() {
        assert_eq!(ucoordinate_to_str(UVec2::new(1, 2)), String::from("(1, 2)"))
    }

    #[test]
    fn test_maybe_cursor_pos_to_str() {
        let none = None;
        let some = Some(Vec2::new(0.0, 0.0));
        assert_ne!(maybe_cursor_pos_to_str(none), maybe_cursor_pos_to_str(some));
    }

    #[test]
    fn test_maybe_logical_viewport_rect_to_str() {
        let none = None;
        let some = Some(Rect::new(1.1, 2.2, 3.3, 4.4) );
        assert_ne!(maybe_logical_viewport_rect_to_str(none), maybe_logical_viewport_rect_to_str(some));
    }

    #[test]
    fn test_maybe_physical_viewport_rect_to_str() {
        let none = None;
        let some = Some(URect::new(1, 2, 3, 4) );
        assert_ne!(maybe_physical_viewport_rect_to_str(none), maybe_physical_viewport_rect_to_str(some));
    }



    #[test]
    fn test_player_responds_to_key_press_up() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the right arrow button, thanks Periwinkle
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);

        app.update();

        // Position must have changed now
        assert_ne!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_key_press_right() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the right arrow button, thanks Periwinkle
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);

        app.update();

        // Position must have changed now
        assert_ne!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_key_press_down() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the right arrow button, thanks Periwinkle
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowDown);

        app.update();

        // Position must have changed now
        assert_ne!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_key_press_left() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<bevy::input::InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));

        // Press the right arrow button, thanks Periwinkle
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);

        app.update();

        // Position must have changed now
        assert_ne!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }
}
