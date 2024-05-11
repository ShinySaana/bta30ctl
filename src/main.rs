use btleplug::api::CentralEvent;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::error::Error;
use futures_util::stream::StreamExt;
use std::env;


const BTA_NAME: &str = "FiiO BTA30 Pro";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("bta30ctl requires one argument (volume).");
        std::process::exit(1);
    }

    let volume: u8 = args.get(1).unwrap().parse().unwrap();
    if volume > 60 {
        eprintln!("volume must be between 0 and 60 inclusive.");
        std::process::exit(1);
    }

    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();
    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;
    let mut bta_peripheral: Option<Peripheral> = None;    

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let discovered = central.peripheral(&id).await?;
                let local_name = discovered.properties()
                    .await
                    .unwrap()
                    .unwrap()
                    .local_name;

                if local_name.is_some_and(|n| n == BTA_NAME) {
                    bta_peripheral = Some(discovered);
                    break;
                }
            }
            _ => {}
        }
    }

    let bta_peripheral: Peripheral = bta_peripheral.unwrap();
    
    println!("{}", bta_peripheral.properties()
        .await
        .unwrap()
        .unwrap()
        .local_name.unwrap()
    );

    println!("{}", bta_peripheral.properties()
        .await
        .unwrap()
        .unwrap()
        .address
    );

    bta_peripheral.discover_services().await?;

    if !bta_peripheral.is_connected().await? {
        bta_peripheral.connect().await?;
    }

    let volume_adjustable = bta30ctl::set_volume_mode_setting(
        bta30ctl::OperationalMode::Rx,
        bta30ctl::VolumeModeSetting::Adjustable,
    ).unwrap();
    bta30ctl::send_command(&bta_peripheral, &volume_adjustable).await?;

    let volume_off = bta30ctl::set_volume_command(volume).unwrap();
    bta30ctl::send_command(&bta_peripheral, &volume_off).await?;

    Ok(())
}
