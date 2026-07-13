//! Network module - wave-based inter-node communication
//! 
//! Layer 2 Ethernet driver for transmitting/receiving wave data
//! between cooperative nodes

pub mod wave_nic;

pub use wave_nic::WaveNetworkDriver;
