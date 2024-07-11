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

fn coordinat_to_str(coordinat: Vec2) -> String {
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
    format!("{}-{}", coordinat_to_str(r.min), coordinat_to_str(r.max))
}

fn urect_to_str(r: URect) -> String {
    format!("({}, {})-({}, {})", r.min.x, r.min.y, r.max.x, r.max.y)
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
    //mut mouse_motion_event: EventReader<bevy::input::mouse::MouseMotion>,
    player_query: Query<(&Transform, &Player)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    //for _event in mouse_motion_event.read() {
    let (camera, _, camera_transform) = camera_query.single();
    let mut text = text_query.single_mut().0;
    let maybe_cursor_pos = window_query.single().cursor_position();
    let line_cursor_pos: String = if maybe_cursor_pos.is_some() {
        format!(
            "cursor_pos: {}",
            coordinat_to_str(maybe_cursor_pos.unwrap())
        )
        .to_string()
    } else {
        "cursor_pos: outside window".to_string()
    };
    // cursor_por in world
    let line_cursor_world_pos: String = if maybe_cursor_pos.is_some() {
        let cursor_world_pos = camera
            .viewport_to_world_2d(camera_transform, maybe_cursor_pos.unwrap())
            .unwrap();
        //text_transform.translation = Vec2::extend(cursor_world_pos, 0.0);
        format!("cursor_world_pos: {}", coordinat_to_str(cursor_world_pos)).to_string()
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
    //}
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
    //mut resize_reader: EventReader<bevy::window::WindowResized>,
    mut text_query: Query<(&mut Text, &Transform, &ResizeText)>,
    camera_query: Query<(&Camera, &OrthographicProjection)>,
) {
    let (mut text, _, _) = text_query.single_mut();
    //for e in resize_reader.read() {
    //let line_event = format!("event: {:.1} x {:.1}", e.width, e.height);
    let (camera, projection) = camera_query.single();

    let maybe_logical_viewport_rect = camera.logical_viewport_rect();
    let line_logical_viewport_rect: String = if maybe_logical_viewport_rect.is_some() {
        format!(
            "logical_viewport_rect: {}",
            rect_to_str(maybe_logical_viewport_rect.unwrap())
        )
    } else {
        "No logical_viewport_rect".to_string()
    };
    // physical denotes actual screen pixels
    let maybe_physical_viewport_rect = camera.physical_viewport_rect();
    let line_physical_viewport_rect: String = if maybe_physical_viewport_rect.is_some() {
        format!(
            "physical_viewport_rect: {}",
            urect_to_str(maybe_physical_viewport_rect.unwrap())
        )
    } else {
        "No physical_viewport_rect".to_string()
    };

    // projection
    let projection_area = projection.area;
    let line_projection_area = format!("projection_area: {}", rect_to_str(projection_area));
    text.sections[0].value = format!(
        "{}\n{}\n{}",
        line_logical_viewport_rect,
        line_physical_viewport_rect,
        line_projection_area,
    );
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

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
        assert_eq!(get_player_scale(&mut app), Vec2::new(1.0, 1.0));
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
}
