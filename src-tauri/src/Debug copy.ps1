# USB Fixer - Par Angel Virion
# Prépare une clé USB pour HPUSBDisk

# Vérifie les droits admin
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process powershell -Verb runAs -ArgumentList "-File `"$PSCommandPath`""
    exit
}

Write-Host "=== USB Fixer ===" -ForegroundColor Cyan
Write-Host ""

# Liste les clés USB
$usbDisks = Get-Disk | Where-Object { $_.BusType -eq 'USB' }

if ($usbDisks.Count -eq 0) {
    Write-Host "Aucune clé USB détectée!" -ForegroundColor Red
    Read-Host "Appuyez sur Entrée pour quitter"
    exit
}

# Affiche les clés
Write-Host "Clés USB détectées:" -ForegroundColor Yellow
foreach ($disk in $usbDisks) {
    $sizeGB = [math]::Round($disk.Size / 1GB, 1)
    Write-Host "  Disque $($disk.Number): $($disk.FriendlyName) - $sizeGB GB"
}

$diskNumber = $usbDisks[0].Number
$selected = $usbDisks[0]

Write-Host ""
Write-Host "Clé sélectionnée: $($selected.FriendlyName)" -ForegroundColor Green
Write-Host "ATTENTION: Toutes les données seront effacées!" -ForegroundColor Red
$confirm = Read-Host "Continuer? (O/N)"

if ($confirm -ne "O" -and $confirm -ne "o") {
    exit
}

Write-Host ""
Write-Host "Préparation..." -ForegroundColor Cyan

try {
    # Étape 1: Clean avec diskpart (plus fiable)
    Write-Host "  [1/3] Nettoyage..."
    
    $diskpartScript = @"
select disk $diskNumber
clean
convert mbr
create partition primary
active
assign
"@
    
    $diskpartScript | diskpart | Out-Null
    
    Start-Sleep -Seconds 3
    
    # Étape 2: Récupère la lettre assignée
    Write-Host "  [2/3] Récupération de la lettre..."
    
    # Refresh disk info
    Update-Disk -Number $diskNumber -ErrorAction SilentlyContinue
    
    $partition = Get-Partition -DiskNumber $diskNumber -ErrorAction SilentlyContinue | Where-Object { $_.DriveLetter }
    
    if ($partition) {
        $driveLetter = $partition.DriveLetter
        Write-Host "  [3/3] Lettre assignée: ${driveLetter}:" -ForegroundColor Green
    } else {
        Write-Host "  [3/3] Assignation manuelle de la lettre..."
        # Force assign
        $part = Get-Partition -DiskNumber $diskNumber | Select-Object -First 1
        $part | Add-PartitionAccessPath -AccessPath "F:" -ErrorAction SilentlyContinue
        $driveLetter = "F"
    }
    
    Write-Host ""
    Write-Host "=== PRÊT ===" -ForegroundColor Green
    Write-Host "La clé est prête sur ${driveLetter}:" -ForegroundColor Green
    Write-Host "HPUSBDisk peut maintenant la formater en FAT32." -ForegroundColor Cyan
    
    # Ouvre HPUSBDisk si présent
    $hpPath = Join-Path $PSScriptRoot "HPUSBDisk.exe"
    if (Test-Path $hpPath) {
        Write-Host ""
        Write-Host "Ouverture de HPUSBDisk..." -ForegroundColor Yellow
        Start-Process $hpPath
    }
    
} catch {
    Write-Host ""
    Write-Host "ERREUR: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Read-Host "Appuyez sur Entrée pour quitter"
