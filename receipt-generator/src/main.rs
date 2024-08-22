use std::env;

struct Receipt{
    item:String,
    cost:String,
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
        let cost = things[1];
        let receipt = Receipt {
            item: purchased.to_string(),
            cost: cost.to_string(),
        };
        receipts.push(receipt);
    }
    display_table(receipts);
}

fn display_table(receipts: Vec<Receipt>){
    println!("{:>15}","-----------");
    println!("{:>15}","| Receipt |");
    println!("{:>15}","-----------");
    println!("----------------------");
    println!("| {:<10} | {:<5} |","Item","Cost");
    println!("----------------------");
    for receipt in receipts{
        println!("| {:<10} | {:<5} |",receipt.item,receipt.cost);
    }
}