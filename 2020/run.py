import sys
import os
import importlib.util

if __name__ == "__main__":
    day = sys.argv[1]
    day_folder = os.path.join(os.path.dirname(__file__), day)
    input_file = os.path.join(day_folder, "input.txt")
    code_file = os.path.join(day_folder, "code.py")

    with open(input_file, 'r') as input_file:
        input = input_file.read()

    spec = importlib.util.spec_from_file_location("day", code_file)
    day_code = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(day_code)
    day_code.run(input)


