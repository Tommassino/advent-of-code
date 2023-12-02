# Advent of code Year 2020 Day 22 solution
# Author = witzatom
# Date = December 2020


def run(puzzle_input):
    player1, player2 = parse_input(puzzle_input)
    print(f"Part One : {part1(player1.copy(), player2.copy())}")
    print(f"Part Two : {part2(player1.copy(), player2.copy())}")


def parse_input(puzzle_input):
    return tuple(
        [
            int(i)
            for i in player.split("\n")[1:]
        ]
        for player in puzzle_input.split("\n\n")
    )


def part1(player1, player2):
    while player1 and player2:
        card1 = player1.pop(0)
        card2 = player2.pop(0)
        if card1 > card2:
            player1.extend([card1, card2])
        elif card2 > card1:
            player2.extend([card2, card1])
        else:
            raise ValueError()
    score = sum(
        (idx + 1) * card
        for idx, card in enumerate(reversed(player1 + player2))
    )
    return score


def part2(player1, player2):
    _, cards = recursive_combat(player1, player2)
    score = sum(
        (idx + 1) * card
        for idx, card in enumerate(reversed(cards))
    )
    return score


def recursive_combat(player1, player2):
    seen_decks = set()
    while player1 and player2:
        deck_combo = (tuple(player1), tuple(player2))
        if deck_combo in seen_decks:
            return True, player1
        seen_decks.add(deck_combo)
        card1 = player1.pop(0)
        card2 = player2.pop(0)
        enough_cards1 = len(player1) >= card1
        enough_cards2 = len(player2) >= card2
        player1_winner = card1 > card2
        if enough_cards1 and enough_cards2:
            player1_winner, _ = recursive_combat(
                player1[:card1].copy(),
                player2[:card2].copy()
            )
        if player1_winner:
            player1.extend([card1, card2])
        else:
            player2.extend([card2, card1])
    if player1:
        return True, player1
    else:
        return False, player2
