use common_lib::log::debug;

use crate::types::{DELETE, GET, POST, RQM};

/// returns (get / post / delete)
/// O(n)
pub fn sort_requests(
    requests: &Vec<RQM>
) -> (Vec<RQM>, Vec<RQM>, Vec<RQM>) {
    
    let mut gets = vec![];
    let mut posts = vec![];
    let mut deletes = vec![];
    for quest in requests {
        match quest.header.as_str() {
            GET => {
                gets.push(quest.clone());
            },
            POST => {
                posts.push(quest.clone());
            },
            DELETE => {
                deletes.push(quest.clone());
            },
            _ => {debug!("an unkown header found while sorting")}
        }
    }
    return (gets, posts, deletes);
}


/// takes a ref to the admin requests and header that you want to be sorted 
/// and return all the request with the matching header 
/// if you want all the requests but saperated fn sort sort_requests is a better match
/// O(n)
pub fn sort_requests_with_header(
    requests: &Vec<RQM>,
    header: &String,
) -> Vec<RQM> {
    let mut sorted = vec![];
    for quest in requests {
        if quest.header.as_str() == header {
                sorted.push(quest.clone());
        }
    }
    return sorted;
}



/// is the same as sort_requests_with_header and 
/// returns the size of all (header)requests combined
/// O(n)
pub fn sort_and_size_requests_with_header(
    requests: &Vec<RQM>,
    header: &String,
) -> (u64, Vec<RQM>) {
    let mut sorted = vec![];
    let mut size = 0i64;
    for quest in requests {
        if quest.header.as_str() == header {
            sorted.push(quest.clone());
            size+=quest.size;
        }
    }
    return (size as u64, sorted);
}



