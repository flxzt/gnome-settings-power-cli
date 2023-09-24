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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn screen_step_up_down() -> anyhow::Result<()> {
        let connection = zbus::Connection::session().await.unwrap();
        let proxy = ScreenProxy::new(&connection).await?;
        let (_new_brightness, _) = proxy.step_up().await?;
        let (_new_brightness, _) = proxy.step_down().await?;
        Ok(())
    }
}
