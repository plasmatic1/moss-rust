# Paper

Paper: [here](http://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf)

## Legend

* Bold = definition

## Main stuff

Goal: Be able to detect partial copies/matches in code.

### Background

Previous algorithms focused on detecting $k$-grams (substrings of length $k$).  Idea:

* Collect all $k$-grams of the full string (after preprocessing)
    * Select some subset of each $k$-gram to hash and call it the **fingerprint**
        * i.e. We select only hashes that have value "0 mod p"
            * **Gap**: Distance between consecutive selected gaps
            * Issue: maximum gap can be arbitrarily large in practice, and matches smaller than the gap may not be detected
    
Document fingerprinting works quite well as $k$-grams as it makes no assumptions about underlying data, and it's easy to adapt fingerprinting for different algorithms.

1. Whitespace, capitalization, punctuation, etc. insensitivity.  Matches should be unaffected by things that don't matter
    * i.e. for software text matching should be variable-name insensitive
2. Noise suppression: very short matches (i.e. 'the') can be considered as noise as they're simply common idioms and don't indicate plagiarism
3. Position independence: example:
    * Course-grained permutation (i.e. reordering paragraphs) should not matches
    * Removing a single paragraph should not affect other matches

**Remarks:**
* For (1), we first do a pass to "normalize" the text, such as
    * removing punctuation and whitespace
    * converting letters to lowercase
    * changing all variable names to 'V'
* For (2), we have the flexibility to choose the best $k$ for plagiarism detection
    * TODO: fill in correct value from section 5

#### Notes on Past Algorithms

* String matching of $k$-grams: Rabin-karp rolling hash
    * Improvement: multiply everything by $b$ one more time at the end to make the added character affect all bits of the hash more uniformly
* Selecting all hashes would cause our output to be the size of a whole document... huge!!!
    * Is this really a problem nowadays?
    * What if we just skipped this portion?
    * Thus, we fingerprint a $k$-gram by only selecting some hashes
* Our algorithm for selection must be position-independent
    * Otherwise shifting the document could result in completely different results

Other misc. results
* Zipf's law
    * English words have frequency inveresly proportional to their rank (where rank is the position of the frequency if we sorted all the frequencies in decreasing order)

### Will Introduce

Goal of Paper: give a method for selecting fingerprints s.t. at least part of any sufficiently long match is detected
    * Detect $\ge 1$ $k$-gram in any shared substring of length at least $w+k-1$

* **Winnowing**: Efficient local fingerprinting algorithm (the paper will describe this)
* **Window**: A window of length $w$ ($w$ selected by the user) is a consecutive sequence of $w$ hashes of $k$-grams
* **Local**: An algorithm is local iff for any window of consecutive hashes, a hash is selected as part of a fingerprint.
* **Density**: The density of a fingerprinting algorithm 
    * Claim that winnowing achieves density of $2/(w+1)$, while the best density if $1.5/(w+1)$
    * Goal: minimize density while also maintaining local matches

# Algorithm Description (Winnowing)

First, the user choose constants $k, t$ s.t. $k \le t$:

1. Matches of size $<k$ are considered noise and thus should be ignored (noise threshold).  We do this by only considering $k$-grams
2. We will always detect matches of size at least $t$

We start by finding the hash of every $k$-gram: $h_1, \ldots$.  For any sequence of hashes $h_1, \ldots, h_n$ with length $n>t-k$ (AKA these hashes represent a substring of more than length $t$), then we must choose at least one of these hashes as part of the fingerprint.

This suggests us choosing a window size of $w=t-k+1$ (i.e. $w>t-k$).  For each window, select the minimum hash value within the window, breaking ties by rightmost hash.  These selected hashes are the fingerprint of the document.

**Rmk**: If the hash for a single window (both value and position) remain the same in the next window, we don't record the hash a second time.  Refer to diagram 2e.

**Rmk**: We often want to also save positional information about fingerprints (i.e. to recover the matching substrings).

---

Issue: algorith performs poorly on low-entropy strings (i.e. density is very high)

Fix: change the selection algorithm to this instead:
* In each window, select the minimal hash
* If there is a tie, prefer the hash selected by the previous window (i.e. 1 posn to the left)
* Else, select the rightmost minimal hash

This prevents very homogenous data from having huge hash counts

* The paper also mentions that this algorithm is no longer local, but still preserves our desired properties of guaranteeing to find matches of a certain length.

# Algorithm Description (Full)

First, we apply normalization on the code to remove any irrelevant features, such as

* TODO

This preprocessing step is completely separate from the fingerprinting step, and is meant to 'normalize' the input.

---

Then, for each document we hash its $k$-grams, and then use winnowing to compute a set of fingerprints for each document, and a mapping from fingerprint to global location (in the original string), so we can recover it.

We then insert all of these fingerprints into a dictionary of some sort for comparison.

---

We then go over each document again.  For a document $d$ we run the fingerprinting algorithm a second time, and we check the selected fingerprints in the dictionary, which gives us a list of all matching fingerprints by index.

We group these matches by the document the match came from, allowing us to sort the documents by most matched to least matched.

**Alternative Algorithmic Formulation**: For each document, we get the intersection of fingerprints with every other document.

Once we have a good , we can separate out documents which have 'sufficient numbers of matches' and 

## Thoughts on preprocessing

From paper:

> Along the same lines, early versions of M OSS incorporated a technique similar to Baker’s parameterized matches (Section 2). However, we found that replacing all of the parameters with a single constant and in- creasing k by 1 worked just as well. This appears to be a general trick: sophisticated efforts to exploit document semantics can of- ten be closely approximated by very simple exploits of document semantics together with a small increase in k.

Baker's parameterized matches refers to a method of matching strings where some strings are designated as 'parameters', and two strings are equal if there is a renaming of the parameters that causes them to match exactly.

This quote says that if we just rename all the names to the same constant and doing `k++;` worked just as well as a complex variable-name-insensitive-but-still-structure-sensitive algorithm.

---

Personal things:

* For each token in the preprocessed document, we must tie it to a 'global range' (range in the original text) that it matches to.  We can start this range at the actual first position of the token, and end it right before the start of the next token.
    * For example, "int abc;      \nint bcd;    \n" might be processed to "TV;TV;" (where T is type, V is var).  In which case, the correspondances are
        * T => "int "
        * V => "abc"
        * ; => ";      \n"
        * T => "int "
        * V => "bcd"
        * ; => "    \n"

More on this in `preprocessing.md`