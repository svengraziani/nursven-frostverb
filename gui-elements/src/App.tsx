import { FrostPluginPanel, SnowfallBackground } from './components';
import './App.css';

export function App() {
  return (
    <>
      <SnowfallBackground />
      <main className="app-shell">
        <FrostPluginPanel />
      </main>
    </>
  );
}
