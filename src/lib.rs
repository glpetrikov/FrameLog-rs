/* =================================================
 * FrameLog-rs, MIT - License
 * ─────────────────────────────────────────────────
 * FrameLog-rs
 * lib.rs
 * ─────────────────────────────────────────────────
 * Updated on:
 * 2025.12.5
 * ─────────────────────────────────────────────────
 * Made by:
 * Gleb Petrikov
 * ─────────────────────────────────────────────────
 * Description:
 * main lib file
 * =================================================
 */

/// Main logger, prints messages to the console with colors.
pub mod logger;

/// Working with file-based log output.
pub mod filehandler;

/// Buffer for accumulating messages before output.
pub mod buffer;

/// Logger that writes messages to a buffer.
pub mod buf_logger;
