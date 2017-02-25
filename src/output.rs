use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn produce_output(result: BTreeMap<i32, BTreeSet<i32>>) -> String {
    let all: Vec<CacheAndVideos> = result.iter()
        .filter(|&(_, video_ids)| !video_ids.is_empty())
        .map(|(&cache_id, ref videos_id)| CacheAndVideos::new(cache_id, videos_id.iter().cloned().collect()))
        .collect();
    let returned: String = format!("{}\n", all.len());
    all.iter().fold(returned, |result, cache_and_video| {
        let video_id_str: Vec<String> = cache_and_video.video_ids.iter()
            .map(|video_id| format!("{}", video_id))
            .collect();
        let joined = video_id_str.join(" ");
        result + &format!("{} {}\n", cache_and_video.cache_id, joined)
    })
}

struct CacheAndVideos {
    cache_id: i32,
    video_ids: Vec<i32>
}

impl CacheAndVideos {
    fn new(cache_id: i32, video_ids: Vec<i32>) -> CacheAndVideos {
        CacheAndVideos {
            cache_id: cache_id,
            video_ids: video_ids
        }
    }
}