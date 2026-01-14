// USB Fixer - Par Angel Virion
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Drive { disk_number: number; name: string; size: string; letter: string | null }

export default function App() {
  const [drives, setDrives] = useState<Drive[]>([]);
  const [sel, setSel] = useState<number | null>(null);
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);

  const load = async () => {
    setLoading(true);
    try { setDrives(await invoke<Drive[]>("get_drives")); }
    catch (e) { setStatus(""+e); }
    setLoading(false);
  };
  
  const format = async () => {
    if (sel === null) return;
    setStatus("Formatage...");
    await invoke("format_drive", { n: sel }).catch(e => setStatus(""+e));
    setStatus("Terminé! HPUSBDisk s'ouvre.");
    setTimeout(() => { load(); setSel(null); setStatus(""); }, 2000);
  };

  useEffect(() => { load(); }, []);

  return (
    <div className="app">
      <h1>USB Fixer</h1>
      <p className="author">Par Angel Virion</p>
      <div className="warn">⚠️ Toutes les données seront effacées!</div>
      
      <div className="header">
        <span>Clé USB</span>
        <button onClick={load} disabled={loading} className={loading ? "spin" : ""}><span>↻</span></button>
      </div>
      
      {drives.length === 0 ? <div className="empty">Aucune clé USB</div> : drives.map(d => (
        <div key={d.disk_number} className={`drive ${sel === d.disk_number ? "sel" : ""}`}
             onClick={() => setSel(sel === d.disk_number ? null : d.disk_number)}>
          <div><b>{d.name || `Disque ${d.disk_number}`}</b><span>{d.size}{d.letter && ` • ${d.letter}:`}</span></div>
          {sel === d.disk_number && <span>✓</span>}
        </div>
      ))}
      
      <button className="fmt" onClick={format} disabled={sel === null}>Formater</button>
      {status && <div className="status">{status}</div>}
    </div>
  );
}
