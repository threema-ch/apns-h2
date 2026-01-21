use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APS, APSAlert, APSSound, Payload};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct WebPushAlert<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub action: &'a str,
}

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # use apns_h2::request::notification::{NotificationBuilder, WebNotificationBuilder, WebPushAlert};
/// # use apns_h2::request::payload::PayloadLike;
/// # fn main() {
/// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
/// builder.set_sound("prööt");
/// let payload = builder.build("device_id", Default::default())
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct WebNotificationBuilder<'a> {
    alert: WebPushAlert<'a>,
    sound: Option<&'a str>,
    url_args: &'a [&'a str],
    interruption_level: Option<crate::request::payload::InterruptionLevel>,
}

impl<'a> WebNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new(alert: WebPushAlert<'a>, url_args: &'a [&'a str]) -> WebNotificationBuilder<'a> {
        WebNotificationBuilder {
            alert,
            sound: None,
            url_args,
            interruption_level: None,
        }
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_sound("meow");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"sound\":\"meow\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(&mut self, sound: &'a str) -> &mut Self {
        self.sound = Some(sound);
        self
    }

    /// Set the interruption level to active. The system presents the notification
    /// immediately, lights up the screen, and can play a sound.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_active_interruption_level();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"interruption-level\":\"active\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_active_interruption_level(&mut self) -> &mut Self {
        self.interruption_level = Some(crate::request::payload::InterruptionLevel::Active);
        self
    }

    /// Set the interruption level to critical. The system presents the notification
    /// immediately, lights up the screen, and bypasses the mute switch to play a sound.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_critical_interruption_level();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"interruption-level\":\"critical\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_critical_interruption_level(&mut self) -> &mut Self {
        self.interruption_level = Some(crate::request::payload::InterruptionLevel::Critical);
        self
    }

    /// Set the interruption level to passive. The system adds the notification to
    /// the notification list without lighting up the screen or playing a sound.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_passive_interruption_level();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"interruption-level\":\"passive\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_passive_interruption_level(&mut self) -> &mut Self {
        self.interruption_level = Some(crate::request::payload::InterruptionLevel::Passive);
        self
    }

    /// Set the interruption level to time sensitive. The system presents the notification
    /// immediately, lights up the screen, can play a sound, and breaks through system
    /// notification controls.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_time_sensitive_interruption_level();
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"interruption-level\":\"time-sensitive\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_time_sensitive_interruption_level(&mut self) -> &mut Self {
        self.interruption_level = Some(crate::request::payload::InterruptionLevel::TimeSensitive);
        self
    }

    /// Set the interruption level directly. Controls how the notification is presented to the user.
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use apns_h2::request::payload::{PayloadLike, InterruptionLevel};
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello", body: "World", action: "View"}, &["arg1"]);
    /// builder.set_interruption_level(InterruptionLevel::Active);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"interruption-level\":\"active\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_interruption_level(&mut self, level: crate::request::payload::InterruptionLevel) -> &mut Self {
        self.interruption_level = Some(level);
        self
    }
}

impl<'a> NotificationBuilder<'a> for WebNotificationBuilder<'a> {
    fn build(self, device_token: &'a str, options: NotificationOptions<'a>) -> Payload<'a> {
        Payload {
            aps: APS {
                alert: Some(APSAlert::WebPush(self.alert)),
                badge: None,
                sound: self.sound.map(APSSound::Sound),
                thread_id: None,
                content_available: None,
                category: None,
                mutable_content: None,
                interruption_level: self.interruption_level,
                url_args: Some(self.url_args),
                timestamp: None,
                event: None,
                content_state: None,
                attributes_type: None,
                attributes: None,
                input_push_channel: None,
                input_push_token: None,
            },
            device_token,
            options,
            data: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::payload::PayloadLike;
    use serde_json::Value;

    #[test]
    fn test_webpush_notification() {
        let payload = WebNotificationBuilder::new(
            WebPushAlert {
                action: "View",
                title: "Hello",
                body: "world",
            },
            &["arg1"],
        )
        .build("device-token", Default::default())
        .to_json_string()
        .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "title": "Hello",
                    "body": "world",
                    "action": "View",
                },
                "url-args": ["arg1"]
            }
        });

        assert_eq!(expected_payload, serde_json::from_str::<Value>(&payload).unwrap());
    }
}
