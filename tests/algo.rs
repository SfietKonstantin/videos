extern crate videos;
use std::collections::BTreeMap;
use videos::types::*;
use videos::algo::{descent_gain, GainMode};

#[test]
fn test_gain_1() {
    let cache_info = CacheInfo::new(1, 500);
    let videos = vec![Video::new(0, 100)];
    let mut endpoint0_relation: BTreeMap<i32, i32> = BTreeMap::new();
    endpoint0_relation.insert(-1, 200);
    endpoint0_relation.insert(0, 100);
    let endpoints = vec![Endpoint::new(0, endpoint0_relation)];
    let requests = vec![Request::new(0, 0, 1000)];
    let gain = descent_gain(GainMode::PureGain, &cache_info, &videos, endpoints, requests);
    assert_eq!(gain.get(&100000).unwrap()[0], (0, 0));
}

#[test]
fn test_gain_2() {
    let cache_info = CacheInfo::new(1, 500);
    let videos = vec![Video::new(0, 100), Video::new(1, 200)];
    let mut endpoint0_relation: BTreeMap<i32, i32> = BTreeMap::new();
    endpoint0_relation.insert(-1, 200);
    endpoint0_relation.insert(0, 100);
    let endpoints = vec![Endpoint::new(0, endpoint0_relation)];
    let requests = vec![Request::new(0, 0, 1000), Request::new(1, 0, 1500)];
    let gain = descent_gain(GainMode::PureGain, &cache_info, &videos, endpoints, requests);

    assert_eq!(gain.get(&100000).unwrap()[0], (0, 0));
    assert_eq!(gain.get(&150000).unwrap()[0], (1, 0));
}

#[test]
fn test_gain_3() {
    let cache_info = CacheInfo::new(1, 500);
    let videos = vec![Video::new(0, 100)];
    let mut endpoint0_relation: BTreeMap<i32, i32> = BTreeMap::new();
    endpoint0_relation.insert(-1, 300);
    endpoint0_relation.insert(0, 200);
    let mut endpoint1_relation: BTreeMap<i32, i32> = BTreeMap::new();
    endpoint1_relation.insert(-1, 300);
    endpoint1_relation.insert(0, 100);
    let endpoints = vec![Endpoint::new(0, endpoint0_relation), Endpoint::new(1, endpoint1_relation)];
    let requests = vec![Request::new(0, 0, 1000), Request::new(0, 1, 1500)];
    let gain = descent_gain(GainMode::PureGain, &cache_info, &videos, endpoints, requests);

    assert_eq!(gain.get(&300000).unwrap()[0], (0, 0));
}