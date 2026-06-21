use leptos_i18n_build::{Config, TranslationsInfos};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=locales");

    let out = PathBuf::from(std::env::var("OUT_DIR")?).join("i18n");

    let cfg = Config::new("cs")?
        .add_locale("en")?.add_namespaces(["layout", "home"])?;

    let infos = TranslationsInfos::parse(cfg)?;
    infos.emit_diagnostics();
    infos.rerun_if_locales_changed();
    infos.generate_i18n_module(out)?;

    Ok(())
}