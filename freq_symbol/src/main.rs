use std::collections::HashMap;

fn main() {
    let text = "hello, world!";

    let mut freq: HashMap<char, usize> = HashMap::new();

    for c in text.chars() {
        // пропускаем пробелы (если нужно)
/*        if c.is_whitespace() {
            continue;
        }
*/
        // увеличиваем счётчик для символа
        *freq.entry(c).or_insert(0) += 1;
    }

    // выводим результат
    for (ch, count) in &freq {
        println!("'{}': {}", ch, count);
    }

    println!("'{:?}'", freq);
}
