// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::{Manager, Window};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn new_hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}
#[derive(serde::Serialize, Clone)]
struct InstallProgress {
    app_id: String,
    app_name: String,
    status: String, // "installing" | "done" | "error" | "skipped"
    message: String,
}

#[tauri::command]
fn is_admin() -> bool {
    #[cfg(target_os = "windows")]
    {
        match new_hidden_command("net").arg("session").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

async fn run_optimization(window: &Window, id: &str, name: &str) {
    // Emit starting optimization
    window
        .emit(
            "install_progress",
            InstallProgress {
                app_id: id.to_string(),
                app_name: name.to_string(),
                status: "installing".to_string(),
                message: format!("Optimizing {}...", name),
            },
        )
        .unwrap();

    let script = match id {
        "Opt.CleanTemp" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Remove-Item -Path "$env:TEMP\*" -Recurse -Force
            Remove-Item -Path "C:\Windows\Temp\*" -Recurse -Force
            Clear-DnsClientCache
            Write-Output "Cleaned temp files, prefetch, and DNS cache."
            "#
        }
        "Opt.Debloat" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $apps = @(
                "Microsoft.3DBuilder", "Microsoft.BingWeather", "Microsoft.GetHelp", 
                "Microsoft.Getstarted", "Microsoft.Messaging", "Microsoft.MicrosoftSolitaireCollection", 
                "Microsoft.MixedReality.Portal", "Microsoft.OneConnect", "Microsoft.People", 
                "Microsoft.Print3D", "Microsoft.SkypeApp", "Microsoft.Wallet", "Microsoft.XboxApp", 
                "Microsoft.XboxGameOverlay", "Microsoft.XboxGamingOverlay", "Microsoft.XboxIdentityProvider", 
                "Microsoft.YourPhone", "Microsoft.ZuneMusic", "Microsoft.ZuneVideo"
            )
            $count = 0
            foreach ($app in $apps) {
                $p = Get-AppxPackage -Name $app -AllUsers
                if ($p) {
                    $p | Remove-AppxPackage
                    $count++
                }
            }
            Write-Output "Successfully removed $count pre-installed bloatware apps."
            "#
        }
        "Opt.HighPerf" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            powercfg /setactive 8c5e7fda-e8bf-4a94-9a85-a6e23a8c635c
            powercfg /duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 | Out-Null
            $plans = powercfg /list
            $ultimate = $plans | Select-String "Ultimate Performance"
            if ($ultimate) {
                $guid = [regex]::Match($ultimate.Line, '([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})').Value
                powercfg /setactive $guid
                Write-Output "Ultimate Performance plan activated."
            } else {
                Write-Output "High Performance plan activated."
            }
            "#
        }
        "Opt.VisualEffects" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" -Name "VisualFXSetting" -Value 2
            Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "UserPreferencesMask" -Value ([byte[]](144, 18, 3, 128, 16, 0, 0, 0))
            Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "MenuShowDelay" -Value "0"
            Write-Output "Visual effects optimized for performance."
            "#
        }
        "Opt.NetworkTweak" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            netsh int tcp set global autotuninglevel=normal
            netsh int tcp set global chimney=enabled
            netsh int tcp set global dca=enabled
            netsh int tcp set global netdma=enabled
            netsh int tcp set global ecncapability=enabled
            Write-Output "TCP Auto-Tuning and network adapter settings optimized."
            "#
        }
        "Opt.DisableStartup" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "OneDrive"
            Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "com.squirrel.Teams.Teams"
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run" -Name "com.squirrel.Teams.Teams" -Value ([byte[]](3,0,0,0,0,0,0,0,0,0,0,0))
            Write-Output "Startup programs (OneDrive, Teams) disabled."
            "#
        }
        "Opt.DarkMode" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" -Name "AppsUseLightTheme" -Value 0
            Set-ItemProperty -Name "SystemUsesLightTheme" -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" -Value 0
            Write-Output "Dark Mode enabled system-wide."
            "#
        }
        "Opt.Telemetry" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Stop-Service -Name "DiagTrack"
            Set-Service -Name "DiagTrack" -StartupType Disabled
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry" -Value 0
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection" -Name "AllowTelemetry" -Value 0
            Write-Output "Telemetry services and Windows tracking disabled."
            "#
        }
        "Opt.DefaultBrowserPrompt" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            New-Item -Path "HKCU:\Software\Policies\Microsoft" -Name "Edge" -Force | Out-Null
            Set-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Edge" -Name "DefaultBrowserSettingEnabled" -Value 0
            Set-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Edge" -Name "HideFirstRunExperience" -Value 1
            Write-Output "Edge default browser prompt and first-run experience disabled."
            "#
        }
        "Opt.TrimSSD" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Get-Volume | Where-Object {$_.DriveType -eq 'Fixed' -and $_.FileSystemLabel -ne 'System Reserved'} | Optimize-Volume -Defrag -Trim -Verbose
            Write-Output "TRIM optimization triggered on all volumes."
            "#
        }
        "Opt.DisableHibernate" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            powercfg.exe /hibernate off
            Write-Output "Hibernation disabled system-wide."
            "#
        }
        "Opt.ResetNetwork" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            ipconfig /flushdns
            netsh winsock reset
            netsh int ip reset
            Write-Output "DNS cache flushed and network stack reset."
            "#
        }
        "Opt.DisableCortana" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows" -Name "Windows Search" -Force | Out-Null
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "AllowCortana" -Value 0
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search" -Name "BingSearchEnabled" -Value 0
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search" -Name "CortanaConsent" -Value 0
            Write-Output "Cortana and Bing web search in Start menu disabled."
            "#
        }
        "Opt.DisableGameDVR" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled" -Value 0
            New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows" -Name "GameDVR" -Force | Out-Null
            Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\GameDVR" -Name "AllowGameDVR" -Value 0
            Write-Output "Xbox Game DVR and background recording disabled."
            "#
        }
        "Opt.CloudflareDns" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $adapters = Get-NetAdapter | Where-Object {$_.Status -eq 'Up'}
            foreach ($adapter in $adapters) {
                Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses ("1.1.1.1", "1.0.0.1")
            }
            Write-Output "DNS servers updated to Cloudflare (1.1.1.1)."
            "#
        }
        "Opt.DisableIndexing" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Stop-Service -Name "WSearch" -Force
            Set-Service -Name "WSearch" -StartupType Disabled
            Write-Output "Windows Search Indexing service disabled and stopped."
            "#
        }
        "Opt.GodMode" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $DesktopPath = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), 'GodMode.{ED7BA470-8E54-465E-825C-99712043E01C}')
            New-Item -ItemType Directory -Path $DesktopPath -Force | Out-Null
            Write-Output "God Mode folder successfully created on your Desktop."
            "#
        }
        "Opt.FlushDNS" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $DesktopPath = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), "Flush DNS.bat")
            "@echo off`r`nipconfig /flushdns`r`necho DNS Cache Flushed!`r`npause" | Out-File -FilePath $DesktopPath -Encoding ascii -Force
            Write-Output "Flush DNS batch shortcut created on your Desktop."
            "#
        }
        "Opt.ScheduleCleanup" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $Action = New-ScheduledTaskAction -Execute 'PowerShell.exe' -Argument '-NoProfile -WindowStyle Hidden -Command "Remove-Item -Path \"$env:TEMP\*\" -Recurse -Force -ErrorAction SilentlyContinue; Remove-Item -Path \"C:\Windows\Temp\*\" -Recurse -Force -ErrorAction SilentlyContinue; Clear-DnsClientCache -ErrorAction SilentlyContinue"'
            $Trigger = New-ScheduledTaskTrigger -Daily -At '3:00 AM'
            $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable
            Register-ScheduledTask -TaskName "WinForgeDailyCleanup" -Action $Action -Trigger $Trigger -Settings $Settings -Description "Daily background temp files cleanup and DNS cache flush created by WinForge." -Force | Out-Null
            Write-Output "Successfully scheduled daily cleanup task in Windows Task Scheduler."
            "#
        }
        _ => "",
    };

    if script.is_empty() {
        window
            .emit(
                "install_progress",
                InstallProgress {
                    app_id: id.to_string(),
                    app_name: name.to_string(),
                    status: "error".to_string(),
                    message: "Unknown optimization task".to_string(),
                },
            )
            .unwrap();
        return;
    }

    let output = new_hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output();

    match output {
        Ok(out) => {
            let success = out.status.success();
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();

            if success {
                let msg = if stdout.trim().is_empty() {
                    format!("{} completed successfully", name)
                } else {
                    stdout.trim().to_string()
                };
                window
                    .emit(
                        "install_progress",
                        InstallProgress {
                            app_id: id.to_string(),
                            app_name: name.to_string(),
                            status: "done".to_string(),
                            message: msg,
                        },
                    )
                    .unwrap();
            } else {
                let err_msg = if stderr.trim().is_empty() {
                    format!("Failed to apply {}", name)
                } else {
                    format!("Error: {}", stderr.trim())
                };
                window
                    .emit(
                        "install_progress",
                        InstallProgress {
                            app_id: id.to_string(),
                            app_name: name.to_string(),
                            status: "error".to_string(),
                            message: err_msg,
                        },
                    )
                    .unwrap();
            }
        }
        Err(e) => {
            window
                .emit(
                    "install_progress",
                    InstallProgress {
                        app_id: id.to_string(),
                        app_name: name.to_string(),
                        status: "error".to_string(),
                        message: format!("Error running PowerShell: {}", e),
                    },
                )
                .unwrap();
        }
    }
}

