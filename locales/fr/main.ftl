not_in_book =
    { $absentWordsCount ->
        [one] { $words } n’est pas dans { $book }
        *[other] { $words } ne sont pas dans { $book }
    }.

in_book =
    { $wordsCount ->
        [one] Ce mot est dans { $book }
        *[other] Tous ces mots sont dans { $book }
    }.

nothing_in_book =
    { $wordsCount ->
        [one] Ce mot n’est pas dans { $book }
        *[other] Aucun de ces mots n’est dans { $book }
    }.

percent_in_book = { $percent }% de ces mots ne sont pas dans { $book }.

and = et