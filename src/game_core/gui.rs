use bevy::prelude::*;

use bevy_egui::{
    egui::{self, RichText, TextEdit},
    EguiContexts,
};

use crate::{
    game_util::resources::{NetworkStuff, PlayerName},
    network::messages::ClientMessage,
    GameStage,
};

use super::player::{self, Enemy, Player};

pub fn score_board(
    mut contexts: EguiContexts,
    query_player: Query<&Player>,
    query_enemy: Query<&Enemy>,
    player_name: Res<PlayerName>,
) {
    let ctx = contexts.ctx_mut();

    let mut style = (*ctx.style()).clone();

    style.visuals.panel_fill = egui::Color32::TRANSPARENT;

    ctx.set_style(style);

    let mut score_list: Vec<(String, i32, egui::Color32)> = Vec::new();

    for player in query_player.iter() {
        if player_name.submitted {
            score_list.push((
                player_name.name.clone(),
                player.score.try_into().unwrap(),
                egui::Color32::GREEN,
            ));
        }
    }

    for enemy in query_enemy.iter() {
        if !enemy.name.is_empty() {
            score_list.push((
                enemy.name.to_string(),
                enemy.score.try_into().unwrap(),
                egui::Color32::RED,
            ));
        }
    }

    score_list.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    egui::Area::new("score_board")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctx, |ui| {
            for (id, score, color) in score_list {
                ui.label(RichText::new(format!(" {}: {}", id, score)).color(color));
                ui.add_space(5.0);
            }
        });
}

pub fn setup_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameStage>>,
    mut player_name: ResMut<PlayerName>,
    mut network_stuff: ResMut<NetworkStuff>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("satrunner")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Enter Name: ");
                ui.add(TextEdit::singleline(&mut player_name.name).char_limit(12));
                if ui.small_button("Play").clicked() && !player_name.name.is_empty() {
                    player_name.submitted = true;
                    match network_stuff
                        .write
                        .as_mut()
                        .unwrap()
                        .try_send(ClientMessage::PlayerName(player_name.name.clone()))
                    {
                        Ok(()) => {}
                        Err(e) => error!("Error sending message: {} CHANNEL FULL???", e),
                    };
                    next_state.set(GameStage::InGame);
                }
            });
        });
}