fn main() {
    pasta_macro::build! {
        "dic/main.pasta"
        "dic/main2.pasta"
        "dic/sakuhin.csv"
        "dic/words.csv"
    };
}
