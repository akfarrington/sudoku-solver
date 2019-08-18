# sudoku-solver

An attempt to solve sudoku puzzles without force guessing.

Some of the code is horrible and ugly, especially the web stuff.

This sudoku solver can solve up to "hard" puzzles, and can even solve some "expert" ones. I've done my best to deal with the notes tricks, but kind of gave up at the obvious/hidden doubles and triples. My obvious and hidden double functions aren't perfect, and will miss them sometimes.

After spending loads of time on this, I thought it would just be better if I moved on to another thing. The things I'd like to fix if I continued to spend time on this would be to clean up the code, make the code more idiomatic, use more functional programming, and change the empty board so that it doesn't require filling empty cells with zeros.

**Note** that the html for the sudoku board is all from [this gist](https://gist.github.com/thebinarypenguin/4d45ffe87096e508800b5d11544bf2fa). I tried my hardest to find any kind of copyright info about gists, but couldn't find anything. So, with that said, I thought it would be okay if I used it and gave credit where credit is due. Thanks, thebinarypenguin. css isn't my strong suit.