#!/usr/bin/env python3

import argparse
import re
import pathlib

import requests


def main(args):
    wordle_url = "https://www.powerlanguage.co.uk/wordle/"
    res = requests.get(wordle_url)
    res.raise_for_status()
    html = res.text

    matches = list(re.finditer(r'src="(?P<rel>main.[0-9a-f]+.js)"', html))
    script_url = wordle_url + matches[-1].group("rel")
    res = requests.get(script_url)
    res.raise_for_status()
    js_text = res.text

    word_lists = sorted(
        (
            [string_literal[1:-1] for string_literal in match.group(0)[1:-1].split(",")]
            for match in re.finditer(r'\["[a-z]{5}"(,"[a-z]{5}")*\]', js_text)
        ),
        key=len,
    )
    assert len(word_lists) == 2

    possible_secrets, allowed_guesses = word_lists

    with open("wordle_allowed_guesses.txt", "w") as f:
        f.writelines(line + "\n" for line in allowed_guesses)

    with open("wordle_possible_secrets.txt", "w") as f:
        f.writelines(line + "\n" for line in possible_secrets)


def arg_main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--pdb",
        action="store_true",
        help="Start a pdb post mortem on uncaught exception",
    )

    args = parser.parse_args()

    try:
        main(args)
    except Exception:
        if args.pdb:
            import pdb, traceback

            traceback.print_exc()
            pdb.post_mortem()
        raise


if __name__ == "__main__":
    arg_main()
