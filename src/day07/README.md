# --- Day 7: Camel Cards ---

A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

So ==>

Every hand is exactly one type. From strongest to weakest, they are:

Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456


If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.


==>

`2AAA3` and `A6663` have the same type: `Three of a kind`, then we compare their first card in each hand,
`A > 2`, so `A6663 > 2AAA3`

# Note

我最初没有审好题, 把它复杂化了:
- 我是先把一手牌排好序(`2AAA3 -> AAA32`, `A6663 -> 666A3`), 然后才进行下一步比较, 
    此时 type 相同, 然后 `A > 6`, so `2AAA3 > A6663`
- 并且我原以为同一种 type 之间也会有大小关系(`AAA21 > KKK87`)


