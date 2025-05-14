use std::time::Duration;

use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use tokio::{task, time};

mod internal;

#[tokio::main(flavor = "current_thread")]
async fn main() {

    let mqtt_host: &str = "localhost";
    let mqtt_port: u16 = 1883;
    
    let mut options: MqttOptions = MqttOptions::new("rust-mqtt-demo", mqtt_host, mqtt_port);
    options.set_keep_alive(Duration::from_secs(10));

    let (mut client, mut eventloop): (AsyncClient, EventLoop) = AsyncClient::new(options, 10);
    client.subscribe("chat", QoS::AtMostOnce).await.unwrap();

    task::spawn(async {
        loop {
            //listen to a channel and send 
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    task::spawn(async {
        loop {
            let notification = eventloop.poll().await.unwrap();
        }
    }); 

    loop {

    }
}
