use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::runtime::Runtime;
use url::Url;
use crate::commandline::parser::parse_args;
use crate::parser::extractor::StreamExtractor;
use crate::downloader::simple::SimpleDownloadManager;
use crate::downloader::live::LiveDownloadManager;

mod commandline;
mod parser;
mod downloader;
mod entity;
mod crypto;
mod muxer;
mod utils;
mod i18n;

fn main() {
    // 版本信息
    const VERSION_INFO: &str = "N_m3u8DL-RE (Beta version) 20251228";
    
    // 解析命令行参数
    let option = parse_args();
    
    // 初始化日志系统
    if !option.no_log {
        utils::logger::init_logger(option.log_file_path.as_deref(), &option.log_level);
        log::info!("{}", VERSION_INFO);
    } else {
        // 关闭所有日志输出
        log::set_max_level(log::LevelFilter::Off);
        println!("{}", VERSION_INFO);
    }
    
    // 初始化运行时
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 执行主要工作
        if let Err(e) = do_work(option).await {
            eprintln!("错误: {:?}", e);
            std::process::exit(1);
        }
    });
}

async fn do_work(option: commandline::options::MyOption) -> Result<(), Box<dyn std::error::Error>> {
    // 构建HTTP请求头
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36".to_string());
    
    // 添加用户自定义的头
    for (key, value) in &option.headers {
        headers.insert(key.clone(), value.clone());
        if !option.no_log {
            log::info!("User-Defined Header => {}: {}", key, value);
        } else {
            println!("User-Defined Header => {}: {}", key, value);
        }
    }
    
    // 初始化流提取器
    let base_url = option.base_url.or_else(|| {
        // 如果用户没有提供 base_url，尝试从输入中提取
        if option.input.starts_with("http://") || option.input.starts_with("https://") {
            // 从 URL 中提取 base_url
            if let Ok(url) = url::Url::parse(&option.input) {
                let mut base = url.clone();
                base.set_path("");
                base.set_query(None);
                base.set_fragment(None);
                Some(base.to_string())
            } else {
                None
            }
        } else {
            // 从本地文件路径中提取目录作为 base_url
            if let Ok(path) = std::path::Path::new(&option.input).canonicalize() {
                if let Some(parent) = path.parent() {
                    Some(parent.to_string_lossy().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }
    });
    
    let mut extractor = StreamExtractor::new(base_url, headers.clone());
    
    // 加载和解析媒体流
    if !option.no_log {
        log::info!("加载媒体流...");
    } else {
        println!("加载媒体流...");
    }
    extractor.load_source_from_url(&option.input).await?;
    let streams = extractor.extract_streams().await?;
    
    // 分类流
    let mut video_streams = Vec::new();
    let mut audio_streams = Vec::new();
    let mut subtitle_streams = Vec::new();
    
    for stream in &streams {
        match stream.media_type.as_ref().unwrap_or(&entity::stream::MediaType::VIDEO) {
            entity::stream::MediaType::VIDEO => video_streams.push(stream),
            entity::stream::MediaType::AUDIO => audio_streams.push(stream),
            entity::stream::MediaType::SUBTITLES => subtitle_streams.push(stream),
        }
    }
    
    // 显示流信息
    if !option.no_log {
        log::info!("找到 {} 个流: {} 个视频, {} 个音频, {} 个字幕", 
                  streams.len(), video_streams.len(), audio_streams.len(), subtitle_streams.len());
    } else {
        println!("找到 {} 个流: {} 个视频, {} 个音频, {} 个字幕", 
                 streams.len(), video_streams.len(), audio_streams.len(), subtitle_streams.len());
    }
    
    for stream in &streams {
        if !option.no_log {
            log::info!("{}", stream.id);
        } else {
            println!("{}", stream.id);
        }
    }
    
    // 选择流（暂时选择第一个）
    let selected_streams = if streams.is_empty() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "没有找到可下载的流")));
    } else {
        vec![streams[0].clone()]
    };
    
    // 显示选择的流
    if !option.no_log {
        log::info!("已选择的流:");
    } else {
        println!("已选择的流:");
    }
    
    for stream in &selected_streams {
        if !option.no_log {
            log::info!("{}", stream.id);
        } else {
            println!("{}", stream.id);
        }
    }
    
    // 生成临时目录
    let tmp_dir = option.tmp_dir.unwrap_or_else(|| {
        let save_name = option.save_name.clone().unwrap_or_else(|| "download".to_string());
        format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), save_name)
    });
    
    // 下载配置
    let thread_count = option.thread_count;
    
    // 检查是否为直播流
    let is_live = selected_streams.iter().any(|s| s.playlist.as_ref().map(|p| p.is_live).unwrap_or(false));
    
    if is_live && !option.live_perform_as_vod {
        if !option.no_log {
            log::info!("[警告] 检测到直播流");
        } else {
            println!("[警告] 检测到直播流");
        }
    }
    
    // 显示保存名称
    if let Some(save_name) = &option.save_name {
        if !option.no_log {
            log::info!("保存名称: {}", save_name);
        } else {
            println!("保存名称: {}", save_name);
        }
    }
    
    // 开始下载
    let result = if is_live && !option.live_perform_as_vod {
        // 直播下载
        if !option.no_log {
            log::info!("开始直播录制...");
        } else {
            println!("开始直播录制...");
        }
        let live_manager = LiveDownloadManager::new(
            headers,
            thread_count,
            tmp_dir,
            option.live_record_limit,
            option.live_wait_time,
            option.live_take_count
        );
        live_manager.start_record(&selected_streams).await
    } else {
        // 点播下载
        if !option.no_log {
            log::info!("开始下载...");
        } else {
            println!("开始下载...");
        }
        let simple_manager = SimpleDownloadManager::new(
            headers,
            thread_count,
            tmp_dir
        );
        simple_manager.start_download(&selected_streams).await
    };
    
    match result {
        Ok(success) => {
            if success {
                if !option.no_log {
                    log::info!("[成功] 下载完成");
                } else {
                    println!("[成功] 下载完成");
                }
            } else {
                if !option.no_log {
                    log::error!("[失败] 下载失败");
                } else {
                    println!("[失败] 下载失败");
                }
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "下载失败")));
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    
    Ok(())
}
