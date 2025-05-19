// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ ffi::OsStr, fs, os::windows::ffi::OsStrExt, path::Path, process::Command};

use tauri::{api::path::{app_data_dir, resource_dir}, Manager};
use serde::{Serialize, Deserialize};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        UI::Shell::*,
        UI::WindowsAndMessaging::*,
        Foundation::*,
        Graphics::Gdi::*,
    },
};

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
        // format("当前仅支持 Windows 系统")
        ResultWrapper { status: false, data: "".to_string(), info: "当前仅支持 Windows 系统".to_string() }
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


#[tauri::command]
fn get_app_icon(path: &str) -> ResultWrapper<String> {
    if !Path::new(path).exists() {
        return ResultWrapper { 
            status: false, 
            info: format!("路径上的目标不存在: {}", path),
            data:"".to_string()
        };
    }
    match extract_icon(path) {
        Ok(base64_icon) => ResultWrapper {
            status: true,
            info: "ok".to_string(),
            data: base64_icon
        },
        Err(e) => ResultWrapper {
            status: false,
            info: format!("提取图标失败: {}", e),
            data: "".to_string()
        }
    }
}

fn extract_icon(path:&str) ->Result<String, String> {
    unsafe  {
        // 将路径转换为宽字符
        let wide_path: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        // 获取文件图标句柄
        let mut large_icon = HICON(0);
        let mut small_icon = HICON(0);
        let result = ExtractIconExW(
            PCWSTR::from_raw(wide_path.as_ptr()),
            0, // 第一个图标
            &mut large_icon,
            &mut small_icon,
            1, // 我们只提取一个图标
        );
        
        if result <= 0 {
            return Err(format!("无法提取图标，错误码: {}", result));
        }
        
        // 使用大图标（如果有）或小图标
        let icon_handle = if large_icon.0 != 0 { large_icon } else { small_icon };
        if icon_handle.0 == 0 {
            return Err("无法获取图标句柄".to_string());
        }
        
        // 获取图标的信息
        let mut icon_info = ICONINFO::default();
        if !GetIconInfo(icon_handle, &mut icon_info).as_bool() {
            DestroyIcon(icon_handle);
            return Err("无法获取图标信息".to_string());
        }
        
        // 获取位图信息
        let mut bitmap_info = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: 0,
            biHeight: 0,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: 0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };
        
        let hdc = GetDC(HWND(0));
        if hdc.0 == 0 {
            DeleteObject(icon_info.hbmColor);
            DeleteObject(icon_info.hbmMask);
            DestroyIcon(icon_handle);
            return Err("无法获取设备上下文".to_string());
        }
        
        // 获取图标尺寸
        if !GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            0,
            ptr::null_mut(),
            std::mem::transmute(&mut bitmap_info),
            DIB_RGB_COLORS,
        ).as_bool() {
            ReleaseDC(HWND(0), hdc);
            DeleteObject(icon_info.hbmColor);
            DeleteObject(icon_info.hbmMask);
            DestroyIcon(icon_handle);
            return Err("无法获取图标尺寸".to_string());
        }
        
        let width = bitmap_info.biWidth;
        let height = bitmap_info.biHeight.abs();
        
        // 分配内存用于存储图标数据
        let mut buffer = vec![0u8; (width * height * 4) as usize];
        
        // 获取图标数据
        if !GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as u32,
            buffer.as_mut_ptr() as *mut _,
            std::mem::transmute(&mut bitmap_info),
            DIB_RGB_COLORS,
        ).as_bool() {
            ReleaseDC(HWND(0), hdc);
            DeleteObject(icon_info.hbmColor);
            DeleteObject(icon_info.hbmMask);
            DestroyIcon(icon_handle);
            return Err("无法获取图标数据".to_string());
        }
        
        // 清理资源
        ReleaseDC(HWND(0), hdc);
        DeleteObject(icon_info.hbmColor);
        DeleteObject(icon_info.hbmMask);
        DestroyIcon(icon_handle);
        
        // 创建PNG图像并编码为Base64
        let icon_data = convert_rgba_to_png(&buffer, width as u32, height as u32)
            .map_err(|e| format!("PNG转换错误: {}", e))?;
        
        // Base64编码
        let base64_icon = general_purpose::STANDARD.encode(&icon_data);
        Ok(format!("data:image/png;base64,{}", base64_icon))
    }
}

fn convert_rgba_to_png(rgba_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    // 使用image crate将RGBA数据转换为PNG
    // 这里需要添加image crate到您的依赖中
    // 简化版本，实际使用请添加proper error handling
    let mut png_data = Vec::new();
    
    // 这里应该使用image crate处理图像数据
    // 示例伪代码:
    // let img = ImageBuffer::from_raw(width, height, rgba_data.to_vec())
    //    .ok_or("无法创建图像缓冲区")?;
    // img.write_to(&mut png_data, image::ImageFormat::Png)
    //    .map_err(|e| format!("无法编码PNG: {}", e))?;
    
    // 这里是简化实现，实际应用中替换为上面的代码
    png_data.extend_from_slice(rgba_data);
    
    Ok(png_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,launch_app,load_app_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
