# Aboout
This project uses spacy and rust to get definitions for words from duden dls file

# Why make this project
I wanted a way to get just mono-lingual-definitions and nothing else.

# Why use Spacy
Because a lemma is more likely to contain a definition in the dictionary

# How to use this project
1. Clone the repo
2. The duden file is a compressed dsl file so you need to extract it using the following command ``dictzip -d deu-deu_duden_universal_as_3_0.dsl.dz``
3. Then put the file in ```deu-deu_duden_universal_as_3_0.dsl``` inside the root folder of the repo
4. and run the following command ```cargo run WORD_YOU_WANT```
