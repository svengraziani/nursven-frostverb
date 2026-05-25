use crate::params::FrostVerbParams;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FactoryPreset {
    pub name: &'static str,
    pub category: &'static str,
    pub params: FrostVerbParams,
}

impl FactoryPreset {
    pub fn to_json_object(self) -> String {
        let p = self.params.sanitized();
        format!(
            concat!(
                "{{\n",
                "  \"name\": \"{}\",\n",
                "  \"category\": \"{}\",\n",
                "  \"params\": {{\n",
                "    \"coldness\": {:.6},\n",
                "    \"wind\": {:.6},\n",
                "    \"ice_size\": {:.6},\n",
                "    \"ancient_depth\": {:.6},\n",
                "    \"frozen_harmonics\": {:.6},\n",
                "    \"storm\": {:.6},\n",
                "    \"distance\": {:.6},\n",
                "    \"decay\": {:.6},\n",
                "    \"freeze\": {:.6},\n",
                "    \"mix\": {:.6},\n",
                "    \"input\": {:.6},\n",
                "    \"output\": {:.6}\n",
                "  }}\n",
                "}}"
            ),
            self.name,
            self.category,
            p.coldness,
            p.wind,
            p.ice_size,
            p.ancient_depth,
            p.frozen_harmonics,
            p.storm,
            p.distance,
            p.decay,
            p.freeze,
            p.mix,
            p.input,
            p.output
        )
    }
}

pub fn factory_presets_json() -> String {
    let mut json = String::from("[\n");
    for (index, preset) in FACTORY_PRESETS.iter().enumerate() {
        json.push_str(&indent(&preset.to_json_object(), 2));
        if index + 1 != FACTORY_PRESETS.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push(']');
    json
}

fn indent(input: &str, spaces: usize) -> String {
    let prefix = " ".repeat(spaces);
    input
        .lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub const FACTORY_PRESETS: [FactoryPreset; 8] = [
    preset(
        "Frozen Cave",
        "Frozen Cave",
        0.72,
        0.14,
        0.78,
        0.72,
        0.32,
        0.20,
        0.55,
        0.76,
        0.0,
        0.42,
    ),
    preset(
        "Fjord Distance",
        "Fjord Distance",
        0.58,
        0.22,
        0.68,
        0.61,
        0.20,
        0.18,
        0.86,
        0.64,
        0.0,
        0.36,
    ),
    preset(
        "Ice Plate",
        "Ice Plate",
        0.88,
        0.06,
        0.36,
        0.24,
        0.54,
        0.16,
        0.22,
        0.38,
        0.0,
        0.31,
    ),
    preset(
        "Nordic Temple",
        "Nordic Temple",
        0.64,
        0.10,
        0.82,
        0.88,
        0.42,
        0.12,
        0.48,
        0.84,
        0.0,
        0.45,
    ),
    preset(
        "Whiteout", "Whiteout", 0.82, 0.74, 0.61, 0.52, 0.28, 0.82, 0.78, 0.58, 0.18, 0.40,
    ),
    preset(
        "Distant Horn",
        "Distant Horn",
        0.52,
        0.16,
        0.73,
        0.66,
        0.36,
        0.22,
        0.92,
        0.70,
        0.0,
        0.48,
    ),
    preset(
        "Crystal Tunnel",
        "Crystal Tunnel",
        0.92,
        0.08,
        0.58,
        0.44,
        0.82,
        0.24,
        0.50,
        0.62,
        0.0,
        0.37,
    ),
    preset(
        "Dead Snowfield",
        "Dead Snowfield",
        0.76,
        0.42,
        0.47,
        0.38,
        0.16,
        0.36,
        0.90,
        0.44,
        0.08,
        0.32,
    ),
];

const fn preset(
    name: &'static str,
    category: &'static str,
    coldness: f32,
    wind: f32,
    ice_size: f32,
    ancient_depth: f32,
    frozen_harmonics: f32,
    storm: f32,
    distance: f32,
    decay: f32,
    freeze: f32,
    mix: f32,
) -> FactoryPreset {
    FactoryPreset {
        name,
        category,
        params: FrostVerbParams {
            coldness,
            wind,
            ice_size,
            ancient_depth,
            frozen_harmonics,
            storm,
            distance,
            decay,
            freeze,
            mix,
            input: 0.75,
            output: 0.75,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::PARAMETER_DEFS;

    #[test]
    fn json_export_contains_all_parameter_ids() {
        let json = factory_presets_json();
        for param in PARAMETER_DEFS {
            assert!(json.contains(param.id), "missing {}", param.id);
        }
        for preset in FACTORY_PRESETS {
            assert!(json.contains(preset.name));
        }
    }
}
