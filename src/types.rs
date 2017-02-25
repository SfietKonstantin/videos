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
    pub latency_to_cache: BTreeMap<i32, i32>
}

impl Endpoint {
    pub fn new(id: i32, latency_to_cache: BTreeMap<i32, i32>) -> Endpoint {
        Endpoint {
            id: id,
            latency_to_cache: latency_to_cache
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
    pub request_count: i32
}

impl Request {
    pub fn new(video_id: i32, endpoint_id: i32, request_count: i32) -> Request {
        Request {
            video_id: video_id,
            endpoint_id: endpoint_id,
            request_count: request_count
        }
    }
}

pub struct CacheInfo {
    pub count: i32,
    pub size: i32
}

impl CacheInfo {
    pub fn new(count: i32, size: i32) -> CacheInfo {
        CacheInfo {
            count: count,
            size: size
        }
    }
}