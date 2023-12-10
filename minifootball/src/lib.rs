use bevy::math::Vec2;

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
    let mut points: Vec<Vec2> = (0..n_points).map(|i| {
        // 将迭代索引转换为弧度角。
        // 角度与圆周上的位置成比例。
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (n_points as f32);

        // 使用角度和半径计算圆上点的 x 和 y 坐标，
        // 圆心位于 (m, n)。
        Vec2 {
            x: m + r * angle.cos(), // x 坐标
            y: n + r * angle.sin(), // y 坐标
        }
    }).collect();

    // 在向量末尾添加第一个点以闭合圆。
    if let Some(first_point) = points.first().cloned() {
        points.push(first_point);
    }

    points
}

