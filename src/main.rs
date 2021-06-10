fn main() {
    let mut day = String::new();
    for num in 1..13 {
	match num {
	    1 => day = "first".to_string(),
	    2 => day = "second".to_string(),
	    3 => day = "third".to_string(),
	    4 => day = "fourth".to_string(),
	    5 => day = "fifth".to_string(),
	    6 => day = "sixth".to_string(),
	    7 => day = "seventh".to_string(),
	    8 => day = "eighth".to_string(),
	    9 => day = "nineth".to_string(),
	    10 => day = "tenth".to_string(),
	    11 => day = "eleventh".to_string(),
	    12 => day = "twelfth".to_string(),
	    _ => println!(" Holi! "),
	}
	println!("On the {} day of Christmas\nMy true love gave to me", day);
	if num >= 12 { println!("Twelve drummers drumming"); }
	if num >= 11 { println!("Eleven pipers piping"); }
	if num >= 10 { println!("Ten lords-a-leaping"); }
	if num >= 9 { println!("Nine ladies dancing"); }
	if num >= 8 { println!("Eight maids-a-milking"); }
	if num >= 7 { println!("Seven swans-a-swimming"); }
	if num >= 6 { println!("Six geese-a-laying"); }
	if num >= 5 { println!("Five golden rings"); }
	if num >= 4 { println!("Four calling birds"); }
	if num >= 3 { println!("Three French hens"); }
	if num >= 2 { println!("Two turtle doves"); }
	println!("A partridge in a pear tree\n");
    }
}
