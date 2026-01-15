#Fait par Angel Virion
#Formatage de clé usb et préparation pour le logiciel HPUSBFW.exe (enleve le write protection)

set-executionpolicy RemoteSigned CurrentUser
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator"))  
    {  
      $arguments = "& '" +$myinvocation.mycommand.definition + "'"
      Start-Process powershell -Verb runAs -ArgumentList $arguments
      Break
    }
$usbDisks = Get-Disk | Where-Object { $_.BusType -eq 'USB' } #prend les disk qui sont usb
Get-Disk | Where-Object { $_.BusType -eq 'USB' } | Clear-Disk -RemoveData -RemoveOEM -Confirm:$false | Initialize-Disk -PartitionStyle MBR #clean le drive et converti en mbr
New-Partition -DiskNumber $usbDisks[0].Number -UseMaximumSize -IsActive -AssignDriveLetter #Créer une partition et formate le disque

Start-Process -FilePath "$PSScriptRoot\HPUSBDisk.exe" #ouvre HPUSBDisk

Read-Host "fin du programme. Appuyez sur Enter pour quitter."