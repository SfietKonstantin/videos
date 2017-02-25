use types::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub enum Mode {
    Dummy,
    CacheSpreading,
    CacheFilling,
    Descent
}

pub fn algo(mode: Mode, cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
            requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {
    match mode {
        Mode::Dummy => dummy_algo(),
        Mode::CacheSpreading => cache_spreading(cache_info, videos),
        Mode::CacheFilling => cache_filling(cache_info, videos),
        Mode::Descent => descent(cache_info, videos, endpoints, requests)
    }
}

fn dummy_algo() -> BTreeMap<i32, BTreeSet<i32>> {
    BTreeMap::new()
}

struct FilledCache {
    videos: BTreeSet<i32>,
    remaining_capacity: i32
}

impl FilledCache {
    fn new(capacity: i32) -> FilledCache {
        FilledCache {
            videos: BTreeSet::new(),
            remaining_capacity: capacity
        }
    }

    fn add_video(&mut self, ref video: &Video) -> bool {
        if video.size <= self.remaining_capacity {
            self.remaining_capacity -= video.size;
            self.videos.insert(video.id);
            true
        } else {
            false
        }
    }
}

fn cache_spreading(cache_info: CacheInfo, videos: Vec<Video>) -> BTreeMap<i32, BTreeSet<i32>> {
    let mut filled: BTreeMap<i32, FilledCache>
        = (0..cache_info.count).map(|id| (id, FilledCache::new(cache_info.capacity))).collect();

    let mut current_cache: i32 = 0;
    for video in videos {
        filled.get_mut(&current_cache).map(|cache| cache.add_video(&video));
        current_cache = (current_cache + 1) % cache_info.count;
    }

    let returned: BTreeMap<i32, BTreeSet<i32>> = filled.iter().map(|(cache_id, cache)| {
        let videos: BTreeSet<i32> = cache.videos.clone();
        (*cache_id, videos)
    }).collect();
    returned
}

fn cache_filling(cache_info: CacheInfo, videos: Vec<Video>) -> BTreeMap<i32, BTreeSet<i32>> {
    let mut filled: BTreeMap<i32, FilledCache>
        = (0..cache_info.count).map(|id| (id, FilledCache::new(cache_info.capacity))).collect();

    for video in videos {
        let mut current_cache: i32 = 0;
        let mut ok: bool = false;
        while current_cache < cache_info.count && !ok {
            ok = filled.get_mut(&current_cache).unwrap().add_video(&video);
            if !ok {
                current_cache += 1;
            }
        }
    }

    let returned: BTreeMap<i32, BTreeSet<i32>> = filled.iter().map(|(cache_id, cache)| {
        let videos: BTreeSet<i32> = cache.videos.clone();
        (*cache_id, videos)
    }).collect();
    returned
}

fn descent(cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
           requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {
    // Compute the gain for each cache x video
    // Gain = sum per endpoint ( requests * (latency datacenter - latency cache) )
    let returned: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();

    println!("Process the requests per video x endpoint");
    let mut video_endpoint_to_request: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for video in videos {
        video_endpoint_to_request.insert(video.id, BTreeMap::new());
    }

    for request in requests {
        video_endpoint_to_request.get_mut(&request.video_id).unwrap().insert(request.endpoint_id, request.count);
    }

    println!("Process the endpoints reacheable by a cache");
    let mut cache_endpoint_to_latency: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for cache in 0..cache_info.count {
        cache_endpoint_to_latency.insert(cache, BTreeMap::new());
    }

    for endpoint in endpoints {
        for (cache_id, latency) in endpoint.latency_to_cache {
            cache_endpoint_to_latency.get_mut(&cache_id).unwrap().insert(endpoint.id, latency);
        }
    }

    println!("Process the gain per video x endpoint");




    returned
}