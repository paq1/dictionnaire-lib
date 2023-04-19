use dictionnaire::services::dictionnaire_service::DictionnaireService;

#[test]
pub fn new_dictionnaire_service_should_load_some_words() {
    let expected = true;
    let dictionnaire_service = DictionnaireService::new();
    assert_eq!(dictionnaire_service.words.len() > 1, expected)
}

#[test]
pub fn random_word_should_return_word_higher_than_0() {

    let expected = true;

    let dictionnaire_service = DictionnaireService::new();
    let word = dictionnaire_service.random_word();

    assert_eq!(word.len() > 0, expected)
}