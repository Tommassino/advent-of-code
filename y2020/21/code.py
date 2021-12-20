# Advent of code Year 2020 Day 21 solution
# Author = witzatom
# Date = December 2020
from functools import reduce


def run(puzzle_input):
    foods = parse_input(puzzle_input)
    print(f"Part One : {part1(foods)}")
    print(f"Part Two : {part2(foods)}")


def parse_input(puzzle_input):
    def parse_food(line):
        ingredients, contents = tuple(line.split(" (contains "))
        ingredients = ingredients.split()
        contents = contents[:-1].split(", ")
        return ingredients, contents

    return [
        parse_food(line)
        for line in puzzle_input.split("\n")
    ]


def part1(foods):
    possible_ingredients = {}
    all_ingredients = set()
    for ingredients, allergens in foods:
        all_ingredients.update(ingredients)
        for allergen in allergens:
            if allergen not in possible_ingredients:
                possible_ingredients[allergen] = set(ingredients)
            else:
                possible_ingredients[allergen] = possible_ingredients[allergen].intersection(ingredients)
    ingredients_with_allergens = reduce(lambda a, b: a.union(b), possible_ingredients.values())
    ingredients_without_allergens = all_ingredients.difference(ingredients_with_allergens)
    food_count = sum(
        len(ingredients_without_allergens.intersection(ingredients))
        for ingredients, _ in foods
    )
    return food_count


def part2(foods):
    possible_ingredients = {}
    for ingredients, allergens in foods:
        for allergen in allergens:
            if allergen not in possible_ingredients:
                possible_ingredients[allergen] = set(ingredients)
            else:
                possible_ingredients[allergen] = possible_ingredients[allergen].intersection(ingredients)
    food_assignments = {}
    while possible_ingredients:
        found_allergen, found_ingredient = next(
            (allergen, ingredients.pop())
            for allergen, ingredients in possible_ingredients.items()
            if len(ingredients) == 1
        )
        food_assignments[found_allergen] = found_ingredient
        for allergen in possible_ingredients:
            possible_ingredients[allergen] -= {found_ingredient}
        del possible_ingredients[found_allergen]
    result = ",".join(
        i
        for _, i in sorted(food_assignments.items(), key=lambda x: x[0])
    )
    return result
