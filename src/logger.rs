/* =================================================
 * FrameLog-rs, MIT - License
 * ─────────────────────────────────────────────────
 * FrameLog-rs
 * logger.rs
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.4
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * Logger
 * =================================================
 */

use colored::*;

pub struct Logger;
impl Logger {
    pub fn trace(message: &str){
        println!("{}", message.bright_black());
    }
    pub fn info(message: &str){
        println!("{}", message.green());
    }
    pub fn warn(message: &str){
        println!("{}", message.yellow());
    }
    pub fn error(message: &str){
        println!("{}", message.red());
    }
    pub fn fatal(message: &str){
        println!("{}", message.bright_red());
    }
}