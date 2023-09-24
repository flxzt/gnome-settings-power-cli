use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.gnome.SettingsDaemon.Power.Keyboard",
    default_service = "org.gnome.SettingsDaemon.Power",
    default_path = "/org/gnome/SettingsDaemon/Power"
)]
pub(crate) trait Keyboard {
    #[dbus_proxy(property)]
    fn brightness(&self) -> zbus::Result<i32>;
    #[dbus_proxy(property)]
    fn set_brightness(&self, brightness: i32) -> zbus::Result<()>;
    fn step_up(&self) -> zbus::Result<i32>;
    fn step_down(&self) -> zbus::Result<i32>;
    fn toggle(&self) -> zbus::Result<i32>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn keyboard_step_up_down() -> anyhow::Result<()> {
        let connection = zbus::Connection::session().await.unwrap();
        let proxy = KeyboardProxy::new(&connection).await?;
        let _new_brightness = proxy.step_up().await?;
        let _new_brightness = proxy.step_down().await?;
        Ok(())
    }
}
