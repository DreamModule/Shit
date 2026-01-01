use std::ptr::null_mut;
use std::ffi::OsString;
use std::os::windows::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs::{self, File, OpenOptions};
use std::path::Path;

use winapi::um::winreg::{HKEY_LOCAL_MACHINE, RegOpenKeyExW, RegSetValueExW, RegCloseKey, RegQueryValueExW, RegEnumKeyExW};
use winapi::um::winnt::{KEY_SET_VALUE, KEY_READ, REG_SZ, KEY_WOW64_64KEY};
use winapi::um::minwindef::HKEY;
use winapi::um::securitybaseapi::IsUserAnAdmin;
use winapi::um::sysinfoapi::{GetVersionExW, OSVERSIONINFOW};

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use uuid::Uuid;
use chrono::Local;
use serde::{Deserialize, Serialize};
use regex::Regex;

// --- Logging ---

#[allow(dead_code)]
enum LogLevel { Info, Success, Warning, Error }

fn log_message(level: LogLevel, message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let (prefix, console_prefix) = match level {
        LogLevel::Info => ("[INFO]", "  [INFO]"),
        LogLevel::Success => ("[OK]", "✓"),
        LogLevel::Warning => ("[WARN]", "⚠"),
        LogLevel::Error => ("[ERR]", "✗"),
    };
    
    let log_line = format!("{} {} {}", timestamp, prefix, message);
    
    match level {
        LogLevel::Success => println!("{} {}", console_prefix, message),
        LogLevel::Error => eprintln!("{} {}", console_prefix, message),
        _ => println!("{} {}", console_prefix, message),
    }

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("hwid_spoofer.log") {
        writeln!(file, "{}", log_line).ok();
    }
}

// --- Utilities ---

fn gen_alphanumeric(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

fn gen_realistic_serial() -> String {
    let brands = vec!["DELL", "HP", "LENOVO", "ASUS", "MSI", "ACER", "GIGABYTE"];
    let brand = brands[thread_rng().gen_range(0..brands.len())];
    
    match brand {
        "DELL" => format!("{}-{}", gen_alphanumeric(7), gen_alphanumeric(5)),
        "HP" => gen_alphanumeric(13),
        "LENOVO" => format!("L{}{}", gen_alphanumeric(9), thread_rng().gen_range(1000..9999)),
        "ASUS" => format!("{}BK{}", gen_alphanumeric(6), gen_alphanumeric(4)),
        _ => gen_alphanumeric(12),
    }
}

fn gen_realistic_pc_name() -> String {
    let adjectives = vec!["GAMING", "OFFICE", "HOME", "WORK", "DEV"];
    let models = vec!["PRO", "ELITE", "DESKTOP", "STATION", "PC"];
    
    format!("DESKTOP-{}-{}-{}", 
        adjectives[thread_rng().gen_range(0..adjectives.len())],
        models[thread_rng().gen_range(0..models.len())],
        gen_alphanumeric(5)
    )
}

fn key_exists(path: &str, value_name: &str) -> bool {
    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_value: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut hkey: HKEY = null_mut();
    
    unsafe {
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, wide_path.as_ptr(), 0, KEY_READ | KEY_WOW64_64KEY, &mut hkey) != 0 {
            return false;
        }
        
        let mut data_type = 0u32;
        let mut data_size = 0u32;
        
        let exists = RegQueryValueExW(
            hkey,
            wide_value.as_ptr(),
            null_mut(),
            &mut data_type,
            null_mut(),
            &mut data_size
        ) == 0;
        
        RegCloseKey(hkey);
        exists
    }
}

fn enum_registry_subkeys(path: &str) -> Result<Vec<String>, String> {
    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let mut hkey: HKEY = null_mut();
    let mut subkeys = Vec::new();
    
    unsafe {
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, wide_path.as_ptr(), 0, KEY_READ | KEY_WOW64_64KEY, &mut hkey) != 0 {
            return Err("Cannot open key".to_string());
        }
        
        let mut index = 0u32;
        loop {
            let mut name_buf = vec![0u16; 256];
            let mut name_len = 256u32;
            
            let result = RegEnumKeyExW(
                hkey,
                index,
                name_buf.as_mut_ptr(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut()
            );
            
            if result != 0 {
                break;
            }
            
            name_buf.truncate(name_len as usize);
            subkeys.push(String::from_utf16_lossy(&name_buf).trim_matches(char::from(0)).to_string());
            index += 1;
        }
        
        RegCloseKey(hkey);
    }
    
    Ok(subkeys)
}

