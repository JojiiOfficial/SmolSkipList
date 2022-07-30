pub mod deser;
pub mod link;
pub mod skip_list;
pub mod skip_map;
pub mod utils;

use crate::skip_map::SkipMap;

fn main() {
    let mut terms: Vec<_> = [
        "this", "is", "some", "text", "which", "gets", "indexed", "with", "an", "big", "brain",
        "algo",
    ]
    .into_iter()
    .enumerate()
    .map(|i| (i.1.to_string(), i.0))
    .collect();

    terms.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    println!("{terms:?}");

    let slist = SkipMap::from_sorted_iter(terms.clone());
    for i in &terms {
        let res = slist.find(&i.0.to_string());
        println!("{:?}", res);
    }
}
