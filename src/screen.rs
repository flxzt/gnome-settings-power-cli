use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.gnome.SettingsDaemon.Power.Screen",
    default_service = "org.gnome.SettingsDaemon.Power",
    default_path = "/org/gnome/SettingsDaemon/Power"
)]
pub(crate) trait Screen {
    #[dbus_proxy(property)]
    fn brightness(&self) -> zbus::Result<i32>;
    #[dbus_proxy(property)]
    fn set_brightness(&self, brightness: i32) -> zbus::Result<()>;
    fn step_up(&self) -> zbus::Result<(i32, String)>;
    fn step_down(&self) -> zbus::Result<(i32, String)>;
}

#[cfg(tests)]
mod tests {
    #[test]
    fn screen_step_up_down() -> anyhow::Result<()> {
        let connection = zbus::Connection::session().await.unwrap();
        let proxy = gnome_power::ScreenProxy::new(&connection).await?;
        let (new_level_perc, _) = prox.step_up().await?;
        let (new_level_perc, _) = prox.step_down().await?;
    }
}
