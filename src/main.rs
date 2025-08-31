use bevy::prelude::*;

// ----- Components for UI elements -----

#[derive(Component)]
struct DrakeParameter {
    name: String,
    value: f64,
    min: f64,
    max: f64,
}

// We’ll tag the actual span that holds the result string
#[derive(Component)]
struct ResultText;

// ----- Resource holding the Drake values -----

#[derive(Resource)]
struct DrakeValues {
    rate: f64,
    fp: f64,
    ne: f64,
    fl: f64,
    fi: f64,
    fc: f64,
    l: f64,
}

// ----- Drake equation -----

fn drake_equation(rate: f64, fp: f64, ne: f64, fl: f64, fi: f64, fc: f64, l: f64) -> f64 {
    rate * fp * ne * fl * fi * fc * l
}

// ----- Setup -----

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera (0.16+)
    commands.spawn(Camera2d);

    // Initial values
    commands.insert_resource(DrakeValues {
        rate: 1.0,
        fp: 0.5,
        ne: 0.1,
        fl: 0.1,
        fi: 0.01,
        fc: 0.01,
        l: 10000.0,
    });

    // Root UI node (Node replaces NodeBundle / Style in 0.16)
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            // ----- Title -----
            parent.spawn((
                Text::new("Drake Equation Simulator"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));

            // ----- Parameters -----
            let parameters = vec![
                ("R*", 1.0, 0.1, 10.0),
                ("f_p", 0.5, 0.0, 1.0),
                ("n_e", 0.1, 0.0, 1.0),
                ("f_l", 0.1, 0.0, 1.0),
                ("f_i", 0.01, 0.0, 1.0),
                ("f_c", 0.01, 0.0, 1.0),
                ("L", 10000.0, 100.0, 1_000_000.0),
            ];

            for (name, value, min, max) in parameters {
                // A row container (for future controls)
                parent
                    .spawn((
                        Node {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        DrakeParameter {
                            name: name.to_string(),
                            value,
                            min,
                            max,
                        },
                    ))
                    .with_children(|row| {
                        // Parameter label text
                        row.spawn((
                            Text::new(format!("{name}: {value:.2}")),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        ));
                    });
            }

            // ----- Result display -----
            // In 0.16 text content lives in a TextSpan child. We tag that child with ResultText.
            parent
                .spawn((
                    Text::new("N = "),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                ))
                .with_child((
                    TextSpan::new("0"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 0.0)),
                    ResultText,
                ));
        });
}

// ----- System: update the result text every frame -----

fn update_result(
    drake_values: Res<DrakeValues>,
    mut query: Query<&mut TextSpan, With<ResultText>>,
) {
    if !drake_values.is_changed() && query.is_empty() {
        return;
    }
    let result = drake_equation(
        drake_values.rate,
        drake_values.fp,
        drake_values.ne,
        drake_values.fl,
        drake_values.fi,
        drake_values.fc,
        drake_values.l,
    );

    for mut span in &mut query {
        // TextSpan derefs to String; assign new content
        **span = format!("{result:.2}");
    }
}

// ----- App entry point -----

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_result)
        .run();
}
