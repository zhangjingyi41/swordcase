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

use base64::{Engine, engine::general_purpose};
use image::{ImageBuffer, Rgba};

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
        
        // 修复这里: 用 Some() 包装指针
        let result = ExtractIconExW(
            PCWSTR::from_raw(wide_path.as_ptr()),
            0, // 第一个图标
            Some(&mut large_icon as *mut _),
            Some(&mut small_icon as *mut _),
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
        let size_result = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            0,
            None,  // 这里使用 None 因为我们只是查询尺寸
            std::mem::transmute(&mut bitmap_info),
            DIB_RGB_COLORS,
        );
        println!("size_result: {}", size_result);
        println!("bitmap_info: width={}, height={}", bitmap_info.biWidth, bitmap_info.biHeight);

        if size_result == 0 || bitmap_info.biWidth == 0 || bitmap_info.biHeight == 0 {
            // 如果返回值为0或尺寸为0，尝试使用固定尺寸
            println!("使用默认图标尺寸");
            bitmap_info.biWidth = 32;  // 使用常见的图标大小
            bitmap_info.biHeight = 32;
            // 也可以尝试使用 GetIconInfo 检索到的位图的尺寸
            // 这里也可以考虑获取系统图标大小：SM_CXICON 和 SM_CYICON
        }

        let width = bitmap_info.biWidth;
        let height = bitmap_info.biHeight.abs(); // 使用绝对值，因为 biHeight 可能为负
        
        // 分配内存用于存储图标数据
        let mut buffer = vec![0u8; (width * height * 4) as usize];
        
        // 获取图标数据
        let ico_result = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height as u32,
            Some(buffer.as_mut_ptr() as *mut _),  // 将参数包装在 Some 中
            std::mem::transmute(&mut bitmap_info),
            DIB_RGB_COLORS,
        );
        if ico_result == 0 {  // 检查返回值是否为0
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
    // 创建一个新的缓冲区用于翻转的图像数据
    let mut fixed_data = Vec::with_capacity(rgba_data.len());
    
    // 翻转图像垂直方向 (上下颠倒)
    for y in (0..height).rev() {  // 反向遍历行
        for x in 0..width {
            let pos = ((y * width + x) * 4) as usize;
            if pos + 3 < rgba_data.len() {
                // 同时交换 BGR 与 RGB
                fixed_data.push(rgba_data[pos + 2]);  // B -> R
                fixed_data.push(rgba_data[pos + 1]);  // G -> G
                fixed_data.push(rgba_data[pos]);     // R -> B
                fixed_data.push(rgba_data[pos + 3]);  // A -> A
            }
        }
    }
    
    // 使用修复后的数据创建图像
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, fixed_data)
        .ok_or("无法创建图像缓冲区")?;
    
    // 转换为 DynamicImage
    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    
    // 将图像编码为PNG
    let mut png_data = Vec::new();
    dynamic_img.write_to(&mut std::io::Cursor::new(&mut png_data), image::ImageFormat::Png)
        .map_err(|e| format!("无法编码PNG: {}", e))?;
    
    Ok(png_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,launch_app,load_app_list,get_app_icon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
