not_in_book =
    { $absentWordsCount ->
        [one] { $words } is not in { $book }
        *[other] { $words } are not in { $book }
    }.

in_book =
    { $wordsCount ->
        [one] This word is in { $book }
        *[other] All these words are in { $book }
    }.

nothing_in_book =
    { $wordsCount ->
        [one] This word is not in { $book }
        *[other] None of these words are in { $book }
    }.

and = and