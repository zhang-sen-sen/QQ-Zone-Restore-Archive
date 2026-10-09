mod archive;
mod qlogin;
mod qzone;

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

/// 便携版运行环境检测：程序应解压后运行，防止在压缩包内直接运行
/// （此时数据会写到压缩包挂载/临时解压目录，造成数据丢失与混乱）。
/// 命中特征：路径含 .zip\.rar\.7z 等压缩包挂载段，或 WinRAR 临时
/// 解压目录（Rar$EX）。
#[tauri::command]
fn portable_zip_runtime_check() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let path = exe.to_string_lossy().to_lowercase();
    let markers = [".zip\\", ".rar\\", ".7z\\", ".zip/", ".rar/", ".7z/", "rar$ex"];
    markers.iter().any(|marker| path.contains(marker))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(archive::ArchiveState::new())
        .manage(qlogin::QLoginState::new())
        .manage(qzone::RecycleAuthState::default())
        .manage(qzone::QzonePageTokenState::default())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 允许 WebView 读取安装目录（数据根目录）下的归档媒体文件（图片/视频）
            #[cfg(desktop)]
            {
                use tauri_plugin_fs::FsExt;
                let root = archive::data_root_dir(app.handle())?;
                app.fs_scope().allow_directory(&root, true)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            exit_app,
            portable_zip_runtime_check,
            qlogin::start_qr_login,
            qlogin::poll_qr_login,
            qlogin::get_login_status,
            qlogin::logout_qzone,
            qlogin::open_web_login,
            qlogin::check_web_login,
            qlogin::sync_cookies_to_webview,
            qzone::fetch_first_feeds,
            qzone::fetch_more_feeds,
            qzone::open_recycle_password_window,
            qzone::prepare_recycle_password_window,
            qzone::check_recycle_password,
            qzone::close_recycle_password_window,
            qzone::list_recycle_albums,
            qzone::list_recycle_photos,
            qzone::load_recycle_photo_preview,
            qzone::list_qzone_albums,
            qzone::create_qzone_album,
            qzone::recover_recycle_album,
            qzone::recover_recycle_photos,
            archive::start_feed_archive,
            archive::get_archive_progress,
            archive::cancel_feed_archive,
            archive::list_archive_skips,
            archive::retry_archive_skip,
            archive::list_archived_feeds,
            archive::list_archive_years,
            archive::list_archived_media,
            archive::get_archived_feed,
            archive::count_archived_feeds,
            archive::export_archived_html,
            archive::load_archived_image,
            archive::load_archived_video,
            archive::download_media_local,
            archive::resolve_library_video,
            archive::get_archive_overview,
            archive::list_interactors,
            archive::list_contact_comment_threads,
            archive::sync_qzone_library,
            archive::list_qzone_library,
            archive::list_qzone_library_years,
            archive::get_interaction_ranking,
            archive::delete_archived_feeds,
            archive::clear_archived_feeds,
            archive::delete_all_app_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
