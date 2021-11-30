import sys
import os
import importlib.util

if __name__ == "__main__":
    day = sys.argv[1]
    day_folder = os.path.join(os.path.dirname(__file__), day)

    if len(sys.argv) > 2:
        input_file = os.path.join(day_folder, sys.argv[2])
    else:
        input_file = os.path.join(day_folder, "input.txt")
    code_file = os.path.join(day_folder, "code.py")

    with open(input_file, 'r') as input_file:
        puzzle_input = input_file.read()

    spec = importlib.util.spec_from_file_location("day", code_file)
    day_code = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(day_code)
    day_code.run(puzzle_input)


