extern crate videos;

use videos::parser::parse;

#[test]
fn test_invalid_header() {
    let result_option = parse("0 0 0 0");
    assert!(result_option.is_none());
}

#[test]
fn test_invalid_videos() {
    let result_option = parse("0 0 0 0 0");
    assert!(result_option.is_none());
}

#[test]
fn test_invalid_video_count() {
    let result_option = parse("1 0 0 0 0\n");
    assert!(result_option.is_none());
}

#[test]
fn test_cache_info_and_videos() {
    let result_option = parse("2 0 0 123 456\n12 34");
    assert!(result_option.is_some());

    let (cache_info, videos, endpoints, _) = result_option.unwrap();
    assert_eq!(123, cache_info.count);
    assert_eq!(456, cache_info.size);
    assert_eq!(2, videos.len());
    assert_eq!(0, videos[0].id);
    assert_eq!(12, videos[0].size);
    assert_eq!(1, videos[1].id);
    assert_eq!(34, videos[1].size);
    assert_eq!(0, endpoints.len());
}

#[test]
fn test_end_to_end() {
    let result_option = parse("5 2 4 3 100\n\
    50 50 80 30 110\n\
    1000 3\n\
    0 100\n\
    2 100\n\
    1 300\n\
    500 0\n\
    3 0 1500\n\
    0 1 1000\n\
    4 0 500\n\
    1 0 1000");
    assert!(result_option.is_some());

    let (cache_info, videos, endpoints, requests) = result_option.unwrap();
    assert_eq!(3, cache_info.count);
    assert_eq!(100, cache_info.size);
    assert_eq!(5, videos.len());
    assert_eq!(50, videos[0].size);
    assert_eq!(50, videos[1].size);
    assert_eq!(80, videos[2].size);
    assert_eq!(30, videos[3].size);
    assert_eq!(110, videos[4].size);
    assert_eq!(2, endpoints.len());
    assert_eq!(4, endpoints[0].latency_to_cache.len());
    assert_eq!(1000, *endpoints[0].latency_to_cache.get(&-1).unwrap());
    assert_eq!(100, *endpoints[0].latency_to_cache.get(&0).unwrap());
    assert_eq!(300, *endpoints[0].latency_to_cache.get(&1).unwrap());
    assert_eq!(100, *endpoints[0].latency_to_cache.get(&2).unwrap());
    assert_eq!(500, *endpoints[1].latency_to_cache.get(&-1).unwrap());
    assert_eq!(4, requests.len());
    assert_eq!(3, requests[0].video_id);
    assert_eq!(0, requests[0].endpoint_id);
    assert_eq!(1500, requests[0].request_count);
    assert_eq!(0, requests[1].video_id);
    assert_eq!(1, requests[1].endpoint_id);
    assert_eq!(1000, requests[1].request_count);
    assert_eq!(4, requests[2].video_id);
    assert_eq!(0, requests[2].endpoint_id);
    assert_eq!(500, requests[2].request_count);
    assert_eq!(1, requests[3].video_id);
    assert_eq!(0, requests[3].endpoint_id);
    assert_eq!(1000, requests[3].request_count);
}