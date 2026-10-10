fn main() {
    let html = "<p>Árvíztűrő tükörfúrógép</p>";

    println!("len(): {}", html.len());
    println!("chars().count(): {}", html.chars().count());
    println!(
        "Ennyi extra byte van az ékezetes karakterek miatt: {}",
        html.len() - html.chars().count()
    );
    let mut tags = 0;
    tags += 2;
    println!("Tagek: {}", tags);
}
