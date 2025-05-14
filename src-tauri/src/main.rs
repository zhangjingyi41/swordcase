// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ fs, path::Path, process::Command};

use tauri::{api::path::{app_data_dir, resource_dir}, Manager};

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn launch_app(path: &str) -> String {
    if !Path::new(path).exists() {
        return format!("Path does not exist: {}", path);
    }
    match Command::new(path).spawn() {
        Ok(_) => format!("ok"),
        Err(e) => format!("Failed to start application: {}", e),
    }

}

// v2 版本 泛用性更强，可以打开可执行程序、文件、快捷方式等
#[tauri::command]
fn launch_app_v2(path: &str) -> String {
    if !Path::new(path).exists() {
        return format!("Path does not exist: {}", path);
    }
    use std::process::Command;
    // 根据文件类型选择默认应用打开
    #[cfg(target_os = "windows")]
    {
        match Command::new("cmd").args(&["/C", &path]).spawn()
        {
            Ok(_) => format!("已成功打开文件: {}", path),
            Err(e) => format!("打开文件失败: {}", e)
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Err("当前仅支持 Windows 系统".to_string())
        format("当前仅支持 Windows 系统")
    }
}

// 加载data文件夹中的data.json，读取为字符串
#[tauri::command]
fn load_app_list(handle:tauri::AppHandle)->String {
    let resource_path = handle.path_resolver().resolve_resource("data/app_list.json").expect("Failed to resolve resource path");
    if resource_path.exists() {
        return match fs::read_to_string(resource_path) {
            Ok(content) => content,
            Err(e) => format!("Failed to read file: {}", e),
        }
    }else{
        return format!("File not found: {}", resource_path.display());
    }    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,launch_app,launch_app_v2,load_app_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