#[tauri::command]
async fn install_apps(window: Window, apps: Vec<serde_json::Value>) -> Result<(), String> {
    for app in apps {
        let id = app["id"].as_str().unwrap_or("").to_string();
        let name = app["name"].as_str().unwrap_or("").to_string();

        // Check if this is a system optimization task
        if id.starts_with("Opt.") {
            run_optimization(&window, &id, &name).await;
            continue;
        }

        // Emit status: checking first
        window
            .emit(
                "install_progress",
                InstallProgress {
                    app_id: id.clone(),
                    app_name: name.clone(),
                    status: "installing".to_string(),
                    message: format!("Checking status of {}...", name),
                },
            )
            .unwrap();

        // 1. Locale-independent installation check using `winget list --id`
        let is_installed = match new_hidden_command("winget")
            .args(["list", "--id", &id])
            .output()
        {
            Ok(out) => out.status.success(),
            Err(_) => false,
        };

        if is_installed {
            window
                .emit(
                    "install_progress",
                    InstallProgress {
                        app_id: id.clone(),
                        app_name: name.clone(),
                        status: "skipped".to_string(),
                        message: format!("{} is already installed", name),
                    },
                )
                .unwrap();
            continue;
        }

        // Emit status: installing
        window
            .emit(
                "install_progress",
                InstallProgress {
                    app_id: id.clone(),
                    app_name: name.clone(),
                    status: "installing".to_string(),
                    message: format!("Installing {}...", name),
                },
            )
            .unwrap();

        // 2. Perform silent installation
        let output = new_hidden_command("winget")
            .args([
                "install",
                "--id",
                &id,
                "--silent",
                "--accept-package-agreements",
                "--accept-source-agreements",
            ])
            .output();

        match output {
            Ok(out) => {
                let success = out.status.success();
                if success {
                    let mut msg = format!("{} installed successfully", name);
                    if id == "Python.Python.3.12" {
                        match new_hidden_command("cmd").args(["/c", "python -m pip install --upgrade pip"]).output() {
                            Ok(pip_out) => {
                                if pip_out.status.success() {
                                    msg.push_str(" and upgraded pip to the latest version.");
                                } else {
                                    let pip_err = String::from_utf8_lossy(&pip_out.stderr).to_string();
                                    msg.push_str(&format!(" (Note: Pip upgrade failed: {})", pip_err.trim()));
                                }
                            }
                            Err(e) => {
                                msg.push_str(&format!(" (Note: Could not run pip upgrade: {})", e));
                            }
                        }
                    }

                    window
                        .emit(
                            "install_progress",
                            InstallProgress {
                                app_id: id,
                                app_name: name.clone(),
                                status: "done".to_string(),
                                message: msg,
                            },
                        )
                        .unwrap();
                } else {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                    
                    let mut error_detail = String::new();
                    
                    // Search stdout/stderr for specific exit codes or HRESULTs
                    if let Some(code) = out.status.code() {
                        error_detail = format!("Exit code: {}", code);
                    }
                    
                    // Try to find a line with error code or error message
                    let combined_output = format!("{}\n{}", stdout, stderr);
                    if let Some(err_line) = combined_output.lines().find(|l| l.contains("0x") || l.contains("error") || l.contains("failed") || l.contains("Error")) {
                        if !error_detail.is_empty() {
                            error_detail = format!("{}, details: {}", error_detail, err_line.trim());
                        } else {
                            error_detail = err_line.trim().to_string();
                        }
                    }

                    let message = if error_detail.is_empty() {
                        format!("Failed to install {}", name)
                    } else {
                        format!("Failed to install {}: {}", name, error_detail)
                    };

                    window
                        .emit(
                            "install_progress",
                            InstallProgress {
                                app_id: id,
                                app_name: name,
                                status: "error".to_string(),
                                message,
                            },
                        )
                        .unwrap();
                }
            }
            Err(e) => {
                window
                    .emit(
                        "install_progress",
                        InstallProgress {
                            app_id: id,
                            app_name: name,
                            status: "error".to_string(),
                            message: format!("Error running winget: {}", e),
                        },
                    )
                    .unwrap();
            }
        }
    }

    // Signal completion
    window.emit("install_complete", {}).unwrap();
    Ok(())
}