// --- Registry Core ---

fn read_reg_key(path: &str, value_name: &str) -> Result<String, ()> {
    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_value_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut hkey: HKEY = null_mut();
    
    unsafe {
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, wide_path.as_ptr(), 0, KEY_READ | KEY_WOW64_64KEY, &mut hkey) != 0 {
            return Err(());
        }

        let mut data_type = 0;
        let mut data_size: u32 = 0;
        
        if RegQueryValueExW(hkey, wide_value_name.as_ptr(), null_mut(), &mut data_type, null_mut(), &mut data_size) != 0 {
            RegCloseKey(hkey);
            return Err(());
        }
        
        if data_type != REG_SZ {
            RegCloseKey(hkey);
            return Err(());
        }

        let mut data_buf: Vec<u16> = vec![0; (data_size / 2) as usize];
        if RegQueryValueExW(hkey, wide_value_name.as_ptr(), null_mut(), null_mut(), data_buf.as_mut_ptr() as *mut u8, &mut data_size) != 0 {
            RegCloseKey(hkey);
            return Err(());
        }

        RegCloseKey(hkey);
        
        let result = String::from_utf16_lossy(&data_buf)
            .trim_matches(char::from(0))
            .to_string();
            
        Ok(result)
    }
}

fn set_reg_key_wide(path: &str, value_name: &str, data: &str, dry_run: bool) -> Result<(), String> {
    if dry_run {
        log_message(LogLevel::Info, &format!("[DRY-RUN] Imitate write: {}\\{} = \"{}\"", path, value_name, data));
        return Ok(());
    }
    
    if let Ok(val) = read_reg_key(path, value_name) {
        log_message(LogLevel::Info, &format!("  (Old) {} = \"{}\"", value_name, val));
    }

    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_value: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
    let wide_data: Vec<u16> = data.encode_utf16().chain(std::iter::once(0)).collect();
    let data_len = (wide_data.len() * 2) as u32; 

    let mut hkey: HKEY = null_mut();
    
    unsafe {
        let open_result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            wide_path.as_ptr(),
            0,
            KEY_SET_VALUE | KEY_WOW64_64KEY,
            &mut hkey
        );

        if open_result != 0 {
            return Err(format!("RegOpenKeyExW failed: {}", open_result));
        }

        let set_result = RegSetValueExW(
            hkey,
            wide_value.as_ptr(),
            0,
            REG_SZ,
            wide_data.as_ptr() as *const u8,
            data_len
        );

        RegCloseKey(hkey);

        if set_result != 0 {
            return Err(format!("RegSetValueExW failed: {}", set_result));
        }
    }

    Ok(())
}

fn set_reg_key_safe(path: &str, value_name: &str, data: &str, dry_run: bool) {
    if !dry_run && !key_exists(path, value_name) {
        log_message(LogLevel::Warning, &format!("Key not exist: {}\\{}", path, value_name));
        return;
    }
    
    match set_reg_key_wide(path, value_name, data, dry_run) {
        Ok(_) => log_message(LogLevel::Success, &format!("{}\\{} -> \"{}\"", path, value_name, data)),
        Err(e) => {
            log_message(LogLevel::Error, &format!("Update failed {}\\{}: {}", path, value_name, e));
            // Warning about protected keys
            if !dry_run {
                log_message(LogLevel::Warning, &format!("Key {} may be TrustedInstaller-protected. Manual ownership required.", path));
            }
        }
    }
}

// --- Spoofing Functions ---

