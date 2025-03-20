fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let original = "exemplo";
    let invertida = reverse(original);
    println!("Original: {}", original);
    println!("Invertida: {}", invertida);
}



