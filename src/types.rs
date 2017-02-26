use std::collections::BTreeMap;

pub struct Video {
    pub id: i32,
    pub size: i32
}

impl Video {
    pub fn new(id: i32, size: i32) -> Video {
        Video {
            id: id,
            size: size
        }
    }
}

pub struct Endpoint {
    pub id: i32,
    pub cache_to_latency: BTreeMap<i32, i32>
}

impl Endpoint {
    pub fn new(id: i32, cache_to_latency: BTreeMap<i32, i32>) -> Endpoint {
        Endpoint {
            id: id,
            cache_to_latency: cache_to_latency
        }
    }
}

pub struct Cache {
    pub id: i32,
    pub capacity: i32
}

pub struct Request {
    pub video_id: i32,
    pub endpoint_id: i32,
    pub count: i32
}

impl Request {
    pub fn new(video_id: i32, endpoint_id: i32, count: i32) -> Request {
        Request {
            video_id: video_id,
            endpoint_id: endpoint_id,
            count: count
        }
    }
}

pub struct CacheInfo {
    pub count: i32,
    pub capacity: i32
}

impl CacheInfo {
    pub fn new(count: i32, capacity: i32) -> CacheInfo {
        CacheInfo {
            count: count,
            capacity: capacity
        }
    }
}