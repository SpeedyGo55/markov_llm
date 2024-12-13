# Markov LLM

## Introduction

This is a simple implementation of a Markov Chain in Rust. It is a simple implementation of a Markov Chain that can be
used to generate text based on a given input text. It is more a proof of concept.

## Usage

currently the program is not very user friendly. At the moment the chain is trained on the text in the `data.txt` file
every time the program is run. The Input text is hardcoded in the `main.rs` file. To run the program simply run
`cargo run` in the root directory of the project. To change the input text you have to change the `INPUT_TEXT` constant
in the `main.rs` file.
To change the training text you have to change the `data.txt` file.

## my speedys_markov_chain Crate Documentation

### `MarkovChain::new() -> MarkovChain`

This function creates a new MarkovChain object.

### `MarkovChain::train(&mut self, data: Vec<String>, state_size: usize)`

This function trains the MarkovChain object on the given data. The data is a vector of strings where each string is a
sentence.

### `MarkovChain::generate(&self, length: usize) -> String`

This function generates a new string of the given length based on the training data. It uses a random starting point in
the training data.

### `MarkovChain::complete(&mut self, sentence: String, min_len: usize) -> String`

This function completes the given sentence to the given minimum length. It uses the last words of the sentence as a
starting point.

### `MarkovChain::save(&self, path: &str)`

This function saves the MarkovChain object to a file at the given path.

### `MarkovChain::load(path: &str) -> MarkovChain`

This function loads a MarkovChain object from a file at the given path.

## Example

Look at the `main.rs` file for an example of how to use the MarkovChain object.