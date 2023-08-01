#!/usr/bin/env python3

import argparse
import sys

from cartist import Artist
from cartist.cli import CLIArtist
from cartist.rules import DEFAULT_RULE_SET
from cartist.rules import get_ruleset_from_csv


def parse_args(args):
    """Parses input arguments."""

    parser = argparse.ArgumentParser(description='Draws amazing cellular automata.')
    parser.add_argument(
        '-r',
        '--rule-set',
        metavar='FILE',
        nargs=1,
        dest='ruleset',
        help='CSV file with rule set.',
    )
    parser.add_argument(
        metavar='NUMBER',
        dest='number',
        type=int,
        nargs='?',
        default=25,
        help='Number to be squared (default: %(default)s)',
    )

    run_type = parser.add_mutually_exclusive_group()
    run_type.add_argument(
        '-g', '--gui', action='store_true', dest='gui', help='Run GUI version.'
    )
    run_type.add_argument(
        '-s',
        '--sqrt-only',
        action='store_true',
        dest='sqrt_only',
        help='Prints only sqrt of the given number (computed with the given rule set)',
    )

    return parser.parse_args(args)


def main(args=None):
    args = parse_args(args if args else sys.argv[1:])
    ruleset = DEFAULT_RULE_SET
    try:
        if args.ruleset:
            ruleset = get_ruleset_from_csv(args.ruleset[0])
    except Exception as ex:
        print(ex)
        sys.exit(1)

    if args.sqrt_only:
        artist = Artist(args.number, ruleset)
    elif args.gui:
        try:
            from cartist.gui import GUIArtist
            artist = GUIArtist(args.number, ruleset)
        except ImportError:
            print('tkinter not installed, falling back to CLI...\n')
            artist = CLIArtist(args.number, ruleset)
    else:
        artist = CLIArtist(args.number, ruleset)

    artist.paint()


if __name__ == '__main__':
    main()
