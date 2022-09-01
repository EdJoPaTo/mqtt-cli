use std::thread::sleep;
use std::time::Duration;

use rumqttc::{Client, Connection, QoS};

use crate::format;
use crate::mqtt::Payload;

#[derive(Clone, Copy)]
pub enum Mode {
    Dry,
    Normal,
    Silent,
}

pub fn clean_retained(mut client: Client, mut connection: Connection, mode: Mode) {
    let mut amount: usize = 0;
    for notification in connection.iter() {
        if let rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(_)) =
            notification.expect("connection error")
        {
            break;
        }
    }
    for notification in connection.iter() {
        match notification {
            Ok(rumqttc::Event::Outgoing(rumqttc::Outgoing::Disconnect)) => break,
            Ok(rumqttc::Event::Outgoing(rumqttc::Outgoing::PingReq)) => {
                client.disconnect().unwrap();
            }
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                if publish.payload.is_empty() {
                    // Thats probably myself cleaning up
                    continue;
                }
                if !publish.retain {
                    client.disconnect().unwrap();
                    continue;
                }
                let topic = publish.topic.clone();
                if !matches!(mode, Mode::Silent) {
                    let qos = format::qos(publish.qos);
                    let size = publish.payload.len();
                    let payload = format::payload(&Payload::new(&publish.payload), size);
                    println!("QoS:{:11} {:50} {}", qos, publish.topic, payload);
                }
                amount += 1;
                if !matches!(mode, Mode::Dry) {
                    client.publish(topic, QoS::ExactlyOnce, true, []).unwrap();
                }
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("Connection Error: {}", err);
                sleep(Duration::from_millis(25));
            }
        }
    }
    match mode {
        Mode::Silent => {}
        Mode::Dry => println!("Dry run: would have cleaned {} topics", amount),
        Mode::Normal => println!("Cleaned {} topics", amount),
    }
}
