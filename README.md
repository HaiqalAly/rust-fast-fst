### Personal experiment on FST

I've been playing around with the `fst` crate to handle sets of strings efficiently. Here are my quick takeaways:

### What I Learned
1.  **Order Matters**: The `MapBuilder` (previously `SetBuilder`) requires keys to be inserted in **lexicographic order**. I had to sort my vector first.
2.  **Fuzzy Search**: Using the `Levenshtein` automaton allows for powerful approximate string matching (e.g., finding "food" when searching for "foo" with an edit distance of 2).
3.  **Map vs Set**: I switched to using `Map` instead of `Set`, associating a value with each key (currently just `0`).

### Benchmarking & Performance
*   **Compilation**: Interesting finding I found is that `Map` and `MapBuilder` seem to make the program compile faster, though the difference is negligible for this small experiment.
*   **Speed**:
    *   **Build Time**: Creating the FST from **103,495 words** took approximately **53ms**.
    *   **Search Time**: Performing a Levenshtein search (distance 1) for "love" took around **335µs**.
*   **Storage**: The compression is significant.
    *   Original `dict.txt`: **977 KB**
    *   Compressed `dict.fst`: **279 KB**
    *   **Reduction**: The FST is ~29% of the original file size, achieving a **~71% reduction** in storage.
*   **Next Step**: Maybe explore memory mapping specifically for larger datasets or try using streaming more extensively.

