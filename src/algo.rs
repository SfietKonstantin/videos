use types::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cmp::max;

pub enum Mode {
    Dummy,
    CacheSpreading,
    CacheFilling,
    Descent,
    DescentCost,
    DescentAudience,
    BestVideo,
    DescentAmend
}

pub fn algo(mode: Mode, cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
            requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {
    match mode {
        Mode::Dummy => dummy_algo(),
        Mode::CacheSpreading => cache_spreading(cache_info, videos),
        Mode::CacheFilling => cache_filling(cache_info, videos),
        Mode::Descent => descent(GainMode::PureGain, cache_info, videos, endpoints, requests),
        Mode::DescentCost => descent(GainMode::GainOverCost, cache_info, videos, endpoints, requests),
        Mode::DescentAudience => descent(GainMode::GainOverAudience, cache_info, videos, endpoints, requests),
        Mode::BestVideo => best_video(cache_info, videos, endpoints, requests),
        Mode::DescentAmend => descent_amend(cache_info, videos, endpoints, requests)
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

pub enum GainMode {
    PureGain,
    GainOverCost,
    GainOverAudience
}

pub fn descent_gain(gain_mode: GainMode, ref cache_info: &CacheInfo, ref videos: &Vec<Video>,
                    endpoints: Vec<Endpoint>, requests: Vec<Request>) -> BTreeMap<i32, Vec<(i32, i32)>> {
    println!("Process the requests per video x endpoint");
    let mut video_endpoint_to_request: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for request in requests {
        if !video_endpoint_to_request.contains_key(&request.video_id) {
            video_endpoint_to_request.insert(request.video_id, BTreeMap::new());
        }
        video_endpoint_to_request.get_mut(&request.video_id).unwrap().insert(request.endpoint_id, request.count);
    }

    println!("Process the endpoints reacheable by a cache");
    let mut datacenter_endpoint_to_latency: BTreeMap<i32, i32> = BTreeMap::new();
    let mut cache_endpoint_to_latency: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for endpoint in endpoints {
        for (cache_id, latency) in endpoint.cache_to_latency {
            if cache_id >= 0 {
                if !cache_endpoint_to_latency.contains_key(&cache_id) {
                    cache_endpoint_to_latency.insert(cache_id, BTreeMap::new());
                }
                cache_endpoint_to_latency.get_mut(&cache_id).unwrap().insert(endpoint.id, latency);
            } else {
                datacenter_endpoint_to_latency.insert(endpoint.id, latency);
            }
        }
    }

    println!("Process the gain per video x endpoint");

    // Compute the gain for each cache x video
    // Gain = sum per endpoint ( requests * (latency datacenter - latency cache) )
    let mut gains: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
    for video in *videos {
        println!("Video: {}", video.id);
        let endpoint_to_request = video_endpoint_to_request.get(&video.id);
        let endpoints: BTreeSet<i32> = match endpoint_to_request {
            Some(endpoint_to_request) => endpoint_to_request.iter()
                .map(|(&endpoint, _)| endpoint)
                .collect(),
            None => BTreeSet::new()
        };

        for cache_id in 0..cache_info.count {
            let mut all_requests = 0;
            let endpoints_latency = cache_endpoint_to_latency.get(&cache_id).unwrap();
            let gain = endpoints_latency.iter()
            .filter(|&(endpoint, _)| endpoints.contains(endpoint))
            .fold(0, |gain, (endpoint, latency)| {
                let requests = endpoint_to_request.unwrap().get(&endpoint).unwrap();
                let datacenter_latency = datacenter_endpoint_to_latency.get(&endpoint).unwrap();
                all_requests += *requests;
                gain + (datacenter_latency - latency) * requests
            });
            let gain_over_audience = match all_requests {
                0 => 0,
                _ => gain / all_requests
            };
            let effective_gain = match gain_mode {
                GainMode::PureGain => gain,
                GainMode::GainOverCost => gain / video.size,
                GainMode::GainOverAudience => gain_over_audience
            };
            if !gains.contains_key(&effective_gain) {
                gains.insert(effective_gain, Vec::new());
            }
            gains.get_mut(&effective_gain).unwrap().push((video.id, cache_id));
        }
    }

    println!("Done processing gain");
    gains
}

fn descent(gain_mode: GainMode, cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
           requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {

    let gains = descent_gain(gain_mode, &cache_info, &videos, endpoints, requests);
    let mut filled: BTreeMap<i32, FilledCache>
    = (0..cache_info.count).map(|id| (id, FilledCache::new(cache_info.capacity))).collect();

    for (_, ref mapping) in gains.iter().rev() {
        for &(video_id, cache_id) in *mapping {
            filled.get_mut(&cache_id).unwrap().add_video(&videos[video_id as usize]);
        }
    }


    let returned: BTreeMap<i32, BTreeSet<i32>> = filled.iter().map(|(cache_id, cache)| {
        let videos: BTreeSet<i32> = cache.videos.clone();
        (*cache_id, videos)
    }).collect();
    returned
}

fn best_video(cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
              requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {
    // First, compute endpoints for each video
    let mut video_endpoint_to_request: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();
    for video in &videos {
        video_endpoint_to_request.insert(video.id, BTreeMap::new());
    }

    for request in &requests {
        video_endpoint_to_request.get_mut(&request.video_id).unwrap().insert(request.endpoint_id, request.count);
    }

    let mut datacenter_endpoint_to_latency: BTreeMap<i32, i32> = BTreeMap::new();
    let mut cache_endpoint_to_latency: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for endpoint in endpoints {
        for (cache_id, latency) in endpoint.cache_to_latency {
            if cache_id >= 0 {
                if !cache_endpoint_to_latency.contains_key(&cache_id) {
                    cache_endpoint_to_latency.insert(cache_id, BTreeMap::new());
                }
                cache_endpoint_to_latency.get_mut(&cache_id).unwrap().insert(endpoint.id, latency);
            } else {
                datacenter_endpoint_to_latency.insert(endpoint.id, latency);
            }
        }
    }

    // Then compute caches that are needed for each video
    let mut video_to_caches: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
    for video in &videos {
        println!("Video: {}", video.id);
        if !video_to_caches.contains_key(&video.id) {
            video_to_caches.insert(video.id, BTreeSet::new());
        }

        let caches = video_to_caches.get_mut(&video.id).unwrap();
        let endpoint_to_requests = video_endpoint_to_request.get(&video.id).unwrap();
        for cache_id in 0..cache_info.count {
            if cache_endpoint_to_latency.contains_key(&cache_id) {
                for (endpoint, _) in cache_endpoint_to_latency.get(&cache_id).unwrap() {
                    if endpoint_to_requests.contains_key(endpoint) {
                        caches.insert(cache_id);
                    }
                }
            }
        }
    }

    // Compute gain for each video
    let mut gain_to_videos: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for video in &videos {
        let gain = video_endpoint_to_request.values().fold(0, |gain, ref endpoint_to_request| {
            gain + endpoint_to_request.values().fold(0, |gain, &request| gain + request)
        });
        if !gain_to_videos.contains_key(&gain) {
            gain_to_videos.insert(gain, Vec::new());
        }
        gain_to_videos.get_mut(&gain).unwrap().push(video.id);
    }

    // Fill caches
    let mut filled: BTreeMap<i32, FilledCache>
    = (0..cache_info.count).map(|id| (id, FilledCache::new(cache_info.capacity))).collect();

    for (_, ref video_ids) in gain_to_videos.iter().rev() {
        for video_id in *video_ids {
            for cache_id in video_to_caches.get(video_id).unwrap() {
                filled.get_mut(&cache_id).unwrap().add_video(&videos[*video_id as usize]);
            }
        }
    }

    let returned: BTreeMap<i32, BTreeSet<i32>> = filled.iter().map(|(cache_id, cache)| {
        let videos: BTreeSet<i32> = cache.videos.clone();
        (*cache_id, videos)
    }).collect();
    returned
}

// Returns for (video, cache) a map of endpoint -> saved
pub fn segmented_gain(ref cache_info: &CacheInfo, ref videos: &Vec<Video>,
                      endpoints: Vec<Endpoint>, requests: Vec<Request>) -> BTreeMap<(i32, i32), BTreeMap<i32, i32>> {
    println!("Process the requests per video x endpoint");
    let mut video_endpoint_to_request: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for request in requests {
        if !video_endpoint_to_request.contains_key(&request.video_id) {
            video_endpoint_to_request.insert(request.video_id, BTreeMap::new());
        }
        video_endpoint_to_request.get_mut(&request.video_id).unwrap().insert(request.endpoint_id, request.count);
    }

    println!("Process the endpoints reacheable by a cache");
    let mut datacenter_endpoint_to_latency: BTreeMap<i32, i32> = BTreeMap::new();
    let mut cache_endpoint_to_latency: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();

    for endpoint in endpoints {
        for (cache_id, latency) in endpoint.cache_to_latency {
            if cache_id >= 0 {
                if !cache_endpoint_to_latency.contains_key(&cache_id) {
                    cache_endpoint_to_latency.insert(cache_id, BTreeMap::new());
                }
                cache_endpoint_to_latency.get_mut(&cache_id).unwrap().insert(endpoint.id, latency);
            } else {
                datacenter_endpoint_to_latency.insert(endpoint.id, latency);
            }
        }
    }

    println!("Process the gain per video x endpoint");

    let mut gains: BTreeMap<(i32, i32), BTreeMap<i32, i32>> = BTreeMap::new();
    for video in *videos {
        println!("Video: {}", video.id);
        let endpoint_to_request = video_endpoint_to_request.get(&video.id);
        let endpoints: BTreeSet<i32> = match endpoint_to_request {
            Some(endpoint_to_request) => endpoint_to_request.iter()
                .map(|(&endpoint, _)| endpoint)
                .collect(),
            None => BTreeSet::new()
        };

        for cache_id in 0..cache_info.count {
            gains.insert((video.id, cache_id), BTreeMap::new());
            let mut gain_map = gains.get_mut(&(video.id, cache_id)).unwrap();

            let endpoints_latency = cache_endpoint_to_latency.get(&cache_id).unwrap();
            let endpoints = endpoints_latency.iter().filter(|&(endpoint, _)| endpoints.contains(endpoint));
            for (endpoint, latency) in endpoints {
                let requests = endpoint_to_request.unwrap().get(&endpoint).unwrap();
                let datacenter_latency = datacenter_endpoint_to_latency.get(&endpoint).unwrap();
                let gain = (datacenter_latency - latency) * requests;
                gain_map.insert(*endpoint, gain);
            }
        }
    }

    println!("Done processing gain");
    gains
}

fn gain(video_id: i32, cache_id: i32, videos: &Vec<Video>,
        gains: &BTreeMap<(i32, i32), BTreeMap<i32, i32>>, filled: &BTreeMap<i32, FilledCache>) -> i32 {
    if videos[video_id as usize].size > filled.get(&cache_id).unwrap().remaining_capacity {
        0
    } else {
        gains.get(&(video_id, cache_id)).unwrap().values().fold(0, |gain, value| gain + value)
    }
}

fn gain_simple(video_id: i32, cache_id: i32, videos: &Vec<Video>,
               local_gains: &BTreeMap<i32, i32>, filled: &BTreeMap<i32, FilledCache>) -> i32 {
    if videos[video_id as usize].size > filled.get(&cache_id).unwrap().remaining_capacity {
        0
    } else {
        local_gains.values().fold(0, |gain, value| gain + value)
    }
}

fn left_space(filled: &BTreeMap<i32, FilledCache>) -> i32 {
    filled.values().fold(0, |space, ref cache| space + cache.remaining_capacity)
}

fn descent_amend(cache_info: CacheInfo, videos: Vec<Video>, endpoints: Vec<Endpoint>,
                 requests: Vec<Request>) -> BTreeMap<i32, BTreeSet<i32>> {

    let mut gains = segmented_gain(&cache_info, &videos, endpoints, requests);
    let mut filled: BTreeMap<i32, FilledCache>
    = (0..cache_info.count).map(|id| (id, FilledCache::new(cache_info.capacity))).collect();

    let mut computed_gains: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    for (&(video_id, cache_id), _) in &gains {
        computed_gains.insert((video_id, cache_id), gain(video_id, cache_id, &videos, &gains, &filled));
    }

    let total = left_space(&filled);
    let mut left = left_space(&filled);
    while left > 0 && gains.len() > 0 {
        println!("{} / {}", left, total);

        // Build the best gain
        let mut current_cache_id = -1;
        let mut current_video_id = -1;
        let mut current_gain = 0;

        for (&(video_id, cache_id), &local_gain) in &computed_gains {
            if local_gain > current_gain {
                current_video_id = video_id;
                current_cache_id = cache_id;
                current_gain = local_gain;
            }
        }

        println!("Best gain: {} for video {} and cache {}", current_cache_id, current_video_id, current_cache_id);
        filled.get_mut(&current_cache_id).unwrap().add_video(&videos[current_video_id as usize]);
        let gain_per_endpoint: BTreeMap<i32, i32> = gains.get(&(current_video_id, current_cache_id)).unwrap().clone();
        gains.remove(&(current_video_id, current_cache_id));
        computed_gains.remove(&(current_video_id, current_cache_id));

        // Amend all other caches: putting the video inside them is likely to yield a smaller gain
        for (&(video_id, cache_id), mut other_gain_per_endpoint) in gains.iter().filter(|&(&(video_id, _), _)| video_id == current_video_id) {
            let other_endpoints: BTreeSet<&i32> = gain_per_endpoint.keys().clone().collect();
            for endpoint_id in other_endpoints.iter().filter(|&endpoint_id| other_endpoints.contains(endpoint_id)) {
                
            }
        }


        /*
        gains.iter_mut()
            .filter(|&(&(video_id, _), _)| { video_id == current_video_id })
            .map(|(&(video_id, cache_id), mut other_gain_per_endpoint)| {
                other_gain_per_endpoint.iter_mut()
                    .filter(|&(endpoint, _)| { other_endpoints.has(&endpoint) })
                    .map(|&(endpoint, gain)| {

                    }).count();

                for (endpoint, gain) in &gain_per_endpoint {
                    if other_gain_per_endpoint.contains_key(&endpoint) {
                        let local_gain = other_gain_per_endpoint.get_mut(&endpoint).unwrap();
                        let previous_local_gain: i32 = *local_gain;
                        let new_local_gain = max(previous_local_gain - gain, 0);
                        println!("New local gain {} {} {}", new_local_gain, previous_local_gain, gain);
                        // other_gain_per_endpoint.insert(*endpoint, new_local_gain);
                        //local_gain = new_local_gain as &mut i32;
                    }
                }
                computed_gains.insert((video_id, cache_id), gain_simple(video_id, cache_id, &videos, &other_gain_per_endpoint, &filled));
            }).count();
        */

        left = left_space(&filled);
    }

    let returned: BTreeMap<i32, BTreeSet<i32>> = filled.iter().map(|(cache_id, cache)| {
        let videos: BTreeSet<i32> = cache.videos.clone();
        (*cache_id, videos)
    }).collect();
    returned
}