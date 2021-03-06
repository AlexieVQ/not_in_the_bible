# Examples

## Sources

This directory contains two examples of sources to use with the bot.

These two examples are text files containing a list of words extracted from
their original source by using the [`tokenize`](../src/bin/tokenize.rs)
executable.

[`bible_en.txt`](bible_en.txt) is a list of words extracted from the
*King James Bible*
([Wikisource](https://en.wikisource.org/wiki/Bible_(King_James))), which is in
the public domain outside the United Kingdom.

[`bible_fr.txt`](bible_fr.txt) is a list of words extracted from the French
translation of the Bible made by Louis-Isaac Lemaistre de Sacy between 1667 and
1696 ([Wikisource](https://fr.wikisource.org/wiki/Bible_Sacy)), which is in the
public domain.

## Exclusion lists

This directory also contains two files listing function and common words in
[English](ignored_en.txt) and [French](ignored_fr.txt). They are used in the
`excluded` parameter of the [config file](../config.yaml.example) to provide
words to ignore when searching in the sources.
