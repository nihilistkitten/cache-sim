use cache_sim::condition::Condition;
use std::collections::HashMap;
use std::fs::File;
use itertools::Itertools;

use cache_sim::{atf::parse, output::to_csv, GeneralModelItem, NoCondition, Trace, LastNItems};

fn main() -> anyhow::Result<()> {
    let trace = Trace::from(
        parse(include_bytes!("traces/systor17/testLUN.atf").as_slice())?
            .into_iter()
            .map(GeneralModelItem::from)
            .collect::<Vec<_>>(),
    );
    
	dbg!("parsed");
	let record_file = File::options().append(true).create(true).open("src/histograms/stack-distances.csv")?;
	dbg!("file open");
    //let stack_distances = trace.stack_distances();
    let stack_distances = Trace::from(vec![0,0]).stack_distances();
    dbg!("stack dists done");
	
    to_csv("testLUN", &[trace.average_entropy()], &stack_distances, record_file)?;

	
	/*
	// Output frequency histograms
    let file = File::create("src/histograms/testLUN-histograms.csv")?;
    let mut conditions: HashMap<String, Box<dyn Condition<GeneralModelItem>>> =
        HashMap::with_capacity(2);

    // TODO: is there a way to statically create a hashmap with type-erased values?
    conditions.insert(String::from("NoCondition"), Box::new(NoCondition));
    conditions.insert(
        String::from("EqualsPrevious"),
        Box::new(|t: &Trace<_>, i| i > 0 && t[i - 1] == t[i]),
    );
    
    for item in trace.iter().unique().copied().collect::<Vec<_>>(){
		let name = format!("After{}",item.to_string());
		conditions.insert(
        name,
        Box::new(LastNItems::new(vec![item])),
    );
	}

    trace.write_conditional_frequencies(conditions, || Ok(file.try_clone()?))?;
	*/
	
    Ok(())
}
