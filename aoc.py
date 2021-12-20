# Advent of code project template creation tool
# Author = Tomas Witzany
# Date = 18/12/2020

import os
import importlib
import fire
from typing import Optional
import doctest
from init import InitApp, Config


class AoCRunner:
    def __init__(self) -> None:
        with open("user_session_id", "r") as file:
            user_session_id = file.read()
            self.config = Config(
                download_statements=False, # downloads the description of the problem
                download_input=True, # downloads the problem input.txt
                make_template=True, # creates code.py in the days directory
                overwrite=False, # whether to overwrite existing files
                author="witzatom", # author handle to put in the template
                default_year=2021, # the default year for aoc commands
                date="December 2021", # date to put in the template
                download_attempts=2, # number of attempts per test
                
                # under here mostly things you will not want to change
                base_folder=os.path.dirname(os.path.realpath(__file__)),
                link="https://adventofcode.com/",
                user_session_id=user_session_id,
                user_agent="adventofcode_working_directories_creator"
            )

    def init(self, day: int, year: Optional[int] = None) -> None:
        if year is None:
            year = self.config.default_year
        
        InitApp(self.config).initialize(year, day)

    def run(self, day: int, year: Optional[int] = None, input: Optional[str] = None) -> None:
        if year is None:
            year = self.config.default_year

        day_folder = os.path.join(os.path.dirname(__file__), f"y{year}", str(day))
        if input is not None:
            input = os.path.join(day_folder, input)
        else:
            input = os.path.join(day_folder, "input.txt")
        
        with open(input, 'r') as input_file:
            puzzle_input = input_file.read()

        module = self._get_module(day, year)
        module.run(puzzle_input)

    def doctest(self, day: int, year: Optional[int] = None) -> None:
        if year is None:
            year = self.config.default_year
        
        module = self._get_module(day, year)
        doctest.testmod(module, verbose=True)

    def _get_module(self, day: int, year: int):
        return importlib.import_module(f"y{year}.{day}.code")


if __name__ == "__main__":
    fire.Fire(AoCRunner)
