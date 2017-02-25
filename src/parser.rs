use types::*;
use std::collections::BTreeMap;

pub fn parse(input: &str) -> Option<(CacheInfo, Vec<Video>, Vec<Endpoint>, Vec<Request>)> {
    let mut header = Header::new();
    let mut videos: Vec<Video> = Vec::new();
    let mut endpoints: Vec<Endpoint> = Vec::new();
    let mut splitted = input.split('\n');

    splitted.next()
        .and_then(parse_header)
        .and_then(|parsed_header| {
            header = parsed_header;
            splitted.next()
        })
        .and_then(|videos| { parse_videos(videos, header.video_count) })
        .and_then(|parsed_videos| {
            videos = parsed_videos;
            parse_endpoints(&mut splitted, header.endpoint_count)
        }).and_then(|parsed_endpoints| {
            endpoints = parsed_endpoints;
            parse_requests(&mut splitted, header.request_count)
        }).map(|requests| {
            (CacheInfo::new(header.cache_count, header.cache_size), videos, endpoints, requests)
        })
}

struct Header {
    video_count: i32,
    endpoint_count: i32,
    request_count: i32,
    cache_count: i32,
    cache_size: i32
}

impl Header {
    fn new() -> Header {
        Header {
            video_count: 0,
            endpoint_count: 0,
            request_count: 0,
            cache_count: 0,
            cache_size: 0
        }
    }
}

fn string_to_i32(string: &str) -> Option<i32> {
    string.parse::<i32>().ok()
}

fn parse_header(header: &str) -> Option<Header> {
    let parsed_header: Option<Vec<i32>> = header.split(' ').map(string_to_i32).collect();
    parsed_header.and_then(|values| {
        match values.len() {
            5 => Some(Header {
                video_count: values[0],
                endpoint_count: values[1],
                request_count: values[2],
                cache_count: values[3],
                cache_size: values[4]
            }),
            _ => None
        }
    })
}

fn parse_videos(header: &str, video_count: i32) -> Option<Vec<Video>> {
    let parsed_sizes: Option<Vec<i32>> = header.split(' ').map(string_to_i32).collect();
    parsed_sizes.and_then(|sizes| {
        if sizes.len() as i32 == video_count {
            Some(sizes.iter().enumerate()
                .map(|(id, &size)| Video::new(id as i32, size))
                .collect::<Vec<Video>>())
        } else {
            None
        }
    })
}

fn parse_endpoints(iter: &mut Iterator<Item = &str>, endpoint_count: i32) -> Option<Vec<Endpoint>> {
    let mut state = CurrentEndpointState::new();
    while !state.error && state.latency_to_cache.len() < endpoint_count as usize {
        if iter.next().map(|input| state.process(input)).is_none() {
            state.error = true
        }
    }

    if !state.error {
        Some(state.latency_to_cache.into_iter().enumerate()
            .map(|(id, latency_to_cache)| Endpoint::new(id as i32, latency_to_cache))
            .collect::<Vec<Endpoint>>())
    } else {
        None
    }
}

struct CurrentEndpointState {
    current_latency_to_cache: BTreeMap<i32, i32>,
    current_total_latency_count: i32,
    current_latency_count: i32,
    shoud_parse_header: bool,
    latency_to_cache: Vec<BTreeMap<i32, i32>>,
    error: bool

}

impl CurrentEndpointState {
    fn new() -> CurrentEndpointState {
        CurrentEndpointState {
            current_latency_to_cache: BTreeMap::new(),
            current_total_latency_count: 0,
            current_latency_count: 0,
            shoud_parse_header: true,
            latency_to_cache: Vec::new(),
            error: false
        }
    }

    fn parse_header(&mut self, header: &str) {
        let values: Option<Vec<i32>> = header.split(' ').map(string_to_i32).collect();
        match values {
            Some(ref values) if values.len() == 2 => self.set_header(values[1], values[0]),
            _ => self.error = true
        }
    }

    fn set_header(&mut self, total_latency_count: i32, datacenter_latency: i32) {
        self.current_latency_to_cache = BTreeMap::new();
        self.current_latency_to_cache.insert(-1, datacenter_latency);
        self.current_total_latency_count = total_latency_count;
        self.current_latency_count = 0;
        self.shoud_parse_header = false;
    }

    fn parse_latency(&mut self, latency: &str) {
        self.current_latency_count += 1;
        let values: Option<Vec<i32>> = latency.split(' ').map(string_to_i32).collect();
        match values {
            Some(ref values) if values.len() == 2 => self.set_latency(values[0], values[1]),
            _ => self.error = true
        }
    }

    fn set_latency(&mut self, cache_id: i32, latency: i32) {
        self.current_latency_to_cache.insert(cache_id, latency);
    }

    fn process(&mut self, input: &str) {
        if self.shoud_parse_header {
            self.parse_header(input);
        } else {
            self.parse_latency(input);
        }

        if self.current_latency_count == self.current_total_latency_count {
            self.shoud_parse_header = true;
            self.latency_to_cache.push(self.current_latency_to_cache.clone());
        }
    }
}

fn parse_requests(iter: &mut Iterator<Item = &str>, requests_count: i32) -> Option<Vec<Request>> {
    let parsed_requests: Option<Vec<Request>> = iter.map(parse_request).collect();
    parsed_requests.and_then(|requests| {
        if requests.len() as i32 == requests_count {
            Some(requests)
        } else {
            None
        }
    })
}

fn parse_request(request: &str) -> Option<Request> {
    let parsed_request: Option<Vec<i32>> = request.split(' ').map(string_to_i32).collect();
    parsed_request.and_then(|request| {
        if request.len() == 3 {
            Some(Request::new(request[0], request[1], request[2]))
        } else {
            None
        }
    })
}