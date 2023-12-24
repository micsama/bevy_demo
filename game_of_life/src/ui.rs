use crate::{basic::*, setup_cells};
use bevy::prelude::*;
use bevy_egui::{egui,EguiContexts};

//重制按钮的方案
fn handle_reset_btn(gs: &mut ResMut<GameState>, ui_state: ResMut<UiState>) {
    gs.row_count = ui_state.row_count_input as usize;
    gs.density = ui_state.density_input as usize;
    let new_state = GameState::new(gs.density, gs.row_count);
    gs.active_cells = new_state.active_cells;
    gs.circles = new_state.circles;
    gs.paused = true;
    gs.reborn=ui_state.reborn_input;
}

/*处理开始游戏的按钮

*/
fn handle_start_btn(gs: &mut ResMut<GameState>) {
    gs.paused = !gs.paused;
}

pub fn handle_ui_events(
    mut egui_context:EguiContexts,
    mut gs: ResMut<GameState>,
    mut ui_state: ResMut<UiState>,
    q: Query<Entity, With<CellSet>>,
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
) {
    egui::TopBottomPanel::top("游戏控制器")
        .resizable(false)
        .min_height(UI_HEIGHT)
        .show(egui_context.ctx_mut(), |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.horizontal(|ui| {
                        let circles = gs.circles;
                        //控制细胞区域的总行数10～100
                        ui.label("rows");
                        ui.add(
                            egui::Slider::new(&mut ui_state.row_count_input, 10.0..=100.)
                                .step_by(10.),
                        );
                        ui.label("reborn");
                        ui.add(egui::Slider::new(&mut ui_state.reborn_input,0.0..=0.5).step_by(0.001)
                        );
                        //控制初始化时随机生成细胞的密度0～9
                        ui.label("density:");
                        ui.add(
                            egui::Slider::new(&mut ui_state.density_input, 0.0..=9.0).step_by(1.),
                        );
                        //控制游戏开始和暂停
                        if gs.paused {
                            if ui.button("Start").clicked() {
                                handle_start_btn(&mut gs);
                            }
                        } else if ui.button("Pause").clicked() {
                            handle_start_btn(&mut gs);
                        }
                        //重制游戏状态
                        if ui.button("Reset").clicked() {
                            handle_reset_btn(&mut gs, ui_state);
                            let entity = q.single();
                            cmd.entity(entity).despawn_recursive();
                            setup_cells(cmd, gs, windows);
                        }
                        ui.label(format!("circles:{}", circles))
                    });
                },
            );
        });
}
