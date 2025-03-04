use super::utils::connect_with_retry;
use std::{
    collections::HashSet,
    io::prelude::*,
    sync::{
        atomic::{AtomicBool, AtomicIsize, Ordering},
        mpsc::Sender,
        Mutex,
    },
    time::{Duration, Instant},
};

use flate2::read::{DeflateDecoder, GzDecoder};
use log::*;
use tungstenite::{
    client::AutoStream, error::ProtocolError, protocol::frame::coding::CloseCode, Error, Message,
    WebSocket,
};

pub(super) enum MiscMessage {
    WebSocket(Message), // WebSocket message that needs to be sent to the server
    Reconnect,          // Needs to reconnect
    Misc,               // Misc message
    Pong,               // Pong message
    Normal,             // Normal message will be passed to on_msg
}

// `WSClientInternal` should be Sync + Send so that it can be put into Arc directly.
pub(super) struct WSClientInternal {
    exchange: &'static str, // Eexchange name
    pub(super) url: String, // Websocket base url
    ws_stream: Mutex<WebSocket<AutoStream>>,
    channels: Mutex<HashSet<String>>,     // subscribed channels
    tx: Mutex<Sender<String>>,            // The sending half of a channel
    on_misc_msg: fn(&str) -> MiscMessage, // handle misc messages
    // converts raw channels to subscribe/unsubscribe commands
    channels_to_commands: fn(&[String], bool) -> Vec<String>,
    should_stop: AtomicBool, // used by close() and run()
    // how often the client should send a ping, None means the client doesn't need to send
    // ping, instead the server will send ping and the client just needs to reply a pong
    client_ping_interval_and_msg: Option<(u64, &'static str)>,
    // Number of unanswered client ping messages, if greater than 3, the process will exit
    num_unanswered_ping: AtomicIsize,
    // How often the server sends a ping, only one of client_ping_interval_and_msg
    // and server_ping_interval should exist
    #[allow(dead_code)]
    server_ping_interval: Option<u64>,
}

impl WSClientInternal {
    pub fn new(
        exchange: &'static str,
        url: &str,
        tx: Sender<String>,
        on_misc_msg: fn(&str) -> MiscMessage,
        channels_to_commands: fn(&[String], bool) -> Vec<String>,
        client_ping_interval_and_msg: Option<(u64, &'static str)>,
        server_ping_interval: Option<u64>,
    ) -> Self {
        let timeout = if client_ping_interval_and_msg.is_some() && server_ping_interval.is_some() {
            panic!(
                "Only one of client_ping_interval_and_msg and server_ping_interval can have value"
            );
        } else if let Some(timeout) = client_ping_interval_and_msg {
            Some(timeout.0 / 2)
        } else {
            server_ping_interval
        };
        let stream = connect_with_retry(url, timeout);
        WSClientInternal {
            exchange,
            url: url.to_string(),
            ws_stream: Mutex::new(stream),
            tx: Mutex::new(tx),
            on_misc_msg,
            channels: Mutex::new(HashSet::new()),
            channels_to_commands,
            should_stop: AtomicBool::new(false),
            client_ping_interval_and_msg,
            num_unanswered_ping: AtomicIsize::new(0),
            server_ping_interval,
        }
    }

    pub fn subscribe(&self, channels: &[String]) {
        self.subscribe_or_unsubscribe(channels, true);
    }

    pub fn unsubscribe(&self, channels: &[String]) {
        self.subscribe_or_unsubscribe(channels, false);
    }

    fn get_send_interval_ms(&self) -> Option<u64> {
        match self.exchange {
            "binance" => Some(100), // WebSocket connections have a limit of 10 incoming messages per second
            "kucoin" => Some(100),  //  Message limit sent to the server: 100 per 10 seconds
            _ => None,
        }
    }

    fn subscribe_or_unsubscribe(&self, channels: &[String], subscribe: bool) {
        let mut diff = Vec::<String>::new();
        {
            let mut guard = self.channels.lock().unwrap();
            for ch in channels.iter() {
                if guard.insert(ch.clone()) {
                    diff.push(ch.clone());
                }
            }
        }

        if !diff.is_empty() {
            let commands = (self.channels_to_commands)(&diff, subscribe);
            let mut ws_stream = self.ws_stream.lock().unwrap();
            commands.into_iter().for_each(|command| {
                let ret = ws_stream.write_message(Message::Text(command));
                if let Err(err) = ret {
                    error!("Failed to send commands due to {}, exiting", err);
                    std::thread::sleep(Duration::from_secs(5));
                    std::process::exit(1); // fail fast, pm2 will restart
                }
                if let Some(interval) = self.get_send_interval_ms() {
                    std::thread::sleep(Duration::from_millis(interval));
                }
            });
        }
    }

    // reconnect and subscribe all channels
    fn _reconnect(&self) {
        warn!("Reconnecting to {}", &self.url);
        {
            let mut guard = self.ws_stream.lock().unwrap();
            let timeout = if self.client_ping_interval_and_msg.is_some()
                && self.server_ping_interval.is_some()
            {
                panic!(
                    "Only one of client_ping_interval_and_msg and server_ping_interval can have value"
                );
            } else if let Some(timeout) = self.client_ping_interval_and_msg {
                Some(timeout.0 / 2)
            } else {
                self.server_ping_interval
            };
            *guard = connect_with_retry(self.url.as_str(), timeout);
        }
        let channels = self
            .channels
            .lock()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !channels.is_empty() {
            let commands = (self.channels_to_commands)(&channels, true);
            let mut ws_stream = self.ws_stream.lock().unwrap();
            commands.into_iter().for_each(|command| {
                let ret = ws_stream.write_message(Message::Text(command));
                if let Err(err) = ret {
                    error!("{}", err);
                }
                if let Some(interval) = self.get_send_interval_ms() {
                    std::thread::sleep(Duration::from_millis(interval));
                }
            });
        }
    }

    // Handle a text msg from Message::Text or Message::Binary
    // Returns true if gets a normal message, otherwise false
    fn handle_msg(&self, txt: &str) -> bool {
        match (self.on_misc_msg)(txt) {
            MiscMessage::Misc => false,
            MiscMessage::Pong => {
                self.num_unanswered_ping.store(0, Ordering::Release);
                debug!(
                    "Received {} from {}, reset num_unanswered_ping to {}",
                    txt,
                    self.exchange,
                    self.num_unanswered_ping.load(Ordering::Acquire)
                );
                false
            }
            MiscMessage::Reconnect => {
                // self.reconnect();
                std::thread::sleep(Duration::from_secs(5));
                std::process::exit(1); // fail fast, pm2 will restart
            }
            MiscMessage::WebSocket(ws_msg) => {
                let ret = self.ws_stream.lock().unwrap().write_message(ws_msg);
                if let Err(err) = ret {
                    error!("{}", err);
                }
                false
            }
            MiscMessage::Normal => {
                if self.exchange == super::mxc::EXCHANGE_NAME
                    && self.url.as_str() == super::mxc::SPOT_WEBSOCKET_URL
                {
                    // special logic for MXC Spot
                    match txt.strip_prefix("42") {
                        Some(msg) => self.tx.lock().unwrap().send(msg.to_string()).unwrap(),
                        None => error!(
                            "{}, Not possible, should be handled by {}.on_misc_msg() previously",
                            txt, self.exchange
                        ),
                    }
                } else {
                    self.tx.lock().unwrap().send(txt.to_string()).unwrap();
                }
                true
            }
        }
    }

    pub fn run(&self, duration: Option<u64>) {
        let start_timstamp = Instant::now();
        let mut last_ping_timestamp = Instant::now();
        let mut num_read_timeout = 0;
        while !self.should_stop.load(Ordering::Acquire) {
            let resp = self.ws_stream.lock().unwrap().read_message();
            let mut succeeded = false;
            match resp {
                Ok(msg) => {
                    num_read_timeout = 0;
                    match msg {
                        Message::Text(txt) => succeeded = self.handle_msg(&txt),
                        Message::Binary(binary) => {
                            let mut txt = String::new();
                            let resp = match self.exchange {
                                super::huobi::EXCHANGE_NAME
                                | super::binance::EXCHANGE_NAME
                                | super::bitget::EXCHANGE_NAME
                                | super::bitz::EXCHANGE_NAME => {
                                    let mut decoder = GzDecoder::new(&binary[..]);
                                    decoder.read_to_string(&mut txt)
                                }
                                super::okex::EXCHANGE_NAME => {
                                    let mut decoder = DeflateDecoder::new(&binary[..]);
                                    decoder.read_to_string(&mut txt)
                                }
                                _ => {
                                    error!("Unknown binary format from {}", self.url);
                                    panic!("Unknown binary format from {}", self.url);
                                }
                            };

                            match resp {
                                Ok(_) => succeeded = self.handle_msg(&txt),
                                Err(err) => error!("Decompression failed, {}", err),
                            }
                        }
                        Message::Ping(resp) => {
                            info!(
                                "Received a ping frame: {}",
                                std::str::from_utf8(&resp).unwrap()
                            );
                            let ret = self
                                .ws_stream
                                .lock()
                                .unwrap()
                                .write_message(Message::Pong(resp));
                            if let Err(err) = ret {
                                error!("{}", err);
                            }
                        }
                        Message::Pong(resp) => {
                            let tmp = std::str::from_utf8(&resp);
                            self.num_unanswered_ping.store(0, Ordering::Release);
                            debug!("Received a pong frame: {} from {}, reset num_unanswered_ping to {}", tmp.unwrap(), self.exchange, self.num_unanswered_ping.load(Ordering::Acquire));
                        }
                        Message::Close(resp) => {
                            match resp {
                                Some(frame) => {
                                    if frame.code != CloseCode::Normal
                                        && frame.code != CloseCode::Away
                                    {
                                        error!(
                                            "Received a CloseFrame: code: {}, reason: {}, {}",
                                            frame.code,
                                            frame.reason,
                                            self.get_error_msg(),
                                        );
                                        std::thread::sleep(Duration::from_secs(5));
                                        std::process::exit(1); // fail fast, pm2 will restart
                                    } else {
                                        warn!(
                                            "Received a CloseFrame: code: {}, reason: {} from {}",
                                            frame.code, frame.reason, self.url
                                        );
                                    }
                                }
                                None => warn!("Received a close message without CloseFrame"),
                            }
                        }
                    }
                }
                Err(err) => {
                    match err {
                        Error::ConnectionClosed => {
                            error!("Server closed connection, exiting now...");
                            std::thread::sleep(Duration::from_secs(5));
                            std::process::exit(1); // fail fast, pm2 will restart
                        }
                        Error::AlreadyClosed => {
                            error!("Impossible to happen, fix the bug in the code");
                            panic!("Impossible to happen, fix the bug in the code");
                        }
                        Error::Io(io_err) => {
                            match io_err.kind() {
                                std::io::ErrorKind::WouldBlock => {
                                    num_read_timeout += 1;
                                    debug!(
                                        "read_message() timeout, increased num_read_timeout to {}",
                                        num_read_timeout
                                    );
                                }
                                std::io::ErrorKind::Interrupted => {
                                    // ignore SIGHUP, which will be handled by reopen
                                    info!("Ignoring SIGHUP");
                                }
                                std::io::ErrorKind::BrokenPipe => {
                                    error!(
                                        "I/O error thrown from read_message(): {}, {:?} {} {}",
                                        io_err,
                                        io_err.kind(),
                                        self.exchange,
                                        self.url
                                    );
                                    std::thread::sleep(Duration::from_secs(5));
                                    std::process::exit(1); // fail fast, pm2 will restart
                                }
                                _ => {
                                    error!(
                                        "I/O error thrown from read_message(): {}, {:?} {} {}",
                                        io_err,
                                        io_err.kind(),
                                        self.exchange,
                                        self.url
                                    );
                                    std::thread::sleep(Duration::from_secs(5));
                                    std::process::exit(1); // fail fast, pm2 will restart
                                }
                            }
                        }
                        Error::Protocol(protocol_err) => {
                            if protocol_err == ProtocolError::ResetWithoutClosingHandshake {
                                error!("ResetWithoutClosingHandshake");
                                std::thread::sleep(Duration::from_secs(5));
                                std::process::exit(1); // fail fast, pm2 will restart
                            } else {
                                error!(
                                    "Protocol error thrown from read_message(): {}",
                                    protocol_err
                                );
                            }
                        }
                        _ => {
                            error!("Error thrown from read_message(): {}", err);
                            panic!("Error thrown from read_message(): {}", err);
                        }
                    }
                }
            };

            if let Some(interval_and_msg) = self.client_ping_interval_and_msg {
                let num_unanswered_ping = self.num_unanswered_ping.load(Ordering::Acquire);
                if num_unanswered_ping > 5 {
                    error!(
                        "Exiting due to num_unanswered_ping: {}, duration: {} seconds",
                        num_unanswered_ping,
                        start_timstamp.elapsed().as_secs()
                    );
                    std::thread::sleep(Duration::from_secs(5));
                    std::process::exit(1); // fail fast, pm2 will restart
                }
                if last_ping_timestamp.elapsed() >= Duration::from_secs(interval_and_msg.0 / 2) {
                    debug!("Sending ping: {}", interval_and_msg.1);
                    // send ping
                    let ping_msg = if interval_and_msg.1.is_empty() {
                        Message::Ping(Vec::new())
                    } else {
                        Message::Text(interval_and_msg.1.to_string())
                    };
                    last_ping_timestamp = Instant::now();
                    if let Err(err) = self.ws_stream.lock().unwrap().write_message(ping_msg) {
                        error!("{}", err);
                    }
                }
            } else if num_read_timeout > 5 {
                error!(
                    "Exiting due to num_read_timeout: {}, duration: {} seconds",
                    num_read_timeout,
                    start_timstamp.elapsed().as_secs()
                );
                std::thread::sleep(Duration::from_secs(5));
                std::process::exit(1); // fail fast, pm2 will restart
            }

            if let Some(seconds) = duration {
                if start_timstamp.elapsed() > Duration::from_secs(seconds) && succeeded {
                    break;
                }
            }
        }
    }

    pub fn close(&self) {
        // break the while loop in run()
        self.should_stop.store(true, Ordering::Release);
        let ret = self.ws_stream.lock().unwrap().close(None);
        if let Err(err) = ret {
            error!("{}", err);
        }
    }

    fn get_error_msg(&self) -> String {
        format!(
            "{}, {}\n{}",
            self.exchange,
            self.url,
            self.channels
                .lock()
                .unwrap()
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

/// Define the new() constructor.
macro_rules! impl_new_constructor {
    ($struct_name:ident, $exchange:ident, $default_url:expr, $channels_to_commands:ident, $on_misc_msg:ident, $client_ping_interval_and_msg:expr, $server_ping_interval:expr) => {
        impl $struct_name {
            /// Creates a websocket client.
            ///
            /// # Arguments
            ///
            /// * `tx` - The sending part of a channel
            /// * `url` - Optional server url, usually you don't need specify it
            pub fn new(tx: Sender<String>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: WSClientInternal::new(
                        $exchange,
                        real_url,
                        tx,
                        $on_misc_msg,
                        $channels_to_commands,
                        $client_ping_interval_and_msg,
                        $server_ping_interval,
                    ),
                }
            }
        }
    };
}

/// Implement the WSClient trait.
macro_rules! impl_ws_client_trait {
    ($struct_name:ident) => {
        impl WSClient for $struct_name {
            fn subscribe_trade(&self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_orderbook(&self, channels: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, channels);
            }

            fn subscribe_orderbook_topk(&self, channels: &[String]) {
                <$struct_name as OrderBookTopK>::subscribe_orderbook_topk(self, channels);
            }

            fn subscribe_l3_orderbook(&self, channels: &[String]) {
                <$struct_name as Level3OrderBook>::subscribe_l3_orderbook(self, channels);
            }

            fn subscribe_ticker(&self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
            }

            fn subscribe_bbo(&self, channels: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, channels);
            }

            fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                <$struct_name as Candlestick>::subscribe_candlestick(self, symbol_interval_list);
            }

            fn subscribe(&self, channels: &[String]) {
                self.client.subscribe(channels);
            }

            fn unsubscribe(&self, channels: &[String]) {
                self.client.unsubscribe(channels);
            }

            fn run(&self, duration: Option<u64>) {
                self.client.run(duration);
            }

            fn close(&self) {
                self.client.close();
            }
        }
    };
}
