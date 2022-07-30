pub mod deser;
pub mod link;
pub mod skip_list;
pub mod skip_map;
pub mod utils;

use skip_list::SkipList;

fn main() {
    let mut terms: Vec<_> = [
        "this", "is", "some", "text", "which", "gets", "indexed", "with", "an", "big", "brain",
        "algo",
    ]
    .into_iter()
    .map(|i| i.to_string())
    .collect();
    terms.sort_unstable();

    let slist = SkipList::from_sorted_iter(terms);
}
