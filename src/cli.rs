use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
pub enum SubCommands {
    /// Clean retained messages from the broker.
    ///
    /// This works by subscribing to the topic and waiting for messages with the retained flag.
    /// Then a message with an empty payload is published retained which clears the topic on the broker.
    /// Ends on the first non retained message or when the timeout is reached.
    #[clap(visible_alias = "c", visible_alias = "clean")]
    CleanRetained {
        /// Topic which gets cleaned.
        ///
        /// Supports filters like 'foo/bar/#'.
        #[clap(value_hint = ValueHint::Other)]
        topic: String,

        /// When there is no message received for the given time the operation is considered done
        #[clap(
            long,
            value_hint = ValueHint::Other,
            value_name = "SECONDS",
            default_value_t = 5.0,
        )]
        timeout: f32,

        /// Dont clean topics, only log them
        #[clap(long)]
        dry_run: bool,
    },

    /// Log values from subscribed topics to stdout
    #[clap(visible_alias = "l")]
    Log {
        /// Topics to watch
        #[clap(
            env = "MQTTUI_TOPIC",
            value_hint = ValueHint::Other,
            default_value = "#",
        )]
        topic: Vec<String>,

        /// Show full MQTT communication
        #[clap(short, long)]
        verbose: bool,
    },

    /// Publish a value quickly
    #[clap(visible_alias = "p", visible_alias = "pub")]
    Publish {
        /// Topic to publish to
        #[clap(value_hint = ValueHint::Other)]
        topic: String,

        /// Payload to be published
        #[clap(value_hint = ValueHint::Unknown)]
        payload: String,

        /// Publish the MQTT message retained
        #[clap(short, long, env = "MQTTUI_RETAIN")]
        retain: bool,

        /// Show full MQTT communication
        #[clap(short, long)]
        verbose: bool,
    },
}

#[derive(Debug, Parser)]
#[clap(about, author, version, name = "MQTT TUI")]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommands: Option<SubCommands>,

    /// URL which represents how to connect to the MQTT broker eq 'mqtts://user:password@localhost:8883'
    ///
    /// Examples: `mqtt://localhost/`, `mqtts://localhost/`, `mqtts://user:password@localhost/`
    ///
    /// Passing the broker password via command line is insecure as the password can be read from the history!
    /// In that case you should pass the broker URL via environment variable.
    #[clap(
        short,
        long,
        env = "MQTTUI_BROKER",
        value_hint = ValueHint::Url,
        value_name = "URL",
        global = true,
        hide_env_values = true,
        default_value = "mqtt://localhost/",
    )]
    pub broker: url::Url,

    /// Username to connect with
    ///
    /// The username to use when connecting to the broker, it can be provided as part of the url if that is preferred
    #[clap(
        short,
        long,
        env = "MQTTUI_USER",
        value_hint = ValueHint::Username,
        global = true,
        hide_env_values = true,
    )]
    pub username: Option<String>,

    /// Password to use when connecting to the broker
    ///
    /// Passing the broker password via command line is insecure as the password can be read from the history!
    /// In that case you should pass the password via environment variable.
    #[clap(
        long,
        env = "MQTTUI_PASSWORD",
        value_hint = ValueHint::Other,
        global = true,
        hide_env_values = true,
    )]
    pub password: Option<String>,

    /// Specify the Client ID to be used for this connection
    ///
    /// Overrides the default randomized client-id for this connection to the broker
    #[clap(
        long,
        short = 'i',
        env = "MQTTUI_CLIENT_ID",
        value_hint = ValueHint::Other,
        global = true,
        hide_env_values = true,
    )]
    pub client_id: Option<String>,

    /// Port to connect to
    ///
    /// The port to use to connect to the broker
    #[clap(
        short,
        long,
        env = "MQTTUI_PORT",
        value_hint = ValueHint::Other,
        global = true,
        hide_env_values = true,
    )]
    pub port: Option<u16>,

    /// Allow insecure TLS connections
    #[clap(long, global = true)]
    pub insecure: bool,

    /// Topic to watch
    #[clap(
        env = "MQTTUI_TOPIC",
        value_hint = ValueHint::Other,
        default_value = "#",
    )]
    pub topic: String,
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