fn spoof_basic_ids(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: Basic IDs");

    let machine_guid = Uuid::new_v4().to_string().to_uppercase();
    set_reg_key_safe("SOFTWARE\\Microsoft\\Cryptography", "MachineGuid", &machine_guid, dry_run);
    
    let hw_guid = format!("{{{}}}", Uuid::new_v4().to_string().to_uppercase());
    set_reg_key_safe("SYSTEM\\CurrentControlSet\\Control\\IDConfigDB\\Hardware Profiles\\0001", "HwProfileGuid", &hw_guid, dry_run);
    
    let install_guid = Uuid::new_v4().to_string().to_uppercase();
    set_reg_key_safe("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "InstallID", &install_guid, dry_run);

    let prod_id = format!("{}-{}-{}-{}-{}", 
        gen_alphanumeric(5), gen_alphanumeric(5), gen_alphanumeric(5), 
        gen_alphanumeric(5), gen_alphanumeric(5)
    );
    set_reg_key_safe("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "ProductId", &prod_id, dry_run);
    set_reg_key_safe("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "BuildGuid", &Uuid::new_v4().to_string().to_uppercase(), dry_run);
    
    Ok(())
}

fn spoof_bios(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: BIOS/CPU");
    
    // UEFI/Secure Boot warning
    log_message(LogLevel::Warning, "!!! WARNING: BIOS keys (HARDWARE\\DESCRIPTION\\System\\BIOS) may fail if UEFI Secure Boot is enabled. !!!");

    let bios_serial = gen_realistic_serial();
    set_reg_key_safe("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemSerialNumber", &bios_serial, dry_run);
    set_reg_key_safe("HARDWARE\\DESCRIPTION\\System\\BIOS", "BaseBoardSerialNumber", &gen_realistic_serial(), dry_run);
    set_reg_key_safe("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemProductName", &format!("Desktop-{}", gen_alphanumeric(6)), dry_run);
    
    let cpus = vec![
        "Intel(R) Core(TM) i9-14900KF CPU @ 5.00GHz",
        "Intel(R) Core(TM) i7-13700K CPU @ 4.50GHz",
        "AMD Ryzen 9 7950X 16-Core Processor",
        "AMD Ryzen 7 7800X3D 8-Core Processor",
    ];
    let cpu = cpus[thread_rng().gen_range(0..cpus.len())];
    set_reg_key_safe("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "ProcessorNameString", cpu, dry_run);
    set_reg_key_safe("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "Identifier", &gen_alphanumeric(12), dry_run);

    Ok(())
}

fn spoof_network_adapters(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: MAC-Addresses");
    let adapters_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";
    
    let subkeys = enum_registry_subkeys(adapters_path)?;

    for subkey in subkeys {
        if subkey.len() == 4 && subkey.chars().all(|c| c.is_ascii_digit()) { 
            let full_path = format!("{}\\{}", adapters_path, subkey);
            
            if key_exists(&full_path, "DriverDesc") {
                let mut mac_bytes = [0u8; 6];
                thread_rng().fill(&mut mac_bytes);
                mac_bytes[0] = (mac_bytes[0] & 0xFE) | 0x02; 
                
                let mac = format!("{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                    mac_bytes[0], mac_bytes[1], mac_bytes[2],
                    mac_bytes[3], mac_bytes[4], mac_bytes[5]
                );
                
                set_reg_key_safe(&full_path, "NetworkAddress", &mac, dry_run);
            }
        }
    }
    
    if !dry_run {
        Command::new("ipconfig").args(&["/flushdns"]).output().ok();
        Command::new("arp").args(&["-d", "*"]).output().ok();
        log_message(LogLevel::Info, "DNS/ARP cache flushed.");
    }
    
    Ok(())
}

fn spoof_usb_devices(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: USB Serials/ContainerIDs");
    let usbstor_path = "SYSTEM\\CurrentControlSet\\Enum\\USBSTOR";
    
    let devices = enum_registry_subkeys(usbstor_path)?;
    
    for device in devices {
        let device_path = format!("{}\\{}", usbstor_path, device);
        let instances = enum_registry_subkeys(&device_path)?;
        
        for instance in instances {
            let instance_path = format!("{}\\{}", device_path, instance);
            
            let new_id = gen_alphanumeric(8).to_uppercase();
            set_reg_key_safe(&instance_path, "ParentIdPrefix", &new_id, dry_run);
            
            let container_id = format!("{{{}}}", Uuid::new_v4().to_string().to_uppercase());
            set_reg_key_safe(&instance_path, "ContainerID", &container_id, dry_run);
            
            let new_desc = format!("Generic USB Device {}", gen_alphanumeric(4));
            set_reg_key_safe(&instance_path, "DeviceDesc", &new_desc, dry_run);
        }
    }
    
    Ok(())
}

fn spoof_computer_name(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: Computer Name");
    
    let new_pc_name = gen_realistic_pc_name().to_uppercase();
    set_reg_key_safe("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName", "ComputerName", &new_pc_name, dry_run);
    set_reg_key_safe("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName", "ComputerName", &new_pc_name, dry_run);
    
    Ok(())
}

fn spoof_volume_serial(dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, "Start: Volume Serial (Needs VolumeID.exe)");

    if !Path::new("volumeid.exe").exists() {
        log_message(LogLevel::Warning, "volumeid.exe not found. Skipping.");
        return Err("volumeid.exe missing".to_string());
    }
    
    let new_serial = format!("{:04X}-{:04X}", 
        thread_rng().gen::<u16>(),
        thread_rng().gen::<u16>()
    );

    if dry_run {
        log_message(LogLevel::Info, &format!("[DRY-RUN] Imitate VolumeID C: {}", new_serial));
        return Ok(());
    }
    
    let output = Command::new("volumeid.exe")
        .args(&["C:", &new_serial])
        .output();
    
    match output {
        Ok(out) => {
            if out.status.success() {
                log_message(LogLevel::Success, &format!("Volume Serial C: changed to {}", new_serial));
                Ok(())
            } else {
                let error_msg = String::from_utf8_lossy(&out.stderr);
                log_message(LogLevel::Error, &format!("VolumeID.exe failed: {}", error_msg));
                Err("VolumeID.exe failed".to_string())
            }
        },
        Err(e) => {
            log_message(LogLevel::Error, &format!("Error executing VolumeID.exe: {}", e));
            Err("VolumeID.exe execution error".to_string())
        }
    }
}

fn backup_registry_keys() -> Result<String, String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_file = format!("registry_backup_{}.txt", timestamp);
    
    let mut file = File::create(&backup_file)
        .map_err(|e| format!("Cannot create backup: {}", e))?;
    
    let keys_to_backup = vec![
        ("SOFTWARE\\Microsoft\\Cryptography", "MachineGuid"),
        ("SYSTEM\\CurrentControlSet\\Control\\IDConfigDB\\Hardware Profiles\\0001", "HwProfileGuid"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemSerialNumber"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "BaseBoardSerialNumber"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemProductName"),
        ("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "ProductId"),
        ("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "BuildGuid"),
        ("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "InstallID"), 
        ("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName", "ComputerName"),
        ("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "ProcessorNameString"), 
        ("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "Identifier"), 
        ("SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}\\0000", "NetworkAddress"),
    ];
    
    writeln!(file, "=== HWID Backup {} ===\n", timestamp).ok();
    
    for (path, value_name) in keys_to_backup {
        match read_reg_key(path, value_name) {
            Ok(value) => {
                writeln!(file, "[{}]", path).ok();
                let safe_value = value.replace('\\', "\\\\").replace('\"', "\\\"");
                writeln!(file, "\"{}\"=\"{}\"", value_name, safe_value).ok();
                writeln!(file, "").ok();
            }
            Err(_) => {
                writeln!(file, "; Cannot read: {}\\{}", path, value_name).ok();
            }
        }
    }
    
    log_message(LogLevel::Success, &format!("Backup created: {}", backup_file));
    Ok(backup_file)
}

fn restore_from_backup(backup_file: &str) -> Result<(), String> {
    log_message(LogLevel::Info, &format!("Start: Restoring from {}", backup_file));
    let content = fs::read_to_string(backup_file)
        .map_err(|e| format!("Error reading backup: {}", e))?;
    
    let re_path = Regex::new(r"\[(.+)\]").unwrap();
    let re_value = Regex::new(r#""([^"]+)"="(.*)""#).unwrap(); 
    
    let mut current_path = String::new();
    
    for line in content.lines() {
        if let Some(caps) = re_path.captures(line) {
            current_path = caps[1].to_string();
        } else if let Some(caps) = re_value.captures(line) {
            let value_name = caps[1].to_string();
            let data = caps[2].to_string().replace("\\\"", "\"").replace("\\\\", "\\");
            
            match set_reg_key_wide(&current_path, &value_name, &data, false) {
                Ok(_) => log_message(LogLevel::Success, &format!("Restored: {}\\{}", current_path, value_name)),
                Err(e) => log_message(LogLevel::Error, &format!("Restore failed {}\\{}: {}", current_path, value_name, e)),
            }
        }
    }
    
    log_message(LogLevel::Success, "Restore completed.");
    Ok(())
}

fn detect_vm() -> bool {
    log_message(LogLevel::Info, "Start: VM Detection");
    
    let vm_indicators = vec![
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer", "VMware"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer", "VirtualBox"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemProductName", "Virtual Machine"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer", "QEMU"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer", "Xen"),
        ("HARDWARE\\DEVICEMAP\\Scsi\\Scsi Port 0\\Scsi Bus 0\\Target Id 0\\Logical Unit Id 0", "Identifier", "VBOX"),
        ("SYSTEM\\CurrentControlSet\\Services\\vmicheartbeat", "DisplayName", "Hyper-V"),
    ];
    
    let mut vm_detected = false;
    
    for (path, value, expected) in vm_indicators {
        if let Ok(actual) = read_reg_key(path, value) {
            if actual.to_lowercase().contains(&expected.to_lowercase()) {
                log_message(LogLevel::Warning, &format!("VM detected: {} in {}", expected, path));
                vm_detected = true;
            }
        }
    }
    
    if !vm_detected {
        log_message(LogLevel::Success, "No VM detected.");
    }
    
    vm_detected
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Config {
    backup_before: bool,
    spoof_basic_ids: bool,
    spoof_bios: bool,
    spoof_mac: bool,
    spoof_usb: bool,
    spoof_computer_name: bool,
    spoof_volume_serial: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            backup_before: true,
            spoof_basic_ids: true,
            spoof_bios: true,
            spoof_mac: true,
            spoof_usb: true,
            spoof_computer_name: true,
            spoof_volume_serial: false,
        }
    }
}

fn load_config() -> Config {
    match fs::read_to_string("config.toml") {
        Ok(content) => toml::from_str(&content).unwrap_or_else(|e| {
            log_message(LogLevel::Error, &format!("Config parse error: {}. Using defaults.", e));
            Config::default()
        }),
        Err(_) => {
            let config = Config::default();
            if let Ok(toml_str) = toml::to_string(&config) {
                fs::write("config.toml", toml_str).ok();
            }
            config
        }
    }
}

fn validate_config(mut config: Config) -> Config {
    if config.spoof_volume_serial {
        if !Path::new("volumeid.exe").exists() {
            log_message(LogLevel::Warning, "volumeid.exe not found. Disabling volume serial spoof.");
            config.spoof_volume_serial = false;
        }
    }
    config
}


fn verify_changes() -> Result<(), String> {
    log_message(LogLevel::Info, "Start: Verify Changes");
    
    let checks = vec![
        ("SOFTWARE\\Microsoft\\Cryptography", "MachineGuid"),
        ("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName", "ComputerName"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemSerialNumber"),
        ("HARDWARE\\DESCRIPTION\\System\\BIOS", "BaseBoardSerialNumber"), 
        ("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", "ProductId"), 
    ];
    
    let mut success_count = 0;
    
    for (path, value_name) in checks {
        match read_reg_key(path, value_name) {
            Ok(value) => {
                log_message(LogLevel::Success, &format!("Check {}: {}", value_name, value));
                success_count += 1;
            }
            Err(_) => {
                log_message(LogLevel::Error, &format!("Check {}: FAILED/NOT FOUND", value_name));
            }
        }
    }
    
    log_message(LogLevel::Info, &format!("Verification Success: {}/{}", success_count, checks.len()));
    Ok(())
}

fn spoof_all(config: &Config, dry_run: bool) -> Result<(), String> {
    log_message(LogLevel::Info, &format!("Start Full Spoof (Dry Run: {})", dry_run));
    
    if config.spoof_basic_ids { spoof_basic_ids(dry_run)?; }
    if config.spoof_bios { spoof_bios(dry_run)?; }
    if config.spoof_mac { spoof_network_adapters(dry_run)?; }
    if config.spoof_usb { spoof_usb_devices(dry_run)?; }
    if config.spoof_computer_name { spoof_computer_name(dry_run)?; }
    if config.spoof_volume_serial { spoof_volume_serial(dry_run)?; }

    log_message(LogLevel::Success, "Full spoof operation finished.");
    Ok(())
}


// --- Entry Point ---

fn main() -> Result<(), String> {
    
    // Admin check
    unsafe {
        if IsUserAnAdmin() == 0 {
            log_message(LogLevel::Error, "Script stopped: Administrator privileges required!");
            return Err("No admin rights".to_string());
        }
    }
    
    let config_raw = load_config();
    let config = validate_config(config_raw);

    println!("\n╔════════════════════════════════════════╗");
    println!("║     HWID Spoofer (Survival) v2.1       ║");
    println!("╚════════════════════════════════════════╝");
    
    detect_vm(); 
    println!("\nCurrent Config: {:?}", config);
    println!("\nSelect operation:");
    println!("  [1] Full Spoof (from config)");
    println!("  [2] Test Mode (Dry-Run)");
    println!("  [3] Restore from Backup");
    println!("  [4] Verify Changes");
    println!("  [0] Exit");
    print!("\nYour choice: ");
    
    let mut input = String::new();
    io::stdout().flush().ok();
    io::stdin().read_line(&mut input).ok();

    match input.trim() {
        "1" => {
            if config.backup_before {
                backup_registry_keys()?;
            }
            spoof_all(&config, false)?;
            verify_changes()?;
            
            println!("\n!!! REBOOT REQUIRED for changes to take effect !!!");
            print!("Reboot now? (y/N): ");
            let mut reboot_input = String::new();
            io::stdout().flush().ok();
            io::stdin().read_line(&mut reboot_input).ok();

            if reboot_input.trim().to_lowercase() == "y" {
                auto_reboot_countdown(10)?;
            }
        },
        "2" => {
            spoof_all(&config, true)?;
        },
        "3" => {
            print!("Enter backup filename (registry_backup_...txt): ");
            let mut backup_file = String::new();
            io::stdout().flush().ok();
            io::stdin().read_line(&mut backup_file).ok();
            restore_from_backup(backup_file.trim())?;
        },
        "4" => {
            verify_changes()?;
        },
        "0" => {
            log_message(LogLevel::Info, "User exit.");
            return Ok(());
        },
        _ => {
            log_message(LogLevel::Error, "Invalid choice!");
            return Ok(());
        }
    }
    
    Ok(())
}

fn auto_reboot_countdown(seconds: u64) -> Result<(), String> {
    
    ctrlc::set_handler(move || {
        println!("\n❌ Reboot aborted by user");
        std::process::exit(0);
    }).map_err(|e| format!("Ctrl+C handler error: {}", e))?;
    
    println!("\n⏰ System will reboot in {} seconds...", seconds);
    println!("   Press Ctrl+C to abort");
    
    for i in (1..=seconds).rev() {
        print!("\r   Reboot in: {} sec...   ", i);
        io::stdout().flush().ok();
        thread::sleep(Duration::from_secs(1));
    }
    
    println!("\n🔄 Rebooting...");
    Command::new("shutdown")
        .args(&["/r", "/t", "0"])
        .spawn()
        .map_err(|e| format!("Shutdown error: {}", e))?;
        
    Ok(())
}
