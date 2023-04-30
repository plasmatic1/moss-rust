Preprocessing will be done as a series of transformers, which are individual algorithms that input a sequence of tokens and outputs a sequence of postprocessed tokens.  Transformers can then be chained together to create more complex preprocessing chains

# Comments

Preprocessor will remove comments and map the range to the preceding token

# Punctuation

* We will remove no punctuation, since it's generally useful in code

# Removing language/variable fragments

* We will need to normalize variable/type/function names
    * In `algorithm.md`
* Observation: 