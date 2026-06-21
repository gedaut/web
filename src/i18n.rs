// src/i18n.rs

// Tento makro-příkaz doslova vloží vygenerovaný kód z build.rs
include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

// Zpřístupníme vygenerované věci (včetně makra t!) pro zbytek aplikace
pub use i18n::*;