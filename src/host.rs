use crate::{
    params::{FrostVerbParams, PARAMETER_DEFS},
    preset::{FACTORY_PRESETS, FactoryPreset},
};

pub const PLUGIN_DESCRIPTOR: PluginDescriptor = PluginDescriptor {
    name: "Frost Verb",
    vendor: "Nursvendsp",
    bundle_id: "com.nursvendsp.frostverb",
    version: env!("CARGO_PKG_VERSION"),
    audio_inputs: 2,
    audio_outputs: 2,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluginDescriptor {
    pub name: &'static str,
    pub vendor: &'static str,
    pub bundle_id: &'static str,
    pub version: &'static str,
    pub audio_inputs: usize,
    pub audio_outputs: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HostParameterSnapshot {
    pub params: FrostVerbParams,
}

impl HostParameterSnapshot {
    pub fn new(params: FrostVerbParams) -> Self {
        Self {
            params: params.sanitized(),
        }
    }

    pub fn set_normalized(&mut self, id: &str, value: f32) -> Result<(), StateError> {
        let value = if value.is_finite() {
            value.clamp(0.0, 1.0)
        } else {
            0.0
        };
        match id {
            "coldness" => self.params.coldness = value,
            "wind" => self.params.wind = value,
            "ice_size" => self.params.ice_size = value,
            "ancient_depth" => self.params.ancient_depth = value,
            "frozen_harmonics" => self.params.frozen_harmonics = value,
            "storm" => self.params.storm = value,
            "distance" => self.params.distance = value,
            "decay" => self.params.decay = value,
            "freeze" => self.params.freeze = value,
            "mix" => self.params.mix = value,
            "input" => self.params.input = value,
            "output" => self.params.output = value,
            _ => return Err(StateError::UnknownParameter),
        }
        Ok(())
    }

    pub fn normalized(&self, id: &str) -> Option<f32> {
        Some(match id {
            "coldness" => self.params.coldness,
            "wind" => self.params.wind,
            "ice_size" => self.params.ice_size,
            "ancient_depth" => self.params.ancient_depth,
            "frozen_harmonics" => self.params.frozen_harmonics,
            "storm" => self.params.storm,
            "distance" => self.params.distance,
            "decay" => self.params.decay,
            "freeze" => self.params.freeze,
            "mix" => self.params.mix,
            "input" => self.params.input,
            "output" => self.params.output,
            _ => return None,
        })
    }

    pub fn encode_state(&self) -> String {
        let values = self.params.as_array();
        let mut state = String::from("frostverb-state-v1\n");
        for (param, value) in PARAMETER_DEFS.iter().zip(values) {
            state.push_str(param.id);
            state.push('=');
            state.push_str(&format!("{value:.8}"));
            state.push('\n');
        }
        state
    }

    pub fn decode_state(input: &str) -> Result<Self, StateError> {
        let mut snapshot = Self::new(FrostVerbParams::default());
        let mut saw_header = false;

        for line in input.lines() {
            if line == "frostverb-state-v1" {
                saw_header = true;
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            let (id, value) = line.split_once('=').ok_or(StateError::MalformedState)?;
            let value = value
                .parse::<f32>()
                .map_err(|_| StateError::MalformedState)?;
            snapshot.set_normalized(id, value)?;
        }

        if saw_header {
            Ok(snapshot)
        } else {
            Err(StateError::MalformedState)
        }
    }
}

impl Default for HostParameterSnapshot {
    fn default() -> Self {
        Self::new(FrostVerbParams::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateError {
    MalformedState,
    UnknownParameter,
}

pub fn factory_preset_by_name(name: &str) -> Option<&'static FactoryPreset> {
    FACTORY_PRESETS
        .iter()
        .find(|preset| preset.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trip_preserves_parameters() {
        let mut snapshot = HostParameterSnapshot::default();
        snapshot.set_normalized("coldness", 0.9).unwrap();
        snapshot.set_normalized("freeze", 0.35).unwrap();

        let encoded = snapshot.encode_state();
        let decoded = HostParameterSnapshot::decode_state(&encoded).unwrap();

        assert_eq!(snapshot, decoded);
    }

    #[test]
    fn preset_lookup_is_case_insensitive() {
        assert_eq!(
            factory_preset_by_name("whiteout").map(|preset| preset.name),
            Some("Whiteout")
        );
    }

    #[test]
    fn descriptor_matches_product_contract() {
        assert_eq!(PLUGIN_DESCRIPTOR.audio_inputs, 2);
        assert_eq!(PLUGIN_DESCRIPTOR.audio_outputs, 2);
        assert_eq!(PLUGIN_DESCRIPTOR.name, "Frost Verb");
    }
}
