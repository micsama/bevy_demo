use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;


/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

/// 计算圆周上的点。
///
/// 这个函数接收圆心坐标 `(m, n)`、半径 `r` 和圆周上要计算的点的数量 `n_points`。
/// 它返回一个 `Vec2` 结构体的向量，每个结构体代表圆周上的一个点。
/// 为了形成闭合的圆，第一个点在末尾重复出现。
///
/// # 参数
///
/// * `m` - 圆心的 x 坐标。
/// * `n` - 圆心的 y 坐标。
/// * `r` - 圆的半径。
/// * `n_points` - 圆周上要计算的点的数量。
///
/// # 返回值
///
/// 一个 `Vec<Vec2>`，其中每个 `Vec2` 都是圆周上的一个点。
///
/// # 示例
///
/// ```
/// let circle_points = get_circle_points(0.0, 0.0, 5.0, 8);
/// for point in circle_points {
///     println!("Point: ({}, {})", point.x, point.y);
/// }
/// ```
pub fn get_circle_points(m: f32, n: f32, r: f32, n_points: usize) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = (0..n_points)
        .map(|i| {
            // 将迭代索引转换为弧度角。
            // 角度与圆周上的位置成比例。
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (n_points as f32);

            // 使用角度和半径计算圆上点的 x 和 y 坐标，
            // 圆心位于 (m, n)。
            Vec2 {
                x: m + r * angle.cos(), // x 坐标
                y: n + r * angle.sin(), // y 坐标
            }
        })
        .collect();

    // 在向量末尾添加第一个点以闭合圆。
    if let Some(first_point) = points.first().cloned() {
        points.push(first_point);
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_two() {
        let circle_points = get_circle_points(0.0, 0.0, 300.0, 3);
        print!("-----\n");
        for point in circle_points {
            println!("Point: ({}, {})", point.x, point.y);
        }
        // assert_eq!(result, 4);
    }
}