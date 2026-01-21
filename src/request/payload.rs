/// Payload with `aps` and custom data
use crate::error::Error;
use crate::request::notification::{DefaultAlert, DefaultSound, NotificationOptions, WebPushAlert};
use erased_serde::Serialize;
use serde_json::{self, Value};
use std::collections::BTreeMap;
use std::fmt::Debug;

/// The data and options for a push notification.
#[derive(Debug, Clone, Serialize)]
pub struct Payload<'a> {
    /// Send options
    #[serde(skip)]
    pub options: NotificationOptions<'a>,
    /// The token for the receiving device
    #[serde(skip)]
    pub device_token: &'a str,
    /// The pre-defined notification payload
    pub aps: APS<'a>,
    /// Application specific payload
    #[serde(flatten)]
    pub data: BTreeMap<&'a str, Value>,
}

/// Object that can be serialized to create an APNS request.
/// You probably just want to use [`Payload`], which implements [`PayloadLike`].
///
/// # Example
/// ```no_run
/// use apns_h2::request::notification::{NotificationBuilder, NotificationOptions};
/// use apns_h2::request::payload::{PayloadLike, APS};
/// use apns_h2::{Client, ClientConfig, DefaultNotificationBuilder, Endpoint};
/// use serde::Serialize;
/// use std::fs::File;
///
/// async fn send() -> Result<(), Box<dyn std::error::Error>> {
///     let builder = DefaultNotificationBuilder::new()
///         .set_body("Hi there")
///         .set_badge(420)
///         .set_category("cat1")
///         .set_sound("ping.flac");
///
///     let payload = builder.build("device-token-from-the-user", Default::default());
///     let mut file = File::open("/path/to/private_key.p8")?;
///
///     let client = Client::token(&mut file, "KEY_ID", "TEAM_ID", ClientConfig::default()).unwrap();
///
///     let response = client.send(payload).await?;
///     println!("Sent: {:?}", response);
///     Ok(())
/// }
///
/// #[derive(Serialize, Debug)]
/// struct Payload<'a> {
///     aps: APS<'a>,
///     my_custom_value: String,
///     #[serde(skip_serializing)]
///     options: NotificationOptions<'a>,
///     #[serde(skip_serializing)]
///     device_token: &'a str,
/// }
///
/// impl<'a> PayloadLike for Payload<'a> {
///     fn get_device_token(&self) -> &'a str {
///         self.device_token
///     }
///     fn get_options(&self) -> &NotificationOptions<'_> {
///         &self.options
///     }
/// }
/// ```
pub trait PayloadLike: serde::Serialize + Debug {
    /// Combine the APS payload and the custom data to a final payload JSON.
    /// Returns an error if serialization fails.
    #[allow(clippy::wrong_self_convention)]
    fn to_json_string(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Returns token for the device
    fn get_device_token(&self) -> &str;

    /// Gets [`NotificationOptions`] for this Payload.
    fn get_options(&self) -> &NotificationOptions<'_>;
}

impl<'a> PayloadLike for Payload<'a> {
    fn get_device_token(&self) -> &'a str {
        self.device_token
    }

    fn get_options(&self) -> &NotificationOptions<'_> {
        &self.options
    }
}

impl<'a> Payload<'a> {
    /// Client-specific custom data to be added in the payload.
    /// The `root_key` defines the JSON key in the root of the request
    /// data, and `data` the object containing custom data. The `data`
    /// should implement `Serialize`, which allows using of any Rust
    /// collection or if needing more strict type definitions, any struct
    /// that has `#[derive(Serialize)]` from [Serde](https://serde.rs).
    ///
    /// Using a `HashMap`:
    ///
    /// ```rust
    /// # use apns_h2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// # use std::collections::HashMap;
    /// # use apns_h2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut payload = DefaultNotificationBuilder::new()
    ///     .set_content_available()
    ///     .build("token", Default::default());
    /// let mut custom_data = HashMap::new();
    ///
    /// custom_data.insert("foo", "bar");
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"content-available\":1,\"mutable-content\":0},\"foo_data\":{\"foo\":\"bar\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    ///
    /// Using a custom struct:
    ///
    /// ```rust
    /// #[macro_use] extern crate serde;
    /// use apns_h2::request::notification::{DefaultNotificationBuilder, NotificationBuilder};
    /// use apns_h2::request::payload::PayloadLike;
    /// fn main() {
    /// #[derive(Serialize)]
    /// struct CompanyData {
    ///     foo: &'static str,
    /// }
    ///
    /// let mut payload = DefaultNotificationBuilder::new()
    ///     .set_content_available()
    ///     .build("token", Default::default());
    /// let mut custom_data = CompanyData { foo: "bar" };
    ///
    /// payload.add_custom_data("foo_data", &custom_data).unwrap();
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"content-available\":1,\"mutable-content\":0},\"foo_data\":{\"foo\":\"bar\"}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// }
    /// ```
    pub fn add_custom_data(&mut self, root_key: &'a str, data: &dyn Serialize) -> Result<&mut Self, Error> {
        self.data.insert(root_key, serde_json::to_value(data)?);

        Ok(self)
    }
}

