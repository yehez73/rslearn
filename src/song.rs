// Task 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me", days[i]);
        for j in (0..i+1).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
}