# Dictionnaire

## Description

Dictionnaire est une librairie contenant la plupart des mot francais. <br>
Elle met à disposition un service permettant d'avoir ces mots dans une liste
et de pouvoir piocher un mot de manière aléatoire

## Exemple

```rust

let dictionnaire_service = DictionnaireService::new();

dictionnaire_service.words; // Vec<String> -> all words
dictionnaire_service.random_word(); // String -> one word

```