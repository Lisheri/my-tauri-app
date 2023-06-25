use tauri::api::dialog::message;
use tauri::utils::assets::EmbeddedAssets;
use tauri::{AboutMetadata, Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

fn get_app_menu(name: &String) -> Submenu {
    Submenu::new(
        "",
        // MenuItem::About 为原生菜单
        Menu::new()
            .add_native_item(MenuItem::About(name.into(), AboutMetadata::new()))
            .add_native_item(MenuItem::Copy),
    )
}

fn get_options_menu() -> Submenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    // 创建菜单, 名字叫做 file, 同时添加上面两个item到菜单中
    Submenu::new(
        "操作",
        Menu::new().add_item(quit).add_item(close)
    )
}

fn get_file_menu() -> Submenu {
    Submenu::new(
        "文件",
        Menu::new()
            .add_item(CustomMenuItem::new("new_file".to_string(), "新建"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "编辑")),
    )
}

// 创建 menu 实例
pub fn create_menu(context: &Context<EmbeddedAssets>) -> Menu {
    // 应用名称
    let name = &context.package_info().name;
    Menu::new()
        .add_submenu(get_app_menu(name))
        .add_submenu(get_file_menu())
        .add_submenu(get_options_menu())
        .add_submenu(Submenu::new("选择", Menu::new()))
}

pub fn emit_menu_event(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let event_window = event.window();
    let win = Some(event_window);
    match event.menu_item_id() {
        "new_file" => {
            // debug 信息（终端输出）
            dbg!("new file");
        }
        "edit_file" => {
            // 发送信息到菜单所属窗口（弹窗形式）
            message(win, "编辑文件", "TODO");
        }
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
          event_window.close().unwrap();
        }
        // "hide" => {
        //     event.window().hide().unwrap();
        // },
        // "show" => {
        //   event.window().show().unwrap();
        // },
        _ => {}
    }
}
