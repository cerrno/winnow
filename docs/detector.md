Detector
========

## Winnowing paper
[Link to paper][winnowing-paper]  
> For this application, positional information (document and line number) is
> stored with each selected fingerprint. The first step builds an index mapping
> fingerprints to locations for all documents, much like the inverted index
> built by search engines mapping words to positions in documents. In the
> second step, each document is fingerprinted a second time and the selected
> fingerprints are looked up in the index; this gives the list of all matching
> fingerprints for each document.  Now the list of matching fingerprints for a
> document d may contain fingerprints from many different documents d1, d2,
> . . .. In the next step, the list of matching fingerprints for each document
> d is sorted by document and the matches for each pair of documents (d, d1
> ), (d, d2 ), . . . is formed. Matches between documents are rank-ordered by
> size (number of fingerprints) and the largest matches are reported to the
> user. Note that up until this last step, no explicit consideration of pairs
> of documents is required. This is very important, as we could not hope to
> carry out copy detection by comparing each pair of documents in a large
> corpus. By postponing the quadratic computation to the last step, we can
> optimize it by never materializing the matching for a pair of documents if it
> falls below some user-specified threshold.

## Our Detection
- Preprocessing
> "It does this by preprocessing the source code files, calculating a numeric
> fingerprint for each file, and then performing a longest common sequence
> search on the two fingerprints. The preprocessing stage replaces all function
> and variable names with a single token, and removes all comments and
> whitespace from the source code. The fingerprint stage calculates hash values
> for windows of characters in the resulting file, preserving the lowest hash
> values as the file’s fingerprint" [Engels et al. 2007][engels-paper]
1. Remove comments and whitespace
2. Tokenize and replace function names, syntax decorators, and variable names

- Positional information (file/line number/commit) stored with each fingerprint
1. First step builds map from fingerprints => locations for all documents
(inverted index)
2. Each document is fingerprinted a second time (**why?**) and the selected fingerprints
are looked up in the index
    - now have a list of all matching fingerprints for each document
    - list of matching fingerprints for a document d may contain fingerprints
      from many other docs d1, d2, ...
3. List of matching fingerprints for each doc d is sorted by document and the
matches for each pair of docs (d, d1), (d, d2), ... is formed. Matched b/w docs
are rank-ordered by size (number of fingerprints) (largest matched reported to
user)
    - Up until step (3), no condsideration of pairs of docs is required
      (O(N^2)).
    - By postponing the quadratic computation to the last step, we can
      optimize it by never materializing the matching for a pair of documents if it
      falls below some user-specified threshold.

## Questions
1. Why fingerprint a second time in step (2)?
2. How to define our 'global location' (file/line-number/commit?)
3. How to select winnowing window size/ngram size? (I think it discusses in the paper)
4. Why to tokenize/replace function names etc. for preprocessing?

[engels-paper]: TODO
[winnowing-paper]: https://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf
