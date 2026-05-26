import { useState } from "react";
import { Snowflake, Waves } from "lucide-react";
import { FrostKnob } from "./FrostKnob";
import { FrostMeter } from "./FrostMeter";
import { FrostSlider } from "./FrostSlider";
import "./FrostPluginPanel.css";

const ornateTopbar = new URL("../../oranet.png", import.meta.url).href;

type FrostPluginPanelProps = {
  storm?: number;
  onStormChange?: (value: number) => void;
};

export function FrostPluginPanel({ storm, onStormChange }: FrostPluginPanelProps) {
  const [ice, setIce] = useState(68);
  const [wind, setWind] = useState(42);
  const [freeze, setFreeze] = useState(76);
  const [internalStorm, setInternalStorm] = useState(74);
  const [decay, setDecay] = useState(61);
  const [depth, setDepth] = useState(48);
  const stormValue = storm ?? internalStorm;

  const handleStormChange = (value: number) => {
    setInternalStorm(value);
    onStormChange?.(value);
  };

  return (
    <section
      className="frost-panel"
      aria-label="Frostverb component composition"
    >
      <div className="frost-panel__topbar" aria-hidden="true">
        <img src={ornateTopbar} alt="" />
      </div>
      <div className="frost-panel__header">
        <div>
          <h1>Frost Verb</h1>
          <p>Nursven WebUI component lab</p>
        </div>
      </div>
      <div className="frost-panel__controls">
        <FrostKnob
          label="Ice Resonance"
          value={ice}
          variant="iceKnob"
          onChange={setIce}
        />
        <FrostSlider label="Wind" value={wind} onChange={setWind} />
        <FrostKnob
          label="Spectral Freeze"
          value={freeze}
          variant="crystalKnob"
          onChange={setFreeze}
        />
        <FrostKnob
          label="Storm"
          value={stormValue}
          variant="runeKnob"
          onChange={handleStormChange}
        />
        <FrostKnob
          label="Decay"
          value={decay}
          variant="woodKnob"
          onChange={setDecay}
        />
        <FrostKnob
          label="Ancient Depth"
          value={depth}
          variant="runeKnob"
          onChange={setDepth}
        />
      </div>
      <div className="frost-panel__meters">
        <FrostMeter
          label="Glacial Return"
          channels={[
            { label: "L", value: 42 },
            { label: "R", value: 48 },
          ]}
          width={560}
        />
        <FrostMeter label="Freeze Bloom" value={76} width={520} />
      </div>
      <div className="frost-panel__mode">
        <Snowflake size={26} />
        <span>Glacial Echoes</span>
        <Waves size={26} />
      </div>
    </section>
  );
}
