import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Drive { disk_number: number; name: string; size: string; letter: string | null }

type Status = { text: string; type: "info" | "success" | "error" };

const RefreshIcon = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
    <path d="M21 2v6h-6"/><path d="M3 12a9 9 0 0 1 15-6.7L21 8"/><path d="M3 22v-6h6"/><path d="M21 12a9 9 0 0 1-15 6.7L3 16"/>
  </svg>
);

export default function App() {
  const [drives, setDrives] = useState<Drive[]>([]);
  const [sel, setSel] = useState<number | null>(null);
  const [status, setStatus] = useState<Status | null>(null);
  const [loading, setLoading] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    try { setDrives(await invoke<Drive[]>("get_drives")); }
    catch (e) { setStatus({ text: String(e), type: "error" }); }
    finally { setLoading(false); }
  }, []);

  const format = useCallback(async () => {
    if (sel === null) return;
    setStatus({ text: "Formatage en cours...", type: "info" });
    try {
      const result = await invoke<string>("format_drive", { n: sel });
      setStatus({ text: result + " — HPUSBDisk s'ouvre.", type: "success" });
    } catch (e) {
      setStatus({ text: String(e), type: "error" });
    }
    setSel(null);
    setTimeout(() => { setStatus(null); load(); }, 4000);
  }, [sel, load]);

  useEffect(() => { load(); }, [load]);

  return (
    <div className="app">
      <div className="brand">
        <h1>USB Fixer</h1>
        <p>Par Angel Virion</p>
      </div>

      <div className="warn">Toutes les données seront effacées</div>

      <div className="section-header">
        <span>Périphériques USB</span>
        <button onClick={load} disabled={loading} className={loading ? "spin" : ""} aria-label="Actualiser">
          <RefreshIcon />
        </button>
      </div>

      <div className="drive-list">
        {drives.length === 0 ? <div className="empty">Aucune clé USB détectée</div> : drives.map(d => (
          <div key={d.disk_number} className={`drive ${sel === d.disk_number ? "sel" : ""}`}
               onClick={() => setSel(prev => prev === d.disk_number ? null : d.disk_number)}
               role="button" tabIndex={0} aria-selected={sel === d.disk_number}
               onKeyDown={e => e.key === "Enter" && setSel(prev => prev === d.disk_number ? null : d.disk_number)}>
            <div className="drive-info">
              <span className="drive-name">{d.name || `Disque ${d.disk_number}`}</span>
              <span className="drive-meta">{d.size}{d.letter && ` • ${d.letter}:`}</span>
            </div>
            <div className="check">✓</div>
          </div>
        ))}
      </div>

      <button className="fmt" onClick={format} disabled={sel === null}>
        {status?.type === "info" ? "Formatage..." : "Formater la clé"}
      </button>

      {status && <div className={`status ${status.type}`}>{status.text}</div>}
    </div>
  );
}
