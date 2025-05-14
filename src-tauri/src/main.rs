// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ fs, path::Path, process::Command};

use tauri::{api::path::{app_data_dir, resource_dir}, Manager};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResultWrapper<T> {
    pub status:bool,
    pub data:T,
    pub info:String,
}

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// v2 版本 泛用性更强，可以打开可执行程序、文件、快捷方式等
#[tauri::command]
fn launch_app(path: &str) -> ResultWrapper<String> {
    if !Path::new(path).exists() {
        return ResultWrapper { 
            status: false, 
            info: format!("路径上的目标不存在: {}", path),
            data:"".to_string()
        };
    }
    use std::process::Command;
    // 根据文件类型选择默认应用打开
    #[cfg(target_os = "windows")]
    {
        match Command::new("cmd").args(&["/C", &path]).spawn()
        {
            Ok(_) => ResultWrapper { 
                status: true, 
                info: "ok".to_string() ,
                data:"".to_string()
            },
            Err(e) => ResultWrapper{
                status:false,
                info:format!("打开文件失败: {}", e),
                data:"".to_string()
            },
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
fn load_app_list(handle:tauri::AppHandle)->ResultWrapper<String> {
    let resource_path = handle.path_resolver().resolve_resource("data/app_list.json").expect("Failed to resolve resource path");
    if resource_path.exists() {
        return match fs::read_to_string(resource_path) {
            Ok(content) => ResultWrapper {
                status: true,
                info: "ok".to_string(),
                data: content,
            },
            Err(e) => ResultWrapper {
                status: false,
                info: format!("读取文件失败: {}", e),
                data: "".to_string(),
            },
        }
    }else{
        return ResultWrapper {
            status: false,
            info: format!("路径上的目标不存在: {}", resource_path.display()),
            data: "".to_string(),
        };
    }    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,launch_app,load_app_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
