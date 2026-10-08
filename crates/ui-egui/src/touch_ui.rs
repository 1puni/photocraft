//! Opt-in touch sizing for alternate workspaces. Desktop geometry stays unchanged.
use egui::{Context, Id};

pub fn set_enabled(ctx: &Context, enabled: bool) {
    ctx.data_mut(|data| data.insert_temp(Id::new("pc-touch-ui"), enabled));
    if enabled {
        ctx.all_styles_mut(|style| {
            style.spacing.interact_size.y = 44.0;
            style.spacing.button_padding.y = 10.0;
            style.spacing.item_spacing.y = 8.0;
            style.spacing.combo_height = 320.0;
        });
    }
}

pub fn enabled(ctx: &Context) -> bool {
    ctx.data(|data| data.get_temp(Id::new("pc-touch-ui"))).unwrap_or(false)
}

pub fn height(ui: &egui::Ui, desktop: f32) -> f32 {
    if enabled(ui.ctx()) { desktop.max(44.0) } else { desktop }
}

#[cfg(test)]
mod tests {
    #[test]
    fn touch_is_opt_in_and_scoped_to_context() {
        let desktop = egui::Context::default();
        let tablet = egui::Context::default();
        super::set_enabled(&tablet, true);
        assert!(super::enabled(&tablet));
        assert!(!super::enabled(&desktop));
        assert!(tablet.style_of(egui::Theme::Dark).spacing.interact_size.y >= 44.0);
    }
}