/// The pre-defined notification data.
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::upper_case_acronyms)]
pub struct APS<'a> {
    /// The notification content. Can be empty for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<APSAlert<'a>>,

    /// A number shown on top of the app icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,

    /// The name of the sound file to play when user receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<APSSound<'a>>,

    /// An app-specific identifier for grouping related notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<&'a str>,

    /// Set to one for silent notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_available: Option<u8>,

    /// When a notification includes the category key, the system displays the
    /// actions for that category as buttons in the banner or alert interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<&'a str>,

    /// If set to one, the app can change the notification content before
    /// displaying it to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_content: Option<u8>,

    /// Interruption level for the notification. Controls how the notification
    /// is presented to the user and what system settings it can bypass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_level: Option<InterruptionLevel>,

    /// The date when the system should automatically remove the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissal_date: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_args: Option<&'a [&'a str]>,

    /// Live Activity: Timestamp for the Live Activity update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    /// Live Activity: Event type ("start" to begin a Live Activity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<&'a str>,

    /// Live Activity: Content state with dynamic data for the Live Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_state: Option<Value>,

    /// Live Activity: Type of attributes for the Live Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_type: Option<&'a str>,

    /// Live Activity: Attributes data for the Live Activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Value>,

    /// Live Activity: Input push channel ID for iOS 18+ channel-based updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_push_channel: Option<&'a str>,

    /// Live Activity: Set to 1 to request a new push token for iOS 18+ token-based updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_push_token: Option<u8>,
}

/// Different notification content types.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum APSAlert<'a> {
    /// A notification that supports all of the iOS features
    Default(DefaultAlert<'a>),
    /// Safari web push notification
    WebPush(WebPushAlert<'a>),
    /// A notification with just a body
    Body(&'a str),
}

/// Different notification sound types.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum APSSound<'a> {
    /// A critical notification (supported only on >= iOS 12)
    Critical(DefaultSound<'a>),
    /// Name for a notification sound
    Sound(&'a str),
}

/// Interruption level for notification delivery and presentation.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum InterruptionLevel {
    /// The system presents the notification immediately, lights up the screen, and can play a sound.
    Active,
    /// The system presents the notification immediately, lights up the screen, and bypasses the mute switch to play a sound.
    Critical,
    /// The system adds the notification to the notification list without lighting up the screen or playing a sound.
    Passive,
    /// The system presents the notification immediately, lights up the screen, can play a sound, and breaks through system notification controls.
    TimeSensitive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::notification::{DefaultNotificationBuilder, NotificationBuilder};

    #[test]
    fn test_interruption_level_serialization() {
        let builder = DefaultNotificationBuilder::new()
            .set_title("Test Title")
            .set_active_interruption_level();
        let payload = builder.build("test-token", Default::default());

        let json = payload.to_json_string().unwrap();
        assert!(json.contains("\"interruption-level\":\"active\""));

        let builder = DefaultNotificationBuilder::new()
            .set_title("Test Title")
            .set_critical_interruption_level();
        let payload = builder.build("test-token", Default::default());

        let json = payload.to_json_string().unwrap();
        assert!(json.contains("\"interruption-level\":\"critical\""));

        let builder = DefaultNotificationBuilder::new()
            .set_title("Test Title")
            .set_passive_interruption_level();
        let payload = builder.build("test-token", Default::default());

        let json = payload.to_json_string().unwrap();
        assert!(json.contains("\"interruption-level\":\"passive\""));

        let builder = DefaultNotificationBuilder::new()
            .set_title("Test Title")
            .set_time_sensitive_interruption_level();
        let payload = builder.build("test-token", Default::default());

        let json = payload.to_json_string().unwrap();
        assert!(json.contains("\"interruption-level\":\"time-sensitive\""));
    }

    #[test]
    fn test_dismissal_date_serialization() {
        let builder = DefaultNotificationBuilder::new()
            .set_title("Test Title")
            .set_dismissal_date(1672531200); // January 1, 2023 00:00:00 UTC
        let payload = builder.build("test-token", Default::default());

        let json = payload.to_json_string().unwrap();
        assert!(json.contains("\"dismissal-date\":1672531200"));
    }

    #[test]
    fn test_live_activity_payload_serialization() {
        use serde_json::json;

        let content_state = json!({
            "currentHealthLevel": 100,
            "eventDescription": "Adventure has begun!"
        });

        let attributes = json!({
            "currentHealthLevel": 100,
            "eventDescription": "Adventure has begun!"
        });

        let builder = DefaultNotificationBuilder::new()
            .set_timestamp(1234)
            .set_event("start")
            .set_content_state(&content_state)
            .set_attributes_type("AdventureAttributes")
            .set_attributes(&attributes)
            .set_title("Adventure Alert")
            .set_body("Your adventure has started!");

        let payload = builder.build("test-token", Default::default());
        let json_str = payload.to_json_string().unwrap();

        // Verify all Live Activity fields are correctly serialized
        assert!(json_str.contains("\"timestamp\":1234"));
        assert!(json_str.contains("\"event\":\"start\""));
        assert!(
            json_str.contains(
                "\"content-state\":{\"currentHealthLevel\":100,\"eventDescription\":\"Adventure has begun!\"}"
            )
        );
        assert!(json_str.contains("\"attributes-type\":\"AdventureAttributes\""));
        assert!(
            json_str
                .contains("\"attributes\":{\"currentHealthLevel\":100,\"eventDescription\":\"Adventure has begun!\"}")
        );

        // Test iOS 18 channel-based Live Activity
        let builder = DefaultNotificationBuilder::new()
            .set_event("start")
            .set_input_push_channel("dHN0LXNyY2gtY2hubA==")
            .set_attributes_type("AdventureAttributes")
            .set_attributes(&attributes);

        let payload = builder.build("test-token", Default::default());
        let json_str = payload.to_json_string().unwrap();

        assert!(json_str.contains("\"input-push-channel\":\"dHN0LXNyY2gtY2hubA==\""));

        // Test iOS 18 token-based Live Activity
        let builder = DefaultNotificationBuilder::new()
            .set_event("start")
            .set_input_push_token()
            .set_attributes_type("AdventureAttributes")
            .set_attributes(&attributes);

        let payload = builder.build("test-token", Default::default());
        let json_str = payload.to_json_string().unwrap();

        assert!(json_str.contains("\"input-push-token\":1"));
    }
}