#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct SystemInfo {
    cpu: String,
    gpu: String,
    ram: String,
    storage: String,
    os: String,
    tpm: String,
    #[serde(rename = "secureBoot")]
    secure_boot: String,
}

#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, String> {
    let script = r#"
        $ErrorActionPreference = 'SilentlyContinue'
        
        # CPU Info
        $cpu_info = Get-CimInstance Win32_Processor
        if ($cpu_info -is [array]) { $cpu_info = $cpu_info[0] }
        $cpu_name = $cpu_info.Name.Trim()
        $cores = $cpu_info.NumberOfCores
        $threads = $cpu_info.NumberOfLogicalProcessors
        if ($cores -and $threads) {
            $cpu_display = "$cpu_name ($cores Cores / $threads Threads)"
        } else {
            $cpu_display = $cpu_name
        }
        
        # GPU Info
        $gpu = (Get-CimInstance Win32_VideoController).Name
        if ($gpu -is [array]) { $gpu = $gpu -join ', ' }
        
        # RAM Info
        $ram_bytes = (Get-CimInstance Win32_PhysicalMemory | Measure-Object Capacity -Sum).Sum
        $ram_gb = [Math]::Round($ram_bytes / 1GB)
        $ram_speed = (Get-CimInstance Win32_PhysicalMemory | Measure-Object Speed -Maximum).Maximum
        if ($ram_speed -and $ram_speed -gt 0) {
            $ram_display = "$ram_gb GB @ $ram_speed MHz"
        } else {
            $ram_display = "$ram_gb GB"
        }
        
        # Storage Info (all fixed volumes)
        $vols = Get-Volume | Where-Object { $_.DriveType -eq 'Fixed' -and $_.DriveLetter }
        $storage_info = @()
        foreach ($vol in $vols) {
            $free = [Math]::Round($vol.SizeRemaining / 1GB)
            $total = [Math]::Round($vol.Size / 1GB)
            $storage_info += "$($vol.DriveLetter): $free GB free / $total GB"
        }
        $storage_display = $storage_info -join "`n"
        
        # OS Info
        $os = (Get-CimInstance Win32_OperatingSystem).Caption
        $build = (Get-CimInstance Win32_OperatingSystem).BuildNumber
        
        # TPM Info
        $tpm_status = "Not Found / Disabled"
        try {
            $tpm = Get-CimInstance -Namespace Root\CIMv2\Security\MicrosoftTpm -ClassName Win32_Tpm -ErrorAction Stop
            if ($tpm) {
                $ver = $tpm.SpecVersion.Split(",")[0].Trim()
                if ($tpm.IsEnabled_InitialValue -eq $true -or $tpm.IsActivated_InitialValue -eq $true) {
                    $tpm_status = "TPM $ver Enabled"
                } else {
                    $tpm_status = "TPM $ver Disabled"
                }
            } else {
                throw "No TPM instance"
            }
        } catch {
            try {
                $tpmtool = tpmtool getdeviceinformation
                $present = ($tpmtool | Select-String "-TPM Present:").Line
                $version = ($tpmtool | Select-String "-TPM Version:").Line
                if ($present -match "True") {
                    $ver = "2.0"
                    if ($version -match "([0-9\.]+)") {
                        $ver = $Matches[1]
                    }
                    $tpm_status = "TPM $ver Enabled"
                }
            } catch {
                $tpm_status = "Not Found / Disabled"
            }
        }
        
        # Secure Boot Info
        $sb_status = "Disabled"
        try {
            $sb = Get-ItemProperty -Path "HKLM:\System\CurrentControlSet\Control\SecureBoot\State" -Name "UEFISecureBootEnabled" -ErrorAction SilentlyContinue
            if ($sb -and $sb.UEFISecureBootEnabled -eq 1) {
                $sb_status = "Enabled"
            }
        } catch {
            $sb_status = "Disabled"
        }
        
        $result = @{
            cpu = $cpu_display
            gpu = $gpu.Trim()
            ram = $ram_display
            storage = $storage_display
            os = "$os (Build $build)"
            tpm = $tpm_status
            secureBoot = $sb_status
        }
        $result | ConvertTo-Json
    "#;

    let output = new_hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let info: SystemInfo = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse JSON: {}, output: {}", e, stdout))?;

    Ok(info)
}

