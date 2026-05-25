//! Native egui UI scaffold for Frost Verb.
//!
//! This module intentionally keeps egui in charge of interaction and dynamic
//! text while allowing host wrappers to provide PNG texture handles for the
//! visual layers.

use egui::{
    Align2, Color32, ComboBox, Context, CornerRadius, FontId, Pos2, Rect, Response, Sense, Shape,
    Stroke, TextureId, Ui, UiBuilder, Vec2, pos2, vec2,
};

use crate::{
    dsp::EngineMeters,
    host::HostParameterSnapshot,
    params::{PARAMETER_DEFS, ParameterDef},
    preset::{FACTORY_PRESETS, FactoryPreset},
};

pub const DESIGN_SIZE: Vec2 = vec2(1000.0, 620.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtworkLayerKind {
    Base,
    StaticOverlay,
    Control,
    Reactive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArtworkLayer {
    pub name: &'static str,
    pub path: &'static str,
    pub kind: ArtworkLayerKind,
    pub rect: RectSpec,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectSpec {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectSpec {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    fn to_rect(self, panel: Rect) -> Rect {
        let sx = panel.width() / DESIGN_SIZE.x;
        let sy = panel.height() / DESIGN_SIZE.y;
        Rect::from_min_size(
            pos2(panel.left() + self.x * sx, panel.top() + self.y * sy),
            vec2(self.w * sx, self.h * sy),
        )
    }
}

pub const ARTWORK_LAYERS: [ArtworkLayer; 11] = [
    ArtworkLayer {
        name: "base-panel",
        path: "assets/ui/base_panel.png",
        kind: ArtworkLayerKind::Base,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "ice-structure",
        path: "assets/ui/ice_structure_overlay.png",
        kind: ArtworkLayerKind::StaticOverlay,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "large-knob-cap",
        path: "assets/ui/large_knob_cap.png",
        kind: ArtworkLayerKind::Control,
        rect: RectSpec::new(0.0, 0.0, 160.0, 160.0),
    },
    ArtworkLayer {
        name: "medium-knob-cap",
        path: "assets/ui/medium_knob_cap.png",
        kind: ArtworkLayerKind::Control,
        rect: RectSpec::new(0.0, 0.0, 118.0, 118.0),
    },
    ArtworkLayer {
        name: "small-knob-cap",
        path: "assets/ui/small_knob_cap.png",
        kind: ArtworkLayerKind::Control,
        rect: RectSpec::new(0.0, 0.0, 86.0, 86.0),
    },
    ArtworkLayer {
        name: "toggle-plate",
        path: "assets/ui/toggle_plate.png",
        kind: ArtworkLayerKind::Control,
        rect: RectSpec::new(0.0, 0.0, 118.0, 48.0),
    },
    ArtworkLayer {
        name: "freeze-glow",
        path: "assets/ui/freeze_glow.png",
        kind: ArtworkLayerKind::Reactive,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "storm-wind",
        path: "assets/ui/storm_wind.png",
        kind: ArtworkLayerKind::Reactive,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "coldness-ice",
        path: "assets/ui/coldness_ice.png",
        kind: ArtworkLayerKind::Reactive,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "meter-accent",
        path: "assets/ui/meter_accent.png",
        kind: ArtworkLayerKind::Reactive,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
    ArtworkLayer {
        name: "foreground-scratches",
        path: "assets/ui/foreground_scratches.png",
        kind: ArtworkLayerKind::StaticOverlay,
        rect: RectSpec::new(0.0, 0.0, 1000.0, 620.0),
    },
];

#[derive(Clone, Debug, Default)]
pub struct FrostVerbArtwork {
    textures: Vec<(&'static str, TextureId)>,
}

impl FrostVerbArtwork {
    pub fn set_texture(&mut self, layer_name: &'static str, texture_id: TextureId) {
        if let Some((_, existing)) = self
            .textures
            .iter_mut()
            .find(|(name, _)| *name == layer_name)
        {
            *existing = texture_id;
        } else {
            self.textures.push((layer_name, texture_id));
        }
    }

    pub fn texture(&self, layer_name: &str) -> Option<TextureId> {
        self.textures
            .iter()
            .find_map(|(name, texture)| (*name == layer_name).then_some(*texture))
    }
}

#[derive(Clone, Debug)]
pub struct FrostVerbGuiState {
    pub snapshot: HostParameterSnapshot,
    pub selected_preset: usize,
    pub artwork: FrostVerbArtwork,
}

impl Default for FrostVerbGuiState {
    fn default() -> Self {
        Self {
            snapshot: HostParameterSnapshot::default(),
            selected_preset: 0,
            artwork: FrostVerbArtwork::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterChange {
    pub id: &'static str,
    pub value: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrostVerbGuiOutput {
    pub parameter_changes: Vec<ParameterChange>,
    pub selected_preset: Option<&'static FactoryPreset>,
}

impl FrostVerbGuiState {
    pub fn show(&mut self, ctx: &Context, meters: EngineMeters) -> FrostVerbGuiOutput {
        let mut output = FrostVerbGuiOutput::default();
        #[allow(deprecated)]
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                apply_frostverb_style(ui);
                output = self.show_inside(ui, meters);
            });
        output
    }

    pub fn show_inside(&mut self, ui: &mut Ui, meters: EngineMeters) -> FrostVerbGuiOutput {
        apply_frostverb_style(ui);
        let available = ui.available_rect_before_wrap();
        let panel = fitted_panel_rect(available);
        let mut output = FrostVerbGuiOutput::default();

        draw_background(ui, panel);
        draw_artwork_layer(ui, panel, &self.artwork, "base-panel", 1.0);
        draw_reactive_overlays(ui, panel, &self.artwork, &self.snapshot, meters);
        draw_panel_lines(ui, panel);

        let mut show_knob = |spec: ControlSpec| {
            if let Some(change) = knob(ui, panel, spec, &mut self.snapshot, &self.artwork) {
                output.parameter_changes.push(change);
            }
        };

        for spec in CONTROL_SPECS {
            show_knob(spec);
        }

        if let Some(change) = freeze_toggle(ui, panel, &mut self.snapshot, &self.artwork) {
            output.parameter_changes.push(change);
        }
        if let Some(preset) = preset_selector(ui, panel, self) {
            output.selected_preset = Some(preset);
        }
        draw_meters(ui, panel, meters);
        draw_artwork_layer(ui, panel, &self.artwork, "foreground-scratches", 0.8);

        ui.allocate_rect(panel, Sense::hover());
        output
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KnobSize {
    Large,
    Medium,
    Small,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ControlSpec {
    id: &'static str,
    center: Pos2,
    size: KnobSize,
}

const CONTROL_SPECS: [ControlSpec; 11] = [
    ControlSpec {
        id: "ice_size",
        center: pos2(260.0, 210.0),
        size: KnobSize::Large,
    },
    ControlSpec {
        id: "ancient_depth",
        center: pos2(500.0, 188.0),
        size: KnobSize::Large,
    },
    ControlSpec {
        id: "coldness",
        center: pos2(740.0, 210.0),
        size: KnobSize::Large,
    },
    ControlSpec {
        id: "wind",
        center: pos2(170.0, 390.0),
        size: KnobSize::Medium,
    },
    ControlSpec {
        id: "frozen_harmonics",
        center: pos2(390.0, 405.0),
        size: KnobSize::Medium,
    },
    ControlSpec {
        id: "storm",
        center: pos2(610.0, 405.0),
        size: KnobSize::Medium,
    },
    ControlSpec {
        id: "distance",
        center: pos2(830.0, 390.0),
        size: KnobSize::Medium,
    },
    ControlSpec {
        id: "decay",
        center: pos2(500.0, 312.0),
        size: KnobSize::Small,
    },
    ControlSpec {
        id: "input",
        center: pos2(178.0, 545.0),
        size: KnobSize::Small,
    },
    ControlSpec {
        id: "mix",
        center: pos2(500.0, 545.0),
        size: KnobSize::Small,
    },
    ControlSpec {
        id: "output",
        center: pos2(822.0, 545.0),
        size: KnobSize::Small,
    },
];

fn fitted_panel_rect(available: Rect) -> Rect {
    let scale = (available.width() / DESIGN_SIZE.x)
        .min(available.height() / DESIGN_SIZE.y)
        .max(0.1);
    let size = DESIGN_SIZE * scale;
    Rect::from_center_size(available.center(), size)
}

fn scale_pos(panel: Rect, pos: Pos2) -> Pos2 {
    pos2(
        panel.left() + pos.x * panel.width() / DESIGN_SIZE.x,
        panel.top() + pos.y * panel.height() / DESIGN_SIZE.y,
    )
}

fn scale_len(panel: Rect, value: f32) -> f32 {
    value * panel.width() / DESIGN_SIZE.x
}

fn parameter(id: &str) -> &'static ParameterDef {
    PARAMETER_DEFS
        .iter()
        .find(|param| param.id == id)
        .expect("UI control references a public parameter")
}

fn knob(
    ui: &mut Ui,
    panel: Rect,
    spec: ControlSpec,
    snapshot: &mut HostParameterSnapshot,
    artwork: &FrostVerbArtwork,
) -> Option<ParameterChange> {
    let def = parameter(spec.id);
    let center = scale_pos(panel, spec.center);
    let radius = match spec.size {
        KnobSize::Large => scale_len(panel, 76.0),
        KnobSize::Medium => scale_len(panel, 56.0),
        KnobSize::Small => scale_len(panel, 40.0),
    };
    let rect = Rect::from_center_size(center, Vec2::splat(radius * 2.0));
    let response = ui.allocate_rect(rect, Sense::click_and_drag());
    let before = snapshot
        .normalized(def.id)
        .unwrap_or(def.normalized_default());
    let value = apply_knob_input(ui, &response, before);
    let changed = (value - before).abs() > f32::EPSILON;
    if changed {
        let _ = snapshot.set_normalized(def.id, value);
    }

    draw_knob(
        ui,
        rect,
        def,
        value,
        response.hovered(),
        response.has_focus(),
        artwork,
        spec.size,
    );
    response.on_hover_text(format!("{} {:.0}%", def.name, value * 100.0));

    changed.then_some(ParameterChange { id: def.id, value })
}

fn apply_knob_input(ui: &Ui, response: &Response, value: f32) -> f32 {
    let mut next = value;
    if response.dragged() {
        let delta = response.drag_delta();
        next = (value + (delta.x - delta.y) * 0.0025).clamp(0.0, 1.0);
    }
    if response.hovered() {
        ui.input(|input| {
            let scroll = input.smooth_scroll_delta().y;
            if scroll.abs() > 0.0 {
                next = (next + scroll * 0.0015).clamp(0.0, 1.0);
            }
        });
    }
    if response.double_clicked() {
        next = 0.5;
    }
    next
}

fn draw_knob(
    ui: &Ui,
    rect: Rect,
    def: &ParameterDef,
    value: f32,
    hovered: bool,
    focused: bool,
    artwork: &FrostVerbArtwork,
    size: KnobSize,
) {
    let painter = ui.painter();
    let center = rect.center();
    let radius = rect.width() * 0.5;
    let stroke = Stroke::new(1.5, Color32::from_rgb(129, 196, 214));
    painter.circle_filled(center, radius, Color32::from_rgb(12, 24, 31));
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(2.0, Color32::from_rgb(41, 81, 92)),
    );
    painter.circle_stroke(center, radius * 0.84, stroke);

    if let Some(texture) = artwork.texture(match size {
        KnobSize::Large => "large-knob-cap",
        KnobSize::Medium => "medium-knob-cap",
        KnobSize::Small => "small-knob-cap",
    }) {
        painter.image(
            texture,
            rect,
            Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    } else {
        painter.circle_filled(center, radius * 0.72, Color32::from_rgb(33, 58, 68));
        painter.circle_stroke(
            center,
            radius * 0.72,
            Stroke::new(1.0, Color32::from_rgb(214, 232, 228)),
        );
    }

    let start = -2.35_f32;
    let end = 2.35_f32;
    let angle = start + (end - start) * value;
    let tip = center + vec2(angle.cos(), angle.sin()) * radius * 0.56;
    painter.line_segment(
        [center, tip],
        Stroke::new(3.0, Color32::from_rgb(235, 248, 244)),
    );
    draw_arc(
        painter,
        center,
        radius * 0.93,
        start,
        angle,
        Stroke::new(3.5, Color32::from_rgb(105, 220, 222)),
    );
    if hovered || focused {
        painter.circle_stroke(
            center,
            radius + 3.0,
            Stroke::new(2.0, Color32::from_rgb(226, 251, 249)),
        );
    }

    painter.text(
        pos2(center.x, rect.bottom() + 8.0),
        Align2::CENTER_TOP,
        def.name,
        FontId::proportional(label_size(size)),
        Color32::from_rgb(220, 235, 232),
    );
    painter.text(
        pos2(center.x, rect.bottom() + 30.0),
        Align2::CENTER_TOP,
        format!("{:.0}%", value * 100.0),
        FontId::monospace(value_size(size)),
        Color32::from_rgb(144, 188, 194),
    );
}

fn draw_arc(
    painter: &egui::Painter,
    center: Pos2,
    radius: f32,
    start: f32,
    end: f32,
    stroke: Stroke,
) {
    let steps = 36;
    let points = (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            let angle = start + (end - start) * t;
            center + vec2(angle.cos(), angle.sin()) * radius
        })
        .collect::<Vec<_>>();
    painter.add(Shape::line(points, stroke));
}

fn label_size(size: KnobSize) -> f32 {
    match size {
        KnobSize::Large => 16.0,
        KnobSize::Medium => 14.0,
        KnobSize::Small => 12.0,
    }
}

fn value_size(size: KnobSize) -> f32 {
    match size {
        KnobSize::Large => 13.0,
        KnobSize::Medium => 12.0,
        KnobSize::Small => 11.0,
    }
}

fn freeze_toggle(
    ui: &mut Ui,
    panel: Rect,
    snapshot: &mut HostParameterSnapshot,
    artwork: &FrostVerbArtwork,
) -> Option<ParameterChange> {
    let rect = RectSpec::new(624.0, 521.0, 118.0, 48.0).to_rect(panel);
    let response = ui.allocate_rect(rect, Sense::click());
    let before = snapshot.normalized("freeze").unwrap_or(0.0);
    let mut value = before;
    if response.clicked() {
        value = if before >= 0.5 { 0.0 } else { 1.0 };
        let _ = snapshot.set_normalized("freeze", value);
    }

    let painter = ui.painter();
    if let Some(texture) = artwork.texture("toggle-plate") {
        painter.image(
            texture,
            rect,
            Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    } else {
        painter.rect_filled(rect, 6.0, Color32::from_rgb(10, 22, 29));
        painter.rect_stroke(
            rect,
            6.0,
            Stroke::new(1.5, Color32::from_rgb(117, 183, 198)),
            egui::StrokeKind::Middle,
        );
    }
    let active = value >= 0.5;
    painter.circle_filled(
        pos2(
            if active {
                rect.right() - rect.height() * 0.5
            } else {
                rect.left() + rect.height() * 0.5
            },
            rect.center().y,
        ),
        rect.height() * 0.30,
        if active {
            Color32::from_rgb(203, 246, 242)
        } else {
            Color32::from_rgb(73, 102, 110)
        },
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        "Freeze",
        FontId::proportional(13.0),
        Color32::from_rgb(223, 238, 236),
    );
    response.on_hover_text("Freeze hold");

    (value - before)
        .abs()
        .gt(&f32::EPSILON)
        .then_some(ParameterChange {
            id: "freeze",
            value,
        })
}

fn preset_selector(
    ui: &mut Ui,
    panel: Rect,
    state: &mut FrostVerbGuiState,
) -> Option<&'static FactoryPreset> {
    let rect = RectSpec::new(352.0, 28.0, 296.0, 36.0).to_rect(panel);
    let mut changed = None;
    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
        apply_preset_style(ui);
        ui.set_min_width(rect.width());
        ComboBox::from_id_salt("frostverb-preset-selector")
            .selected_text(FACTORY_PRESETS[state.selected_preset].name)
            .show_ui(ui, |ui| {
                for (index, preset) in FACTORY_PRESETS.iter().enumerate() {
                    if ui
                        .selectable_value(&mut state.selected_preset, index, preset.name)
                        .changed()
                    {
                        state.snapshot = HostParameterSnapshot::new(preset.params);
                        changed = Some(preset);
                    }
                }
            });
    });
    changed
}

fn draw_background(ui: &Ui, panel: Rect) {
    ui.painter()
        .rect_filled(panel, 0.0, Color32::from_rgb(10, 17, 22));
    ui.painter()
        .rect_filled(panel.shrink(10.0), 8.0, Color32::from_rgb(20, 35, 42));
}

fn draw_panel_lines(ui: &Ui, panel: Rect) {
    let painter = ui.painter();
    painter.rect_stroke(
        panel.shrink(10.0),
        8.0,
        Stroke::new(1.0, Color32::from_rgb(105, 145, 153)),
        egui::StrokeKind::Middle,
    );
    painter.line_segment(
        [
            scale_pos(panel, pos2(70.0, 490.0)),
            scale_pos(panel, pos2(930.0, 490.0)),
        ],
        Stroke::new(1.0, Color32::from_rgb(70, 107, 116)),
    );
    painter.text(
        scale_pos(panel, pos2(36.0, 36.0)),
        Align2::LEFT_CENTER,
        "Frost Verb",
        FontId::proportional(22.0),
        Color32::from_rgb(232, 244, 241),
    );
}

fn draw_artwork_layer(
    ui: &Ui,
    panel: Rect,
    artwork: &FrostVerbArtwork,
    layer_name: &str,
    alpha: f32,
) {
    if let Some(layer) = ARTWORK_LAYERS.iter().find(|layer| layer.name == layer_name) {
        if let Some(texture) = artwork.texture(layer.name) {
            ui.painter().image(
                texture,
                layer.rect.to_rect(panel),
                Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
                Color32::from_white_alpha((alpha.clamp(0.0, 1.0) * 255.0) as u8),
            );
        }
    }
}

fn draw_reactive_overlays(
    ui: &Ui,
    panel: Rect,
    artwork: &FrostVerbArtwork,
    snapshot: &HostParameterSnapshot,
    meters: EngineMeters,
) {
    draw_artwork_layer(
        ui,
        panel,
        artwork,
        "freeze-glow",
        snapshot.normalized("freeze").unwrap_or(0.0),
    );
    let storm_wind = (snapshot.normalized("storm").unwrap_or(0.0)
        + snapshot.normalized("wind").unwrap_or(0.0))
        * 0.5;
    draw_artwork_layer(ui, panel, artwork, "storm-wind", storm_wind);
    draw_artwork_layer(
        ui,
        panel,
        artwork,
        "coldness-ice",
        snapshot.normalized("coldness").unwrap_or(0.0),
    );
    draw_artwork_layer(
        ui,
        panel,
        artwork,
        "meter-accent",
        meters.output_peak.clamp(0.0, 1.0),
    );
}

fn draw_meters(ui: &Ui, panel: Rect, meters: EngineMeters) {
    let painter = ui.painter();
    let specs = [
        (
            "IN",
            meters.input_peak,
            RectSpec::new(54.0, 116.0, 18.0, 342.0),
        ),
        (
            "WET",
            meters.wet_peak,
            RectSpec::new(928.0, 116.0, 18.0, 342.0),
        ),
        (
            "OUT",
            meters.output_peak,
            RectSpec::new(954.0, 116.0, 18.0, 342.0),
        ),
    ];
    for (label, value, spec) in specs {
        let rect = spec.to_rect(panel);
        painter.rect_filled(rect, 4.0, Color32::from_rgb(9, 19, 25));
        let fill_height = rect.height() * value.clamp(0.0, 1.0);
        let fill = Rect::from_min_max(pos2(rect.left(), rect.bottom() - fill_height), rect.max);
        painter.rect_filled(fill, 4.0, Color32::from_rgb(132, 217, 218));
        painter.text(
            pos2(rect.center().x, rect.bottom() + 8.0),
            Align2::CENTER_TOP,
            label,
            FontId::monospace(10.0),
            Color32::from_rgb(151, 189, 193),
        );
    }
}

fn apply_frostverb_style(ui: &mut Ui) {
    ui.spacing_mut().item_spacing = vec2(8.0, 6.0);
    ui.spacing_mut().button_padding = vec2(10.0, 5.0);
    ui.spacing_mut().combo_width = 260.0;

    let visuals = ui.visuals_mut();
    visuals.override_text_color = Some(Color32::from_rgb(222, 239, 236));
    visuals.panel_fill = Color32::from_rgb(9, 17, 22);
    visuals.faint_bg_color = Color32::from_rgb(12, 26, 34);
    visuals.extreme_bg_color = Color32::from_rgb(6, 13, 17);
    visuals.selection.bg_fill = Color32::from_rgb(54, 116, 128);
    visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(191, 242, 240));
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(13, 26, 33);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(172, 205, 206));
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(12, 27, 35);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(70, 120, 132));
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(221, 239, 237));
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(21, 48, 59);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, Color32::from_rgb(144, 226, 226));
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::from_rgb(237, 250, 247));
    visuals.widgets.active.bg_fill = Color32::from_rgb(39, 87, 101);
    visuals.widgets.active.bg_stroke = Stroke::new(1.5, Color32::from_rgb(204, 249, 246));
    visuals.widgets.open.bg_fill = Color32::from_rgb(14, 32, 41);
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, Color32::from_rgb(95, 164, 178));
}

