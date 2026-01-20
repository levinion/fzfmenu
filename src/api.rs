use anyhow::Result;
use itertools::Itertools;

pub fn call_actions(actions: impl IntoIterator<Item = String>) {
    println!(
        "{}",
        actions
            .into_iter()
            .filter(|action| !action.is_empty())
            .join("+")
    );
}

pub fn reload() -> Result<String> {
    let exe = std::env::current_exe()?.to_str().unwrap().to_owned();
    Ok(format!("reload({} _picker)", exe))
}

pub fn change_border_label(label: impl AsRef<str>) -> String {
    format!("change-border-label({})", label.as_ref())
}
