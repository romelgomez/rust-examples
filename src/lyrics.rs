pub fn twelve_days_of_christmas() -> String {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut lyrics = String::new();

    for i in 0..12 {
        lyrics.push_str(&format!("On the {} day of Christmas,\n", days[i]));
        lyrics.push_str("My true love gave to me:\n");
        for j in (0..i + 1).rev() {
            if j == 0 {
                lyrics.push_str(&format!("And {}\n", gifts[j]));
            } else {
                lyrics.push_str(&format!("{},\n", gifts[j]));
            }
        }
        lyrics.push_str("\n");
    }

    lyrics
}
