use averaged_collection::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection::new();

    collection.push(7);
    collection.push(2);
    collection.push(10);

    println!("Average {}", collection.average());
}
