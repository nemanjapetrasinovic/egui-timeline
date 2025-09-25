use egui::{Rect, Vec2, Color32, Sense, Widget, Ui,
    FontId, Response, Pos2, LayerId, Order, Align2,
    Rounding, vec2
};

pub struct Timeline<'a> {
    progress: f64,
    total: f64,
    accent_color: Color32,
    height: f32,
    radius_factor: f32,
    font_size: f32,
    seek_position: &'a mut f64,
}

impl<'a> Timeline<'a> {
    pub fn new(progress: f64, total: f64, seek_position: &'a mut f64) -> Self {
        Self {
            progress,
            total,
            accent_color: Color32::from_rgb(0, 155, 255),
            height: 8.0,
            radius_factor: 0.3,
            font_size: 12.0,
            seek_position,
        }
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = color;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn radius_factor(mut self, factor: f32) -> Self {
        self.radius_factor = factor;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
}

impl<'a> Widget for Timeline<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let font_id = FontId::new(self.font_size, egui::FontFamily::Proportional);
        let time_font_width = ui.ctx().fonts(|fonts| {
            fonts.layout_no_wrap(time_to_display(0.0), font_id, egui::Color32::WHITE).rect.width()
        });

        let desired_size_x = ui.available_size().x;
        let desired_size_y = self.height;
        let desired_size: Vec2 = vec2(desired_size_x, desired_size_y);

        let (rect, mut response) = 
            ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        let visuals = ui.style().interact(&response);

        let mut outer_rect = rect.expand(visuals.expansion);
        outer_rect.set_left(rect.left() + time_font_width + 5.0);
        outer_rect.set_right(rect.right() - time_font_width - 5.0);

        if ui.is_rect_visible(rect) {
            let radius = self.radius_factor * rect.height();
            ui.painter()
                .rect(outer_rect, radius, visuals.bg_fill, visuals.bg_stroke);

            if response.hovered() {
                if let Some(mouse_position) = ui.input(|i| i.pointer.hover_pos()) {
                    let seek_time;
                    if mouse_position.x < outer_rect.min.x {
                        seek_time = 0.0;
                    } else if mouse_position.x > outer_rect.max.x {
                        seek_time = self.total;
                    } else {
                        seek_time = self.total * (mouse_position.x - outer_rect.min.x) as f64 / outer_rect.width() as f64;
                    }

                    draw_tooltip(
                        ui,
                        Pos2::new(mouse_position.x, outer_rect.min.y - self.font_size - 10.0),
                        time_to_display(seek_time),
                        visuals.text_color(),
                        visuals.bg_fill,
                        self.font_size
                    );

                }
            }

            let mut fill_rect = outer_rect;
            let mut seek_rect = outer_rect;

            fill_rect.set_width(fill_rect.width() * self.progress as f32 / self.total as f32);
            ui.painter().rect_filled(
                fill_rect,
                radius,
                self.accent_color
            );

            if response.is_pointer_button_down_on() || response.dragged() {
                if let Some(pt) = response.interact_pointer_pos() {
                    seek_rect.max.x = pt.x;
                    if seek_rect.width() > outer_rect.width() {
                        seek_rect.set_width(outer_rect.width());
                    }

                    let seek_color = {
                        let [r, g, b, _] = self.accent_color.to_array();
                        Color32::from_rgba_unmultiplied(
                            ((r as f32 * 1.1).min(255.0)) as u8,
                            ((g as f32 * 1.1).min(255.0)) as u8,
                            ((b as f32 * 1.1).min(255.0)) as u8,
                            128,
                        )
                    };

                    ui.painter().rect_filled(seek_rect, radius, seek_color);

                    if pt.x < seek_rect.min.x {
                        *self.seek_position = 0.0;
                    } else if pt.x > rect.max.x {
                        *self.seek_position = self.total;
                    } else {
                        *self.seek_position =
                            self.total * seek_rect.width() as f64 / outer_rect.width() as f64;
                    }

                    response.mark_changed();
                }
            }
        }

        ui.painter().text(
            rect.left_top() + Vec2::new(0.0 , self.height / 2.0 - self.font_size / 2.0),
            Align2::LEFT_TOP,
            time_to_display(self.progress),
            FontId::proportional(self.font_size),
            visuals.text_color(),
        );

        ui.painter().text(
            rect.right_top() + Vec2::new(-time_font_width, self.height / 2.0 - self.font_size / 2.0),
            Align2::LEFT_TOP,
            time_to_display(self.total),
            FontId::proportional(self.font_size),
            visuals.text_color(),
        );

        response
    }
}

fn time_to_display(seconds: f64) -> String {
    let is: i64 = seconds.round() as i64;
    let hours = is / (60 * 60);
    let mins = (is % (60 * 60)) / 60;
    let secs = seconds - 60.0 * mins as f64 - 60.0 * 60.0 * hours as f64; // is % 60;

    format!("{}:{:0>2}:{:0>4.1}", hours, mins, secs)
}

fn draw_tooltip(
    ui: &Ui,
    pos: Pos2,
    tooltip_text: impl ToString,
    text_color: Color32,
    tooltip_color: Color32,
    font_size: f32
) {
    let layer_id = LayerId::new(Order::Foreground, ui.id().with("foreground_layer"));
    let foreground_painter = ui.ctx().layer_painter(layer_id);

    let font_id = FontId::new(font_size, egui::FontFamily::Proportional);
    let tooltip_font_width = ui.ctx().fonts(|fonts| {
        fonts.layout_no_wrap(tooltip_text.to_string(), font_id, egui::Color32::WHITE).rect.width()
    });

    let rect = Rect::from_min_size(pos, vec2(tooltip_font_width + 8.0, font_size + 6.0));
    let rounding = Rounding::same(5.0);

    foreground_painter.rect_filled(rect, rounding, tooltip_color);

    foreground_painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        tooltip_text,
        FontId::proportional(font_size),
        text_color,
    );
}
