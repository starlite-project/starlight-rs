use star_lang::*;

fn main() -> LanguageResult<()> {
    let lang = I18nMap::from_dir("./languages").unwrap();

    let english = lang.get("en_us").unwrap();

    dbg!(english.clone());

    let ping = english.get("ping").unwrap();

    dbg!(ping.clone());

    dbg!(ping.run().unwrap());

    let pong = english.get("pong").unwrap();

    dbg!(pong.clone());

    dbg!(pong.run_params(&["10"]).unwrap());

    dbg!(i18n!(lang, "en_us", "ping"));

    Ok(())
}