#[tauri::command]
async fn get_applied_tweaks() -> Vec<String> {
    let script = r#"
        $applied = @()
        $ErrorActionPreference = 'SilentlyContinue'

        if (-not (Get-AppxPackage -Name "Microsoft.3DBuilder" -AllUsers)) { $applied += "Opt.Debloat" }

        $activeScheme = powercfg /getactivescheme
        if ($activeScheme -like "*Ultimate Performance*" -or $activeScheme -like "*High Performance*") { $applied += "Opt.HighPerf" }

        $fx = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" -Name "VisualFXSetting"
        if ($fx -and $fx.VisualFXSetting -eq 2) { $applied += "Opt.VisualEffects" }

        $tcp = netsh int tcp show global
        if ($tcp -like "*autotuninglevel=normal*" -and $tcp -like "*chimney=enabled*") { $applied += "Opt.NetworkTweak" }

        $run = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
        if ($run -and -not $run.OneDrive -and -not $run."com.squirrel.Teams.Teams") { $applied += "Opt.DisableStartup" }

        $theme = Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"
        if ($theme -and $theme.AppsUseLightTheme -eq 0 -and $theme.SystemUsesLightTheme -eq 0) { $applied += "Opt.DarkMode" }

        $diagTrack = Get-Service -Name "DiagTrack"
        $telReg = Get-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry"
        if (($diagTrack -and $diagTrack.StartType -eq "Disabled") -or ($telReg -and $telReg.AllowTelemetry -eq 0)) { $applied += "Opt.Telemetry" }

        $edge = Get-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Edge"
        if ($edge -and $edge.DefaultBrowserSettingEnabled -eq 0) { $applied += "Opt.DefaultBrowserPrompt" }

        $hib = Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Power" -Name "HibernateEnabled"
        if ($hib -and $hib.HibernateEnabled -eq 0) { $applied += "Opt.DisableHibernate" }

        $cortana = Get-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "AllowCortana"
        if ($cortana -and $cortana.AllowCortana -eq 0) { $applied += "Opt.DisableCortana" }

        $dvr = Get-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled"
        if ($dvr -and $dvr.GameDVR_Enabled -eq 0) { $applied += "Opt.DisableGameDVR" }

        $dns = Get-DnsClientServerAddress -AddressFamily IPv4
        if ($dns.ServerAddresses -contains "1.1.1.1") { $applied += "Opt.CloudflareDns" }

        $wsearch = Get-Service -Name "WSearch"
        if ($wsearch -and $wsearch.StartType -eq "Disabled") { $applied += "Opt.DisableIndexing" }

        $desktop = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), 'GodMode.{ED7BA470-8E54-465E-825C-99712043E01C}')
        if (Test-Path $desktop) { $applied += "Opt.GodMode" }

        $flushDns = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), "Flush DNS.bat")
        if (Test-Path $flushDns) { $applied += "Opt.FlushDNS" }

        if (Get-ScheduledTask -TaskName "WinForgeDailyCleanup") { $applied += "Opt.ScheduleCleanup" }

        if ($applied.Count -eq 0) { "[]" } else { ConvertTo-Json -InputObject @($applied) -Compress }
    "#;

    let output = match new_hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
    {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => return Vec::new(),
    };

    let trimmed = output.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    if trimmed.starts_with('[') {
        serde_json::from_str(trimmed).unwrap_or_else(|_| Vec::new())
    } else if trimmed.starts_with('"') {
        let single: Result<String, _> = serde_json::from_str(trimmed);
        match single {
            Ok(s) => vec![s],
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

#[tauri::command]
async fn revert_tweak(window: Window, id: String, name: String) -> Result<(), String> {
    window
        .emit(
            "install_progress",
            InstallProgress {
                app_id: id.clone(),
                app_name: name.clone(),
                status: "installing".to_string(),
                message: format!("Reverting {}...", name),
            },
        )
        .unwrap();

    let script = match id.as_str() {
        "Opt.Debloat" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $apps = @(
                "Microsoft.3DBuilder", "Microsoft.BingWeather", "Microsoft.GetHelp", 
                "Microsoft.Getstarted", "Microsoft.Messaging", "Microsoft.MicrosoftSolitaireCollection", 
                "Microsoft.MixedReality.Portal", "Microsoft.OneConnect", "Microsoft.People", 
                "Microsoft.Print3D", "Microsoft.SkypeApp", "Microsoft.Wallet", "Microsoft.XboxApp", 
                "Microsoft.XboxGameOverlay", "Microsoft.XboxGamingOverlay", "Microsoft.XboxIdentityProvider", 
                "Microsoft.YourPhone", "Microsoft.ZuneMusic", "Microsoft.ZuneVideo"
            )
            foreach ($app in $apps) {
                $manifest = (Get-AppxPackage -AllUsers -Name $app).InstallLocation + "\AppXManifest.xml"
                if (Test-Path $manifest) {
                    Add-AppxPackage -DisableDevelopmentMode -Register $manifest
                }
            }
            Write-Output "Restored package manifests for debloated apps."
            "#
        }
        "Opt.HighPerf" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            powercfg /setactive 381b4222-f694-41f0-9685-ff5bb260df2e
            Write-Output "Balanced power plan activated."
            "#
        }
        "Opt.VisualEffects" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" -Name "VisualFXSetting" -Value 0
            Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "UserPreferencesMask" -Value ([byte[]](158, 30, 7, 128, 18, 0, 0, 0))
            Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "MenuShowDelay" -Value "400"
            Write-Output "Visual effects restored to defaults."
            "#
        }
        "Opt.NetworkTweak" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            netsh int tcp set global autotuninglevel=normal
            netsh int tcp set global chimney=default
            netsh int tcp set global dca=disabled
            netsh int tcp set global netdma=disabled
            netsh int tcp set global ecncapability=disabled
            Write-Output "TCP Auto-Tuning and network settings restored to defaults."
            "#
        }
        "Opt.DisableStartup" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $localApp = $env:LOCALAPPDATA
            if (Test-Path "$localApp\Microsoft\OneDrive\OneDrive.exe") {
                Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "OneDrive" -Value "`"$localApp\Microsoft\OneDrive\OneDrive.exe`" /background"
            }
            Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run" -Name "com.squirrel.Teams.Teams"
            Write-Output "Startup programs re-enabled."
            "#
        }
        "Opt.DarkMode" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" -Name "AppsUseLightTheme" -Value 1
            Set-ItemProperty -Name "SystemUsesLightTheme" -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" -Value 1
            Write-Output "Light Mode re-enabled."
            "#
        }
        "Opt.Telemetry" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-Service -Name "DiagTrack" -StartupType Automatic
            Start-Service -Name "DiagTrack"
            Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry"
            Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection" -Name "AllowTelemetry"
            Write-Output "Telemetry services and Windows tracking re-enabled."
            "#
        }
        "Opt.DefaultBrowserPrompt" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Remove-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Edge" -Name "DefaultBrowserSettingEnabled"
            Remove-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Edge" -Name "HideFirstRunExperience"
            Write-Output "Edge default browser prompts re-enabled."
            "#
        }
        "Opt.DisableHibernate" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            powercfg.exe /hibernate on
            Write-Output "Hibernation re-enabled."
            "#
        }
        "Opt.DisableCortana" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "AllowCortana"
            Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search" -Name "BingSearchEnabled"
            Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Search" -Name "CortanaConsent"
            Write-Output "Cortana and Bing web search re-enabled."
            "#
        }
        "Opt.DisableGameDVR" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled" -Value 1
            Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\GameDVR" -Name "AllowGameDVR"
            Write-Output "Xbox Game DVR re-enabled."
            "#
        }
        "Opt.CloudflareDns" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $adapters = Get-NetAdapter | Where-Object {$_.Status -eq 'Up'}
            foreach ($adapter in $adapters) {
                Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ResetServerAddresses
            }
            Write-Output "DNS settings reset to default."
            "#
        }
        "Opt.DisableIndexing" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Set-Service -Name "WSearch" -StartupType Automatic
            Start-Service -Name "WSearch"
            Write-Output "Windows Search Indexing service re-enabled."
            "#
        }
        "Opt.GodMode" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $DesktopPath = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), 'GodMode.{ED7BA470-8E54-465E-825C-99712043E01C}')
            if (Test-Path $DesktopPath) {
                Remove-Item -Path $DesktopPath -Recurse -Force
                Write-Output "God Mode folder removed."
            } else {
                Write-Output "God Mode folder not found."
            }
            "#
        }
        "Opt.FlushDNS" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            $DesktopPath = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), "Flush DNS.bat")
            if (Test-Path $DesktopPath) {
                Remove-Item -Path $DesktopPath -Force
                Write-Output "Flush DNS shortcut removed."
            } else {
                Write-Output "Flush DNS shortcut not found."
            }
            "#
        }
        "Opt.ScheduleCleanup" => {
            r#"
            $ErrorActionPreference = 'SilentlyContinue'
            Unregister-ScheduledTask -TaskName "WinForgeDailyCleanup" -Confirm:$false
            Write-Output "Scheduled cleanup task removed."
            "#
        }
        _ => "",
    };

    if script.is_empty() {
        window
            .emit(
                "install_progress",
                InstallProgress {
                    app_id: id.clone(),
                    app_name: name.clone(),
                    status: "error".to_string(),
                    message: "This tweak cannot be reverted.".to_string(),
                },
            )
            .unwrap();
        window.emit("install_complete", {}).unwrap();
        return Err("Not reversible".to_string());
    }

    let output = new_hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output();

    match output {
        Ok(out) => {
            let success = out.status.success();
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();

            if success {
                let msg = if stdout.trim().is_empty() {
                    format!("{} reverted successfully", name)
                } else {
                    stdout.trim().to_string()
                };
                window
                    .emit(
                        "install_progress",
                        InstallProgress {
                            app_id: id,
                            app_name: name,
                            status: "done".to_string(),
                            message: msg,
                        },
                    )
                    .unwrap();
            } else {
                let err_msg = if stderr.trim().is_empty() {
                    format!("Failed to revert {}", name)
                } else {
                    format!("Error: {}", stderr.trim())
                };
                window
                    .emit(
                        "install_progress",
                        InstallProgress {
                            app_id: id,
                            app_name: name,
                            status: "error".to_string(),
                            message: err_msg,
                        },
                    )
                    .unwrap();
            }
        }
        Err(e) => {
            window
                .emit(
                    "install_progress",
                    InstallProgress {
                        app_id: id,
                        app_name: name,
                        status: "error".to_string(),
                        message: format!("Error running PowerShell: {}", e),
                    },
                )
                .unwrap();
        }
    }

    window.emit("install_complete", {}).unwrap();
    Ok(())
}


#[tauri::command]
async fn get_installed_apps() -> Vec<String> {
    let output = match new_hidden_command("winget")
        .args(["list", "--accept-source-agreements"])
        .output()
    {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => return Vec::new(),
    };

    let mut installed = Vec::new();
    let words: std::collections::HashSet<&str> = output.split_whitespace().collect();
    for word in words {
        installed.push(word.to_string());
    }
    installed
}

#[tauri::command]
fn close_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command]
fn minimize_window(window: Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn toggle_maximize_window(window: Window) {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            install_apps,
            close_window,
            minimize_window,
            is_admin,
            toggle_maximize_window,
            get_installed_apps,
            get_system_info,
            get_applied_tweaks,
            revert_tweak
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
