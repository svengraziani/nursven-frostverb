#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterDef {
    pub id: &'static str,
    pub name: &'static str,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub unit: &'static str,
    pub automatable: bool,
}

impl ParameterDef {
    pub const fn normalized_default(self) -> f32 {
        (self.default - self.min) / (self.max - self.min)
    }
}

pub const PARAMETER_DEFS: [ParameterDef; 12] = [
    ParameterDef {
        id: "coldness",
        name: "Coldness",
        default: 0.62,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "wind",
        name: "Wind",
        default: 0.18,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "ice_size",
        name: "Ice Size",
        default: 0.55,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "ancient_depth",
        name: "Ancient Depth",
        default: 0.48,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "frozen_harmonics",
        name: "Frozen Harmonics",
        default: 0.24,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "storm",
        name: "Storm",
        default: 0.20,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "distance",
        name: "Distance",
        default: 0.42,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "decay",
        name: "Decay",
        default: 0.58,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "freeze",
        name: "Freeze",
        default: 0.0,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "mix",
        name: "Mix",
        default: 0.34,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "input",
        name: "Input",
        default: 0.75,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
    ParameterDef {
        id: "output",
        name: "Output",
        default: 0.75,
        min: 0.0,
        max: 1.0,
        unit: "",
        automatable: true,
    },
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrostVerbParams {
    pub coldness: f32,
    pub wind: f32,
    pub ice_size: f32,
    pub ancient_depth: f32,
    pub frozen_harmonics: f32,
    pub storm: f32,
    pub distance: f32,
    pub decay: f32,
    pub freeze: f32,
    pub mix: f32,
    pub input: f32,
    pub output: f32,
}

impl Default for FrostVerbParams {
    fn default() -> Self {
        Self {
            coldness: PARAMETER_DEFS[0].default,
            wind: PARAMETER_DEFS[1].default,
            ice_size: PARAMETER_DEFS[2].default,
            ancient_depth: PARAMETER_DEFS[3].default,
            frozen_harmonics: PARAMETER_DEFS[4].default,
            storm: PARAMETER_DEFS[5].default,
            distance: PARAMETER_DEFS[6].default,
            decay: PARAMETER_DEFS[7].default,
            freeze: PARAMETER_DEFS[8].default,
            mix: PARAMETER_DEFS[9].default,
            input: PARAMETER_DEFS[10].default,
            output: PARAMETER_DEFS[11].default,
        }
    }
}

impl FrostVerbParams {
    pub const FIELD_COUNT: usize = 12;

    pub fn sanitized(mut self) -> Self {
        self.coldness = clamp01(self.coldness);
        self.wind = clamp01(self.wind);
        self.ice_size = clamp01(self.ice_size);
        self.ancient_depth = clamp01(self.ancient_depth);
        self.frozen_harmonics = clamp01(self.frozen_harmonics);
        self.storm = clamp01(self.storm);
        self.distance = clamp01(self.distance);
        self.decay = clamp01(self.decay);
        self.freeze = clamp01(self.freeze);
        self.mix = clamp01(self.mix);
        self.input = clamp01(self.input);
        self.output = clamp01(self.output);
        self
    }

    pub fn as_array(self) -> [f32; Self::FIELD_COUNT] {
        [
            self.coldness,
            self.wind,
            self.ice_size,
            self.ancient_depth,
            self.frozen_harmonics,
            self.storm,
            self.distance,
            self.decay,
            self.freeze,
            self.mix,
            self.input,
            self.output,
        ]
    }

    pub fn from_array(values: [f32; Self::FIELD_COUNT]) -> Self {
        Self {
            coldness: values[0],
            wind: values[1],
            ice_size: values[2],
            ancient_depth: values[3],
            frozen_harmonics: values[4],
            storm: values[5],
            distance: values[6],
            decay: values[7],
            freeze: values[8],
            mix: values[9],
            input: values[10],
            output: values[11],
        }
        .sanitized()
    }
}

pub fn clamp01(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}
