use std::env;

struct Receipt{
    item:String,
    cost:f32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}",args);
    if args.len() < 2{
        println!("Usage receipt-generator 'item1-cost,item2-cost' ");
    }

    let information = &args[1];
    // println!("{}",information);
    let items:Vec<&str> = information.split(',').collect();
    // println!("{:?}",items);

    let mut receipts:Vec<Receipt> = Vec::new();

    for items in items{
        let things:Vec<&str> = items.split('-').collect();
        let purchased = things[0];
        let cost = things[1].parse::<f32>().unwrap();
        let receipt = Receipt {
            item: purchased.to_string(),
            cost,
        };
        receipts.push(receipt);
    }
    let total_cost = receipts.iter().map(|receipt| receipt.cost).sum::<f32>();
    display_table(&receipts, total_cost);
    
}

fn display_table(receipts: &Vec<Receipt>, total: f32) {
    println!("{:>15}","-----------");
    println!("{:>15}","| Receipt |");
    println!("{:>15}","-----------");
    println!("----------------------");
    println!("| {:<10} | {:<5} |","Item","Cost");
    println!("----------------------");
    for receipt in receipts{
        println!("| {:<10} | {:<5} |",receipt.item,receipt.cost);
    }
    println!("----------------------");
    println!("| {:<10} | {:<5} |","TOTAL",total);
    println!("----------------------");
}