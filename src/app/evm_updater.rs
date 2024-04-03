use crate::app::carrier::Carrier;
use crate::app::evm_updater::EvmUpdate::{CarrierListUpdate, StationListUpdate};
use crate::app::settings::Settings;
use crate::app::station::Station;
use bus::Bus;
use chrono::DateTime;
use ethers::contract::ContractCall;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Http, LocalWallet, Provider, U256};
use log::error;
use std::sync::Arc;

#[derive(Clone)]
pub enum EvmUpdate {
    CarrierListUpdate(Vec<Carrier>),
    StationListUpdate(Vec<Station>),
}
pub struct EvmUpdater {
    bus: Bus<EvmUpdate>,
    settings: Arc<Settings>,
}

pub fn initialize(bus: Bus<EvmUpdate>, settings: Arc<Settings>) -> EvmUpdater {
    EvmUpdater { bus, settings }
}

impl EvmUpdater {
    pub fn run_update(&mut self) {
        if let Some(contract) = &self.settings.evm_settings.contract {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    {
                        let function_call: ContractCall<
                            SignerMiddleware<Provider<Http>, LocalWallet>,
                            Vec<u64>,
                        > = contract.get_carrier_ids();
                        let mut carriers = Vec::new();
                        match function_call.legacy().call().await {
                            Ok(results) => {
                                for carrier_id in results {
                                    let function_call = contract.carrier_map(carrier_id);
                                    match function_call.call().await {
                                        Ok(result) => {
                                            let carrier = Carrier {
                                                timestamp: DateTime::from_timestamp(
                                                    result.1.as_u64() as i64,
                                                    0,
                                                )
                                                .unwrap(),
                                                name: result.2,
                                                callsign: result.3,
                                                services: result.4,
                                                docking_access: result.5,
                                                allow_notorious: result.6,
                                                current_system: result.7,
                                                current_body: result.8,
                                                next_system: result.9,
                                                next_body: result.10,
                                                departure: DateTime::from_timestamp(
                                                    result.11.as_u64() as i64,
                                                    0,
                                                )
                                                .unwrap(),
                                            };
                                            carriers.push(carrier);
                                        }
                                        Err(err) => {
                                            error!("Error getting carriers: {err}");
                                            //return vec![];
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Error getting carriers ids: {err}");
                            }
                        }
                        if !carriers.is_empty() {
                            self.bus.broadcast(CarrierListUpdate(carriers));
                        }
                    }

                    {
                        let function_call: ContractCall<
                            SignerMiddleware<Provider<Http>, LocalWallet>,
                            Vec<u64>,
                        > = contract.get_stations();
                        let mut stations = Vec::new();
                        match function_call.legacy().call().await {
                            Ok(results) => {
                                for station_id in results {
                                    let function_call = contract.station_map(station_id);
                                    match function_call.call().await {
                                        Ok(result) => {
                                            let station = Station {
                                                timestamp: DateTime::from_timestamp(
                                                    result.1.as_u64() as i64,
                                                    0,
                                                )
                                                .unwrap(),
                                                name: result.2,
                                                _type: result.3,
                                                services: result.9,
                                                system_name: result.5,
                                                faction: result.6,
                                                government: result.7,
                                                economy: result.8,
                                                distance: result.10,
                                                landingpads: result.11,
                                            };
                                            stations.push(station);
                                        }
                                        Err(err) => {
                                            error!("Error getting stations: {err}");
                                            //return vec![];
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Error getting stations ids: {err}");
                            }
                        }
                        if !stations.is_empty() {
                            self.bus.broadcast(StationListUpdate(stations));
                        }
                    }
                });
        }
    }
}
