pub mod basic;
use bevy::diagnostic::LogDiagnosticsPlugin;
use basic::*;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(LifeTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .init_resource::<GameState>()
        .init_resource::<UiState>()
        .add_systems(Startup, (setup, setup_cells))
        .add_systems(Update, (draw_cells, upadte_cells_status))
        .run();
}

/**
 *  游戏初始化
 */
fn setup(mut cmd: Commands, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.resolution.set(WINDOW_INIT_WIDTH, WINDOW_INIT_HEIGHT);
    cmd.spawn(Camera2dBundle::default());
}

/**
 * 初始化细胞区域？为什么放在游戏初始化的一个函数而不是作为一个system呢
 */
fn setup_cells(mut cmd: Commands, game_state: ResMut<GameState>, mut windows: Query<&mut Window>) {
    let window = windows.single_mut();
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.1, 0.1),
                ..default()
            },
            ..default()
        },
        CellSet,
    ))
    .with_children(|parent| {
        let padding = 40.;
        let height = window.height() - UI_HEIGHT;
        let total_width = if window.width() > height {
            height - padding
        } else {
            window.width() - padding
        };
        let width = total_width / game_state.row_count as f32;
        //计算整体偏移量，尽量居中
        let delta_x = -total_width / 2. + width / 2.;
        let delta_y = -total_width / 2. + width / 2. - UI_HEIGHT / 2.;
        for i in 0..game_state.row_count as i32 {
            for j in 0..game_state.row_count as i32 {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: INIT_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                delta_x + width * j as f32,
                                delta_y + width * i as f32,
                                0.,
                            ),
                            scale: Vec3::new(width * 0.9, width * 0.9, 1.),
                            ..default()
                        },
                        ..default()
                    },
                    CellId(i * game_state.row_count as i32 + j),
                ));
            }
        }
    });
}

fn draw_cells(mut q: Query<(&CellId, &mut Sprite), With<CellId>>, game_state: ResMut<GameState>) {
    for (cellid, mut sprite) in &mut q {
        if game_state.active_cells[cellid.0 as usize] {
            sprite.color = ACTIVE_COLOR;
        } else {
            sprite.color = INIT_COLOR;
        }
    }
}

/////////////--------systems--------////////////////

/**
 * 更新细胞的状态
 * 细胞过少：如果一个活细胞少于两个活的邻居，它就会死亡。
 * oxo    oxo
 * oxo -> ooo
 * ooo    ooo
 * 正常：一个有两个或三个活邻居的活细胞可以延续到下一代。
 * xxo    xxo
 * oxo -> oxo
 * ooo    ooo
 * 细胞过多，过度竞争：一个有超过三个活邻居的活细胞死亡。
 * xox    xox
 * oxo -> oxo
 * xoo    xoo
 * 繁衍：如果一个死细胞正好有三个活着的邻居，它就会复活。
 * xox    xox
 * ooo -> oxo
 * xoo    xoo
 */
fn upadte_cells_status(
    time: Res<Time>,
    mut timer: ResMut<LifeTimer>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.paused {
        return;
    }
    if timer.0.tick(time.delta()).just_finished() {
        let mut next_circl = Vec::new();
        let mut alives = 0;
        for (index, cell_status) in game_state.active_cells.iter().enumerate() {
            let rows = index / game_state.row_count;
            let colums = index % game_state.row_count;
            let mut arr = Vec::new();
            if rows >= 1 {
                //如果上面还有其他行，则把上一个的状态加上去
                arr.push(index - game_state.row_count);
                if colums >= 1 {
                    arr.push(index - 1);
                    arr.push(index - 1 - game_state.row_count);
                }
                if colums < game_state.row_count - 1 {
                    arr.push(index + 1);
                    arr.push(index + 1 - game_state.row_count);
                }
            }
            if rows < game_state.row_count - 1 {
                arr.push(index + game_state.row_count);
                if colums >= 1 {
                    arr.push(index - 1 + game_state.row_count);
                }
                if colums < game_state.row_count - 1 {
                    arr.push(index + 1 + game_state.row_count);
                }
            }
            let mut active_cells = 0;
            for i in arr.iter() {
                if game_state.active_cells[*i] {
                    active_cells += 1;
                }
            }
            if *cell_status {
                if active_cells <= 1 || active_cells >= 4 {
                    next_circl.push(false);
                } else {
                    next_circl.push(true);
                    alives += 1;
                }
            } else if active_cells == 3 {
                next_circl.push(true);
                alives += 1;
            } else {
                next_circl.push(false)
            }
        }
        if alives == 0 {
            game_state.paused = true;
        }
        game_state.active_cells = next_circl;
        game_state.circles += 1;
    }
}
