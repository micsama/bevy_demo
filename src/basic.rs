//定义一些游戏中的基础组件
use bevy::{prelude::{Resource,Color},time::Timer};
use bevy::prelude::Component;
use rand::Rng;

#[derive(Resource,Clone)]
pub struct  GameState{
    pub paused: bool, // 是否暂停
    pub row_count: usize, // 世界的宽度
    pub density: usize, // 1~9 设置为随机产生细胞分布的概率
    pub circles: i32 ,//细胞存活周期数
    pub active_cells:Vec<bool>,
}
//ui状态
#[derive(Resource)]
pub struct  UiState{
    pub row_count_input:f32,
    pub density_input:f32,
}

impl Default for UiState{
    fn default() -> Self {
        Self { row_count_input: 10., density_input: 3. }
    }
}

//计时器
#[derive(Resource)]
pub struct LifeTimer(pub Timer);

//初始化游戏状态
impl GameState{
    pub fn new(density:usize, row_count:usize)->Self {
        let mut arr=Vec::new();
        let mut rng= rand::thread_rng();
        for _ in 0..row_count*row_count{
            if rng.gen_range(0..10)<density{
                arr.push(true);
            }else{ arr.push(false)}
        }
        Self{
            paused:false,
            row_count,
            density,
            circles:0,
            active_cells:arr,
        }
    }
}

impl Default for GameState{
    fn default() -> Self {
        Self::new(5,990)
    }
}

//细胞ID
#[derive(Component)]
pub struct CellId(pub i32);
//细胞集合
#[derive(Component)]
pub struct  CellSet;


pub const ACTIVE_COLOR:Color=Color::rgb(0.2, 0.3, 1.);
pub const INIT_COLOR:Color=Color::rgb(0.2, 0.2, 0.2);
pub const UI_HEIGHT:f32 = 10.;
pub const WINDOW_INIT_HEIGHT:f32=1000.;
pub const WINDOW_INIT_WIDTH:f32=1800.;


