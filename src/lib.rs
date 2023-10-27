fn get_repeated_part(n: u32, suffix: &str) -> String {
    if n == 0 {
        return format!("no more bottles {suffix}");
    }
    let first_s = if n > 1 { "s" } else { "" };
    format!("{n} bottle{first_s} {suffix}")
}

pub fn verse(n: u32) -> String {
    // todo!("emit verse {n}")
    if n == 0 {
        return format!(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
"
        );
    }
    // let first_s = if n > 1 { "s" } else { "" };
    // let repeated_part = format!("{n} bottle{first_s} of beer");
    let suffix = "of beer";
    let repeated_part = get_repeated_part(n, suffix);
    let on_the_wall = "on the wall";
    let last_part = get_repeated_part(n - 1, suffix);
    let it_or_one = if n > 1 { "one" } else { "it" };
    format!(
        "{repeated_part} {on_the_wall}, {repeated_part}.
Take {it_or_one} down and pass it around, {last_part} on the wall.\n"
    )
}

fn sing_rec(start: u32, end: u32, acc: String) -> String {
    let new_acc = format!("{}\n{}", acc, verse(start));
    if start <= end {
        return new_acc;
    }
    sing_rec(start - 1, end, new_acc)
}

fn sing_rec_start(start: u32, end: u32) -> String {
    format!("{}\n", sing_rec(start, end, "".to_string()).trim())
}

pub fn sing(start: u32, end: u32) -> String {
    sing_rec_start(start, end)
    // verse(start)
    // todo!("sing verses {start} to {end}, inclusive")
}
