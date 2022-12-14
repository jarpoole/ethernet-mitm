use std::thread::sleep_ms;

use super::attack::Attack;

//use dns_parser::Packet;
use erased_serde::Serialize as ErasedSerialize;
use nom::IResult;
use pcap::Capture;
//use pktparse::{ethernet, ip, ipv4, udp};
use async_trait::async_trait;
use serde::Serialize;
use tokio::task;
use tokio::task::JoinHandle;
use tracing::info;

pub struct Sniff {
    pub interface1: String,
    pub interface2: String,
    pub concurrency: u8,
    pub tasks: Vec<JoinHandle<()>>,
}

#[derive(Serialize)]
struct SniffedPacket {
    foo: u32,
}

#[derive(Serialize)]
struct SniffConfig {
    foo: u32,
}

#[async_trait]
impl Attack for Sniff {
    fn get_config(&self) -> Box<dyn erased_serde::Serialize> {
        Box::new(SniffConfig { foo: 0 })
    }
    fn get_type(&self) -> String {
        "sniff".to_owned()
    }
    fn get_logs(&self) -> Vec<Box<dyn ErasedSerialize>> {
        vec![Box::new(SniffedPacket { foo: 0 })]
    }

    fn start(&mut self) {
        for _ in 0..self.concurrency {
            self.tasks.push(task::spawn(async {
                let mut x = 0;
                loop {
                    info!("{}", x);
                    x += 1;
                    sleep_ms(1000);
                }
            }));
        }
        ()
        /*
        println!("Listening on interface {}", interface_name);
        let mut cap = Capture::from_device(interface_name)
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();
        cap.filter("port 53", true).unwrap();
        while let Ok(packet) = cap.next_packet() {
            println!("");
            println!("got packet! {:?}", packet);
            if let IResult::Ok((remainder, frame)) = ethernet::parse_ethernet_frame(&packet.data) {
                if frame.ethertype == ethernet::EtherType::IPv4 {
                    if let IResult::Ok((remainder, v4)) = ipv4::parse_ipv4_header(&remainder) {
                        if v4.protocol == ip::IPProtocol::UDP {
                            if let IResult::Ok((remainder, udp)) = udp::parse_udp_header(&remainder)
                            {
                                if udp.source_port == 53 || udp.dest_port == 53 {
                                    let dns = Packet::parse(&remainder);
                                    println!("{:?}", dns);
                                }
                            }
                        }
                    }
                }
            }
        }
        ()
        */
    }
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        /*
        for task in self.tasks.iter() {
            task.abort();
        }
        self.tasks.clear();
        */
        Ok(())
    }
}

/*
//extern crate af_packet;
//extern crate dns_parser;
//extern crate nom;
//extern crate num_cpus;
//extern crate pktparse;

use af_packet::rx;
//use std::env;
use dns_parser::Packet;
use nom::IResult;
use pktparse::{ethernet, ip, ipv4, udp};

pub fn start(interface_name: &str) {
    //let args: Vec<String> = env::args().collect();
    let mut settings = rx::RingSettings::default();
    let mut fds = Vec::<i32>::new();

    //disable rx hash stuff because DNS shouldn't need it (At least over UDP)
    settings.fanout_method = rx::PACKET_FANOUT_LB;
    settings.ring_settings.tp_feature_req_word = 0;
    settings.if_name = interface_name.to_string();
    //settings.if_name = args[1].clone();

    for _ in 0..2 {
        let ring_settings = settings.clone();
        let mut ring = rx::Ring::new(ring_settings).unwrap();
        fds.push(ring.socket.fd);
        thread::spawn(move || {
            loop {
                let mut block = ring.get_block();
                for packet in block.get_raw_packets() {
                    //think ethernet header is 82b offset
                    if let IResult::Ok((remainder, frame)) =
                        ethernet::parse_ethernet_frame(&packet.data[82..])
                    {
                        if frame.ethertype == ethernet::EtherType::IPv4 {
                            if let IResult::Ok((remainder, v4)) =
                                ipv4::parse_ipv4_header(&remainder)
                            {
                                if v4.protocol == ip::IPProtocol::UDP {
                                    if let IResult::Ok((remainder, udp)) =
                                        udp::parse_udp_header(&remainder)
                                    {
                                        if udp.source_port == 53 || udp.dest_port == 53 {
                                            let dns = Packet::parse(&remainder);
                                            println!("{:?}", dns);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                block.mark_as_consumed();
            }
        });
    }

    let mut packets: u64 = 0;
    let mut drops: u64 = 0;

    loop {
        let mut stats: (u64, u64) = (0, 0);
        for fd in &fds {
            let ring_stats = rx::get_rx_statistics(*fd).unwrap();
            stats.0 += ring_stats.tp_drops as u64;
            stats.1 += ring_stats.tp_packets as u64;
        }
        drops += stats.0;
        packets += stats.1;
        eprintln!(
            "{} frames received per second, {} dropped. {} total drops of {} total packets ({}%)",
            stats.1,
            stats.0,
            drops,
            packets,
            format!("{:.*}", 4, drops as f64 / packets as f64 * 100 as f64)
        );
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
*/
