# Advent of code Year 2020 Day 4 solution
# Author = witzatom
# Date = December 2020
import re


def run(puzzle_input: str):
    passports = [
        {
            field.split(":")[0]: field.split(":")[1]
            for field in re.split(r"\s", passport)
        }
        for passport in puzzle_input.split("\n\n")
    ]

    def range_validator(min_number, max_number):
        def inner(field):
            if not re.match(r"^\d+$", field):
                return False
            value = int(field)
            return min_number <= value <= max_number

        return inner

    def height_validator():
        pattern = re.compile(r"^(\d+)(cm|in)$")

        def inner(field):
            match = pattern.match(field)
            if not match:
                return False
            value, unit = match.groups()
            if unit == "cm":
                return 150 <= int(value) <= 193
            else:
                return 59 <= int(value) <= 76

        return inner

    def regex_validator(regex):
        pattern = re.compile(regex)

        def inner(field):
            return pattern.match(field) is not None

        return inner

    field_validation = {
        "byr": range_validator(1920, 2002),
        "iyr": range_validator(2010, 2020),
        "eyr": range_validator(2020, 2030),
        "hgt": height_validator(),
        "hcl": regex_validator(r"^#[0-9a-f]{6}$"),
        "ecl": regex_validator(r"^(amb|blu|brn|gry|grn|hzl|oth)$"),
        "pid": regex_validator(r"^\d{9}$"),
        "cid": None
    }

    def is_valid(passport):
        missing_fields = set(field_validation.keys()).difference(set(passport.keys()))
        return len(missing_fields) == 0 or missing_fields == {"cid"}

    valid_passports = [
        passport for passport in passports
        if is_valid(passport)
    ]

    def is_valid_strict(passport):
        if not is_valid(passport):
            return False
        for field, validation in field_validation.items():
            if validation is None:
                continue
            if not validation(passport[field]):
                return False
        return True

    strictly_valid_passports = [
        passport for passport in passports
        if is_valid_strict(passport)
    ]

    print(f"Part One : {len(valid_passports)}")
    print(f"Part Two : {len(strictly_valid_passports)}")
