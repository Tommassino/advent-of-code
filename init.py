# Advent of code project template creation tool
# Author = Tomas Witzany
# Date = 18/12/2020

import os
import requests
from typing import NamedTuple

class Config(NamedTuple):
    download_statements: bool
    download_input: bool
    make_template: bool
    author: str
    overwrite: bool
    date: str
    base_folder: str
    download_attempts: int
    link: str
    user_session_id: str
    user_agent: str
    default_year: int


class InitApp:
    def __init__(self, config: Config) -> None:
        self.config = config

    def initialize(self, year, day):
        day_folder = os.path.join(self.config.base_folder, f"y{year}", str(day))
        if not os.path.exists(day_folder):
            os.makedirs(day_folder)
        print(f"Creating template for day {year}/{day} in {day_folder}")
        self.generate_template(year, day, day_folder)
        self.download_inputs(year, day, day_folder)
        self.download_statements(year, day, day_folder)
        print("Complete")

    def generate_template(self, year, day, day_folder):
        if not self.config.make_template:
            return
        code_path = os.path.join(day_folder, "code.py")
        if os.path.exists(code_path) and not self.config.overwrite:
            print(f"Skipping code template for {year}/{day} as they exist")
            return
        with open(os.path.join(self.config.base_folder, "template.py"), "r") as input:
            template = input.read()
        template = template.replace("{year}", str(year))
        template = template.replace("{day}", str(day))
        template = template.replace("{author}", self.config.author)
        template = template.replace("{date}", self.config.date)
        with open(code_path, "w+") as output:
            output.write(template)
    
    def download_inputs(self, year, day, day_folder):
        if not self.config.download_input:
            return
        output_path = os.path.join(day_folder, "input.txt")
        if os.path.exists(output_path) and not self.config.overwrite:
            print(f"Skipping inputs for {year}/{day} as they exist")
            return
        errors = []
        try:
            input = self._request_attempts(f"{self.config.link}{year}/day/{day}/input", self.config.download_attempts)
            with open(output_path, "w+") as out:
                out.write(input.rstrip("\n"))
            return
        except Exception as e:
            errors.append(e)
        try:
            input = self._request_attempts(f"{self.config.link}{year}/day/{day}/input.txt", self.config.download_attempts)
            with open(output_path, "w+") as out:
                out.write(input.rstrip("\n"))
            return
        except Exception as e:
            errors.append(e)
        print(f"Could not download inputs: {errors}")

    def download_statements(self, year, day, day_folder):
        if not self.config.download_statements:
            return
        output_path = os.path.join(day_folder, "statement.html")
        if os.path.exists(output_path) and not self.config.overwrite:
            print(f"Skipping statements for {year}/{day} as they exist")
            return
        url = f"{self.config.link}{year}/day/{day}"
        try:
            html = self._request_attempts(url, self.config.download_attempts)
            start = html.find("<article")
            end = html.rfind("</article>")+len("</article>")
            end_success = html.rfind("</code>")+len("</code>")
            with open(output_path, "w+") as file:
                file.write(html[start:max(end, end_success)])
        except Exception as e:
            print(f"Could not download statements: {e}")

    def _request_attempts(self, url, attempts):
        last_error = None
        for i in range(attempts):
            try:
                with requests.get(url=url, cookies={"session": self.config.user_session_id}, headers={"User-Agent": self.config.user_agent}) as response:
                    if response.ok:
                        return response.text
                    else:
                        last_error = RuntimeError(f"Url {url}: {response.reason}")
            except requests.exceptions.RequestException as e:
                last_error = e
            except Exception as e:
                last_error = e
        raise last_error
