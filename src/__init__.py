import spacy
import sys


load_model = spacy.load('de_core_news_sm', disable=['parser', 'ner'])

word = sys.argv[1]
doc = load_model(word)

for token in doc:
    print(token.lemma_)