fn apply_preset_style(ui: &mut Ui) {
    ui.spacing_mut().interact_size.y = 30.0;
    let visuals = ui.visuals_mut();
    visuals.window_fill = Color32::from_rgb(8, 18, 24);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(91, 153, 166));
    visuals.menu_corner_radius = CornerRadius::same(6);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(6);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(6);
    visuals.widgets.active.corner_radius = CornerRadius::same(6);
    visuals.widgets.open.corner_radius = CornerRadius::same(6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_control_targets_a_public_parameter() {
        for control in CONTROL_SPECS {
            assert!(PARAMETER_DEFS.iter().any(|param| param.id == control.id));
        }
    }

    #[test]
    fn primary_layout_matches_product_plan() {
        let ids = CONTROL_SPECS
            .iter()
            .map(|control| control.id)
            .collect::<Vec<_>>();
        assert!(ids.starts_with(&["ice_size", "ancient_depth", "coldness"]));
        assert!(ids.contains(&"wind"));
        assert!(ids.contains(&"frozen_harmonics"));
        assert!(ids.contains(&"storm"));
        assert!(ids.contains(&"distance"));
        assert!(ids.contains(&"input"));
        assert!(ids.contains(&"output"));
        assert!(ids.contains(&"mix"));
    }

    #[test]
    fn artwork_manifest_has_no_baked_label_assets() {
        for layer in ARTWORK_LAYERS {
            assert!(!layer.path.contains("label"));
            assert!(!layer.path.contains("text"));
            assert!(!layer.path.contains("value"));
        }
    }
}
