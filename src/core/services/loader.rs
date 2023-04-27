pub trait LoaderWords<Words> {
    fn load_words(&self) -> Words;
}