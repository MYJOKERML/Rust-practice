#![feature(impl_trait_in_assoc_type)]

use anyhow::{anyhow, Error};
// use pilota::serde::de::value;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use volo_gen::volo::example::{self, Item};

pub mod client;

// 用一个Hashmap储存数据
struct ItemDict {
	dict: Arc<RwLock<HashMap<String, String>>>,
}

// 定义 get/set/del/delay_del 的方法
impl ItemDict {
	fn new() -> Self {
		Self {
			dict: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	fn get(&self, key: &str) -> Option<String> {
		let dict = self.dict.read().unwrap();
		match dict.get(key) {
			Some(value) => Some(format!("GET: {:?}\r\n", value)),
			None => None,
		}
	}

	fn set(&self, key: &str, value: &str) -> Option<String> {
		let mut dict = self.dict.write().unwrap();
		match dict.insert(key.to_string(), value.to_string()) {
			Some(_item) => Some(format!("SUCCESSFULLY SET {{ {:?}: {:?} }}\r\n", key, value)),
			None => Some("ERROR: Invalid arguments\r\n".to_string()),
		}
	}

	fn del(&self, key: &str) -> Option<String> {
		let mut dict = self.dict.write().unwrap();
		match dict.remove(key) {
			Some(value) => Some(format!("SUCCESSFULLY DELETED {{ {:?}: {:?} }}\r\n", key, value)),
			None => None,
		}
	}

	async fn delay_del(&self, key: &str, delay: u64) -> Option<String> {
        let dict = self.dict.clone();
        let key = key.to_string();
        let delay = delay.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            dict.write().unwrap().remove(&key);
            tracing::info!("{} is deleted", key);
        });
        None
    }
}

// 存储channel
struct ChannelDict {
	send_dict: RwLock<HashMap<String, Arc<tokio::sync::broadcast::Sender<String>>>>,
}

impl ChannelDict {
	fn new() -> Self {
		Self {
			send_dict: RwLock::new(HashMap::new()),
		}
	}

    fn get_recv(&self, key: &str) -> Option<tokio::sync::broadcast::Receiver<String>> {
        tracing::info!("Getting recv");
        let tx = self.get_send(key);
        tracing::info!("Got send");
        match tx {
            Some(tx) => {
                tracing::info!("Subscribing");
                // Using subscribe() to get a receiver
                Some(tx.subscribe())
            },
            None => {
                tracing::info!("ERROR WHEN GET RECV");
                panic!("ERROR WHEN GET RECV")
            },
        }
    }
	fn get_send(&self, key: &str) -> Option<Arc<tokio::sync::broadcast::Sender<String>>> {
		let is_send = self.send_dict.read().unwrap().contains_key(key);
        match is_send {
            true => {
                Some(self.send_dict.read().unwrap().get(key).unwrap().clone())
            },
            false => {
                self.set(key);
                Some(self.send_dict.read().unwrap().get(key).unwrap().clone())
            }
        }
	}

	fn set(&self, key: &str) {
		let mut dict = self.send_dict.write().unwrap();
		dict.insert(key.to_string(), Arc::new(tokio::sync::broadcast::channel(1024).0));
		tracing::info!("CREATING NEW CHANNEL");
	}

	fn drop(&self, key: &str) {
		let mut dict = self.send_dict.write().unwrap();
		let recv_cnt =  match dict.get(key) {
            Some(send) => send.receiver_count(),
            None => usize::MAX,
        };
        tracing::info!("Dropping recv: {}", recv_cnt);
		if recv_cnt == 0 {
			dict.remove(key);
		}
	}
}

#[macro_export]
macro_rules! getfromitem {
	($item:expr) => {
		match $item.value {
			Some(value) => format!("{} {}", $item.key, value).into(),
			None => $item.key,
		}
	};
}

pub struct S {
	item_dict: ItemDict,
	channel_dict: ChannelDict,
}

unsafe impl Send for S {}
unsafe impl Sync for S {}

