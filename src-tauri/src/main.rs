// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::prelude::*;
use num::bigint::BigUint;
use num::traits::{One, Zero};
use std::mem::replace;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

extern crate chrono;

mod modules;

// use modules::main::main::*;
use modules::main::window_menu::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn tst_rust_func() {
    let dt_start = Local::now();
    let time_start = dt_start.timestamp_millis();
    let mut vec = Vec::new();
    // let collect_iter: Vec<i32> = (0..1000000).collect();
    // 空循环
    for x in 0..1000000 {
        vec.push(x)
    }
    let dt_end = Local::now();
    let time_end: i64 = dt_end.timestamp_millis();
    println!("time: {}", (time_end - time_start));
}

#[tauri::command]
fn fibonacci_func(n: usize) -> String {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1; // 注意&
                           // f0 <- f1, f1 <- f2
                           // ! 使用 mem::replace 可以避免无意义的clone
        f0 = replace(&mut f1, f2);
    }
    f0.to_string()
}

#[tauri::command]
fn use_box() {
    let a = Box::new(1);
    let b = Box::new(2);
    println!("a + b = {}", *a + *b);
    println!("a: {}", a);
    println!("b: {}", b);
}

fn main() {
    let context = tauri::generate_context!();
    let menu = create_menu(&context);
    tauri::Builder::default()
        // 将菜单添加到所有窗口上
        .menu(menu)
        .on_menu_event(emit_menu_event)
        .invoke_handler(tauri::generate_handler![
            greet,
            tst_rust_func,
            fibonacci_func,
            use_box
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::{fibonacci_func, use_box};

    #[test]
    fn it_works() {
        let result = fibonacci_func(50);
        assert_eq!(result, "12586269025");
    }
    #[test]
    fn box_works() {
        use_box();
        assert_eq!(true, true);
    }
}
