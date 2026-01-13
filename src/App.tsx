/**
 * USB Fixer - Application principale
 * Par Angel Virion
 * 
 * Déverrouille les clés USB en écriture seule pour HP USB Disk Storage Format Tool
 */

import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

/* =============================================================================
   TYPES
============================================================================= */

/** Représente une clé USB détectée par le système */
interface UsbDrive {
  disk_number: number;
  friendly_name: string;
  size_formatted: string;
  drive_letter: string | null;
  is_readonly: boolean;
}

/** Types de statut pour les messages utilisateur */
type StatusType = "info" | "success" | "error" | "processing";

/* =============================================================================
   ICÔNES SVG
============================================================================= */

const Icons = {
  Usb: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M6 3v9a3 3 0 1 0 0 6 3 3 0 0 0 0-6m0 6v3M18 9v6a3 3 0 1 0 0 6 3 3 0 0 0 0-6M18 3v2a2 2 0 1 0 0 4 2 2 0 0 0 0-4M6 12h6a2 2 0 0 0 2-2V4M12 12h4" />
    </svg>
  ),
  Refresh: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8M21 3v5h-5" />
    </svg>
  ),
  Warning: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
      <line x1="12" y1="9" x2="12" y2="13" />
      <line x1="12" y1="17" x2="12.01" y2="17" />
    </svg>
  ),
  Check: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3">
      <polyline points="20 6 9 17 4 12" />
    </svg>
  ),
  Flash: () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
    </svg>
  ),
};

/* =============================================================================
   COMPOSANTS
============================================================================= */

/** Carte affichant une clé USB */
function DriveCard({ 
  drive, 
  selected, 
  disabled, 
  onSelect 
}: { 
  drive: UsbDrive; 
  selected: boolean; 
  disabled: boolean;
  onSelect: () => void;
}) {
  return (
    <div
      className={`usb-card ${selected ? "selected" : ""}`}
      onClick={() => !disabled && onSelect()}
    >
      {/* Icône */}
      <div className="usb-icon"><Icons.Usb /></div>
      
      {/* Informations */}
      <div className="usb-info">
        <div className="usb-name">
          {drive.friendly_name || `Disque ${drive.disk_number}`}
        </div>
        <div className="usb-meta">
          <span className="usb-size">{drive.size_formatted}</span>
          {drive.drive_letter && <span className="usb-letter">{drive.drive_letter}:</span>}
          {drive.is_readonly && <span className="usb-locked">🔒 Protégé</span>}
        </div>
      </div>
      
      {/* Indicateur de sélection */}
      <div className="select-dot">{selected && <Icons.Check />}</div>
    </div>
  );
}

/** Modal de confirmation avant formatage */
function ConfirmModal({ 
  drive, 
  onConfirm, 
  onCancel 
}: { 
  drive: UsbDrive; 
  onConfirm: () => void; 
  onCancel: () => void;
}) {
  return (
    <div className="modal-overlay" onClick={onCancel}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-icon"><Icons.Warning /></div>
        <h3>Confirmer le formatage</h3>
        <p>Toutes les données seront définitivement effacées.</p>
        <div className="modal-drive">
          <strong>{drive.friendly_name || `Disque ${drive.disk_number}`}</strong>
          <span>{drive.size_formatted}{drive.drive_letter && ` • ${drive.drive_letter}:`}</span>
        </div>
        <div className="modal-buttons">
          <button className="btn-cancel" onClick={onCancel}>Annuler</button>
          <button className="btn-confirm" onClick={onConfirm}>Formater</button>
        </div>
      </div>
    </div>
  );
}

/* =============================================================================
   APPLICATION PRINCIPALE
============================================================================= */

export default function App() {
  // État
  const [drives, setDrives] = useState<UsbDrive[]>([]);
  const [selected, setSelected] = useState<UsbDrive | null>(null);
  const [loading, setLoading] = useState(true);
  const [processing, setProcessing] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [status, setStatus] = useState<{ type: StatusType; msg: string } | null>(null);

  // Charge la liste des clés USB
  async function loadDrives() {
    setLoading(true);
    try {
      const result = await invoke<UsbDrive[]>("get_usb_drives");
      setDrives(result);
      // Désélectionne si la clé n'existe plus
      if (selected && !result.find((d) => d.disk_number === selected.disk_number)) {
        setSelected(null);
      }
    } catch (err) {
      setStatus({ type: "error", msg: `Erreur: ${err}` });
    } finally {
      setLoading(false);
    }
  }

  // Formate la clé USB sélectionnée
  async function formatDrive() {
    if (!selected) return;
    
    setShowModal(false);
    setProcessing(true);
    setStatus({ type: "processing", msg: "Formatage en cours..." });

    try {
      await invoke("fix_usb_drive", { diskNumber: selected.disk_number });
      setStatus({ type: "success", msg: "Succès! HP USB Disk va s'ouvrir." });
      setTimeout(() => { loadDrives(); setSelected(null); }, 2000);
    } catch (err) {
      setStatus({ type: "error", msg: `Erreur: ${err}` });
    } finally {
      setProcessing(false);
    }
  }

  // Chargement initial
  useEffect(() => { loadDrives(); }, []);

  /* -------------------------------------------------------------------------
     RENDU
  ------------------------------------------------------------------------- */
  return (
    <div className="app">
      {/* En-tête */}
      <header className="header">
        <div className="logo"><Icons.Usb /></div>
        <h1>USB Fixer</h1>
        <p className="subtitle">Déverrouille les clés USB protégées en écriture</p>
        <span className="author">Par Angel Virion</span>
      </header>

      {/* Avertissement */}
      <div className="warning">
        <Icons.Warning />
        <span><strong>Attention:</strong> Toutes les données seront effacées!</span>
      </div>

      {/* Liste des clés USB */}
      <section className="drives-section">
        <div className="section-header">
          <h2>Clé USB</h2>
          <button 
            className={`btn-refresh ${loading ? "spinning" : ""}`}
            onClick={loadDrives}
            disabled={loading || processing}
          >
            <Icons.Refresh />
          </button>
        </div>

        {drives.length === 0 ? (
          <div className="empty">
            <Icons.Usb />
            <p>Aucune clé USB détectée</p>
          </div>
        ) : (
          <div className="drives-list">
            {drives.map((drive) => (
              <DriveCard
                key={drive.disk_number}
                drive={drive}
                selected={selected?.disk_number === drive.disk_number}
                disabled={processing}
                onSelect={() => setSelected(drive)}
              />
            ))}
          </div>
        )}
      </section>

      {/* Bouton d'action */}
      <button
        className="btn-format"
        onClick={() => setShowModal(true)}
        disabled={!selected || processing}
      >
        <Icons.Flash />
        {processing ? "Traitement..." : "Formater la clé USB"}
      </button>

      {/* Message de statut */}
      {status && (
        <div className={`status ${status.type}`}>
          {status.type === "processing" && <div className="spinner" />}
          {status.msg}
        </div>
      )}

      {/* Modal de confirmation */}
      {showModal && selected && (
        <ConfirmModal
          drive={selected}
          onConfirm={formatDrive}
          onCancel={() => setShowModal(false)}
        />
      )}

      {/* Pied de page */}
      <footer>USB Fixer v1.0</footer>
    </div>
  );
}