impl S {
	pub fn new() -> Self {
		Self {
			item_dict: ItemDict::new(),
			channel_dict: ChannelDict::new(),
		}
	}
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    async fn get(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
		match self.item_dict.get(&req.key) {
			Some(value) => Ok(volo_gen::volo::example::ItemResponse {
				item: Item {
					key: req.key.clone(),
					value: Some(value.into()),
					deleted_delay: None,
				},
			}),
			// None => Err(anyhow!("ERROR: Key not found\r\n".to_string())),
			None => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some("ERROR: Key not found\r\n".into()),
                    deleted_delay: None,
                },
			}),
		}
	}
    async fn set(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
		let _ = self.item_dict.set(&req.item.key, &req.item.value.unwrap());
        
        if req.item.deleted_delay.is_some() {
            let _ = self.item_dict.delay_del(&req.item.key, req.item.deleted_delay.unwrap() as u64).await;
        }
        Ok(volo_gen::volo::example::ItemResponse {
            item: Item {
                key: req.item.key.clone(),
                value: None,
                deleted_delay: None,
            },
        })
    }
    async fn del(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
        let result = self.item_dict.del(&req.key);
        match result {
            Some(_) => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some(format!("SUCCESSFULLY DELETED {:?}\r\n", req.key.clone()).into()),
                    deleted_delay: None,
                },
            }),
            None => Ok(volo_gen::volo::example::ItemResponse {
                item: Item {
                    key: req.key.clone(),
                    value: Some("ERROR: Key not found\r\n".into()),
                    deleted_delay: None,
                },
            }),
        }
    }
    async fn ping(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
		let ret = match req.item.value {
            // 如果value存在，返回value
            Some(value) => value,
            // 如果value不存在，返回PONG
            None => "PONG".to_string().into(),
        };
        Ok(volo_gen::volo::example::ItemResponse {
            item: Item {
                key: req.item.key.clone(),
                value: Some(ret),
                deleted_delay: None,
            },
        })
    }
    async fn publish(
        &self,
        req: volo_gen::volo::example::ItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
        let send = self.channel_dict.get_send(&req.item.key);
        match send {
            Some(send) => {
                let subscribers = send.receiver_count();
                let result = send.send(req.item.value.unwrap().to_string());
                
                self.channel_dict.drop(&req.item.key);
                match result {
                    Ok(_) => Ok(volo_gen::volo::example::ItemResponse {
                        item: Item {
                            key: req.item.key.clone(),
                            value: Some(subscribers.to_string().into()),
                            deleted_delay: None,
                        },
                    }),
                    Err(_) => if subscribers == 0 {
                        Ok(volo_gen::volo::example::ItemResponse {
                            item: Item {
                                key: req.item.key.clone(),
                                value: Some(subscribers.to_string().into()),
                                deleted_delay: None,
                            },
                        })
                    } else {
                        Err(::volo_thrift::AnyhowError::from(Error::msg("PUBLISH SEND ERROR")))
                    }
                }
            }
            None => Err(::volo_thrift::AnyhowError::from(Error::msg("PUBLISH GET ERROR"))),
        }
    }
    async fn subscribe(
        &self,
        req: volo_gen::volo::example::KeyRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::ItemResponse, ::volo_thrift::AnyhowError>
    {
        tracing::info!("Reaching subscribe");
        let recv = self.channel_dict.get_recv(&req.key);
        tracing::info!("Got recv");
        match recv {
            Some(mut recv) => {
                // Spawn a task to receive and return the message
                tracing::info!("Spawning thread");
                let thread = tokio::task::spawn(async move {
                    let message = recv.recv().await;
                    match message {
                        Ok(message) => Ok(message),
                        Err(_) => Err(::volo_thrift::AnyhowError::from(Error::msg("SUBSCRIBE RECV ERROR"))),
                    }
                });
                tracing::info!("Awaiting thread");
                // Wait for the message
                let message = thread.await;
                tracing::info!("thread drop: {:?}", message);
                // Try to drop the sender when the receiver is dropped
                self.channel_dict.drop(&req.key);
                match message {
                    Ok(message) => Ok(volo_gen::volo::example::ItemResponse {
                        item: Item {
                            key: req.key.clone(),
                            value: Some(message.unwrap().into()),
                            deleted_delay: None,
                        },
                    }),
                    Err(e) => Err(e.into()),
                }
            }
            None => Err(::volo_thrift::AnyhowError::from(Error::msg("SUBSCRIBE GET ERROR"))),
        }
    }
}

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, S> volo::Service<Cx, example::ItemServiceRequestSend> for FilterService<S>
where
    S: Send + 'static + volo::Service<Cx, example::ItemServiceRequestSend> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: example::ItemServiceRequestSend) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
		let to_check = match req.clone() {
            example::ItemServiceRequestSend::Get(example::ItemServiceGetArgsSend{req}) => {
                req.key
            },
            example::ItemServiceRequestSend::Set(example::ItemServiceSetArgsSend{req}) => {
                getfromitem!(req.item)
            },
            example::ItemServiceRequestSend::Del(example::ItemServiceDelArgsSend{req}) => {
                req.key
            },
            example::ItemServiceRequestSend::Ping(example::ItemServicePingArgsSend{req}) => {
                getfromitem!(req.item)
            },
            example::ItemServiceRequestSend::Subscribe(example::ItemServiceSubscribeArgsSend{req}) => {
                req.key
            },
            example::ItemServiceRequestSend::Publish(example::ItemServicePublishArgsSend{req}) => {
                getfromitem!(req.item)
            },
        };
		tracing::debug!("Checking {}", to_check);
		if to_check.contains("Fuck") ||  to_check.contains("fuck") || to_check.contains("FUCK") {
			return Err(anyhow!("FILTERED").into());
		}
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct FilterLayer;
impl <S> volo::Layer<S> for FilterLayer {
	type Service = FilterService<S>;

	fn layer(self, inner: S) -> Self::Service {
		FilterService(inner)
	}
}
